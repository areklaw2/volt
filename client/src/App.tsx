import { useState, useCallback, useEffect, useRef } from 'react';
import { ThemeProvider } from '@/components/theme-provider';
import { AppLayout } from '@/components/app-layout';
import { MessageList } from '@/components/chat/MessageList';
import { MessageInput } from '@/components/chat/MessageInput';
import Onboarding from '@/components/onboarding';
import type { Message, Conversation } from '@/types';
import { MessageSquare, Info } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useLocalIdentity } from '@/hooks/use-local-identity';
import { fetchConversations, fetchMessages, markAsRead, editMessage } from '@/services/api';
import { connectWebSocket, sendMessage, disconnectWebSocket } from '@/services/ws';

function App() {
  const { identity, setIdentity, clearIdentity } = useLocalIdentity();

  if (!identity) {
    return (
      <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
        <Onboarding onComplete={setIdentity} />
      </ThemeProvider>
    );
  }

  return <Chat userId={identity.id} displayName={identity.displayName} onSignOut={clearIdentity} />;
}

function markConversationReadLocally(conversations: Conversation[], conversationId: string, userId: string, readAt: string): Conversation[] {
  return conversations.map((c) =>
    c.id === conversationId
      ? { ...c, participants: c.participants.map((p) => (p.user_id === userId ? { ...p, last_read_at: readAt } : p)) }
      : c,
  );
}

function Chat({ userId, displayName, onSignOut }: { userId: string; displayName: string; onSignOut: () => void }) {
  const [conversations, setConversations] = useState<Conversation[]>([]);
  const [currentConversationId, setCurrentConversationId] = useState<string | null>(null);
  const [messagesByConversation, setMessagesByConversation] = useState<Record<string, Message[]>>({});

  const currentConversationIdRef = useRef<string | null>(null);
  useEffect(() => {
    currentConversationIdRef.current = currentConversationId;
  }, [currentConversationId]);

  const currentConversation = conversations.find((c) => c.id === currentConversationId) ?? null;
  const messages = currentConversationId ? (messagesByConversation[currentConversationId] ?? []) : [];

  const unreadCounts: Record<string, number> = {};
  for (const conv of conversations) {
    if (conv.id === currentConversationId) {
      // actively viewing it — never show it as unread regardless of timing races
      unreadCounts[conv.id] = 0;
      continue;
    }
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
    connectWebSocket(userId, {
      onMessage: (msg) => {
        setMessagesByConversation((prev) => ({
          ...prev,
          [msg.conversation_id]: [...(prev[msg.conversation_id] ?? []), msg],
        }));

        // still looking at this conversation when the message landed — keep the
        // server's read pointer honest so a refresh mid-chat doesn't show it as unread
        if (msg.conversation_id === currentConversationIdRef.current && msg.sender_id !== userId) {
          const now = new Date().toISOString();
          markAsRead(msg.conversation_id, userId).catch(() => {});
          setConversations((prev) => markConversationReadLocally(prev, msg.conversation_id, userId, now));
        }
      },
      onMessageEdited: (edit) => {
        setMessagesByConversation((prev) => ({
          ...prev,
          [edit.conversation_id]: (prev[edit.conversation_id] ?? []).map((m) =>
            m.id === edit.id ? { ...m, content: edit.content, updated_at: edit.updated_at, edited: true } : m,
          ),
        }));
      },
      onConversation: (conv) => {
        setConversations((prev) => (prev.some((c) => c.id === conv.id) ? prev : [conv, ...prev]));
      },
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

    const now = new Date().toISOString();
    markAsRead(currentConversationId, userId)
      .then(() => {
        setConversations((prev) => markConversationReadLocally(prev, currentConversationId, userId, now));
      })
      .catch(() => {});
  }, [currentConversationId, userId]);

  const handleSend = useCallback(
    (content: string, kind: 'text' | 'image' = 'text') => {
      if (!currentConversationId) {
        return;
      }
      sendMessage(currentConversationId, userId, content, kind);
    },
    [currentConversationId, userId],
  );

  const handleEditMessage = useCallback(
    (messageId: string, content: string) => editMessage(messageId, userId, content),
    [userId],
  );

  const handleCreateConversation = useCallback((conversation: Conversation) => {
    setConversations((prev) => (prev.some((c) => c.id === conversation.id) ? prev : [conversation, ...prev]));
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
        displayName={displayName}
        onSignOut={onSignOut}
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
              onEditMessage={handleEditMessage}
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
