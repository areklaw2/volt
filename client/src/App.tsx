import { useState, useCallback, useEffect } from 'react';
import { ThemeProvider } from '@/components/theme-provider';
import { AppLayout } from '@/components/app-layout';
import { MessageList } from '@/components/chat/MessageList';
import { MessageInput } from '@/components/chat/MessageInput';
import type { Message, Conversation } from '@/types';
import { MessageSquare, Info } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useUser, useAuth } from '@clerk/react-router';
import { initializeApi, createUser, fetchConversations, fetchMessages, markAsRead } from '@/services/api';
import { connectWebSocket, sendMessage, disconnectWebSocket } from '@/services/ws';

function App() {
  const { user } = useUser();
  const { getToken } = useAuth();

  const userId = user?.id || '';

  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [currentConversationId, setCurrentConversationId] = useState<string | null>(null);
  const [messagesByConversation, setMessagesByConversation] = useState<Record<string, Message[]>>({});

  const currentConversation = conversations.find((c) => c.id === currentConversationId) ?? null;
  const messages = currentConversationId ? (messagesByConversation[currentConversationId] ?? []) : [];

  const unreadCounts: Record<string, number> = {};
  for (const conv of conversations) {
    const lastReadStr = conv.participants.find((p) => p.user_id === userId)?.last_read_at;
    const msgs = messagesByConversation[conv.id] ?? [];
    if (lastReadStr) {
      const lastRead = new Date(lastReadStr).getTime();
      unreadCounts[conv.id] = msgs.filter(
        (m) => m.sender_id !== userId && new Date(m.created_at).getTime() > lastRead,
      ).length;
    } else {
      unreadCounts[conv.id] = msgs.filter((m) => m.sender_id !== userId).length;
    }
  }

  initializeApi(getToken);

  useEffect(() => {
    if (!user) {
      return;
    }

    const username = user.username || user.primaryEmailAddress?.emailAddress || user.id;
    const displayName = user.fullName || username;
    createUser(user.id, username, displayName).catch(() => {});
  }, [user]);

  useEffect(() => {
    fetchConversations(userId)
      .then((convs) => {
        setConversations(convs);
        Promise.all(
          convs.map((c) =>
            fetchMessages(c.id).then((msgs) => [c.id, msgs] as const),
          ),
        )
          .then((results) => {
            setMessagesByConversation((prev) => {
              const next = { ...prev };
              for (const [id, msgs] of results) {
                next[id] = msgs;
              }
              return next;
            });
          })
          .catch(() => {});
      })
      .catch(() => setConversations([]));
  }, [userId]);

  useEffect(() => {
    if (!userId) return;
    connectWebSocket(userId, (msg) => {
      setMessagesByConversation((prev) => ({
        ...prev,
        [msg.conversation_id]: [...(prev[msg.conversation_id] ?? []), msg],
      }));
    });
    return () => disconnectWebSocket();
  }, [userId]);

  useEffect(() => {
    if (!currentConversationId) {
      return;
    }
    fetchMessages(currentConversationId)
      .then((msgs) => {
        setMessagesByConversation((prev) => ({ ...prev, [currentConversationId]: msgs }));
      })
      .catch(() => {});

    if (userId) {
      const now = new Date().toISOString();
      markAsRead(currentConversationId, userId)
        .then(() => {
          setConversations((prev) =>
            prev.map((c) =>
              c.id === currentConversationId
                ? {
                    ...c,
                    participants: c.participants.map((p) => (p.user_id === userId ? { ...p, last_read_at: now } : p)),
                  }
                : c,
            ),
          );
        })
        .catch(() => {});
    }
  }, [currentConversationId, userId]);

  const handleSend = useCallback(
    (content: string) => {
      if (!currentConversationId || !userId) {
        return;
      }
      sendMessage(currentConversationId, userId, content);
    },
    [currentConversationId, userId],
  );

  const handleCreateConversation = useCallback((conversation: Conversation) => {
    setConversations((prev) => [conversation, ...prev]);
    setCurrentConversationId(conversation.id);
  }, []);

  function getConversationName(conversation: Conversation): string {
    if (conversation.name) {
      return conversation.name;
    }

    const others = conversation.participants.filter((p) => p.user_id !== userId);
    if (others.length === 0) {
      return 'Unknown';
    }
    if (others.length === 1) {
      return others[0].display_name ?? 'Unknown';
    }
    return others.map((p) => p.display_name ?? 'Unknown').join(', ');
  }

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <AppLayout
        activeConversationId={currentConversationId}
        onSelectConversation={setCurrentConversationId}
        conversations={conversations}
        currentUserId={userId}
        onCreateConversation={handleCreateConversation}
        unreadCounts={unreadCounts}
      >
        {currentConversation ? (
          <div className="flex h-dvh flex-col">
            <header className="flex items-center justify-between border-b px-4 py-3">
              <div>
                <h1 className="text-lg font-semibold">{getConversationName(currentConversation)}</h1>
                {currentConversation.conversation_type === 'group' ? (
                  <span className="text-xs text-muted-foreground">
                    {currentConversation.participants.length} members
                  </span>
                ) : (
                  <span className="text-xs font-medium text-primary">Online</span>
                )}
              </div>
              <Button variant="ghost" size="icon" className="text-muted-foreground">
                <Info className="h-5 w-5" />
              </Button>
            </header>
            <MessageList
              messages={messages}
              currentUserId={userId}
              participants={currentConversation.participants}
              isGroup={currentConversation.conversation_type === 'group'}
            />
            <MessageInput onSend={handleSend} />
          </div>
        ) : (
          <div className="flex h-dvh items-center justify-center">
            <div className="flex flex-col items-center gap-3 text-muted-foreground">
              <MessageSquare className="h-12 w-12" />
              <p className="text-lg">Select a conversation to start chatting</p>
            </div>
          </div>
        )}
      </AppLayout>
    </ThemeProvider>
  );
}

export default App;
