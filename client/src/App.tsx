import { useState, useCallback, useEffect } from 'react';
import { ThemeProvider } from '@/components/theme-provider';
import { AppLayout } from '@/components/app-layout';
import { MessageList } from '@/components/chat/MessageList';
import { MessageInput } from '@/components/chat/MessageInput';
import { conversations as dummyConversations, messagesByConversation } from '@/data/dummy';
import type { Message, Conversation } from '@/types';
import { MessageSquare, Info } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { useUser, useAuth } from '@clerk/react-router';
import { initApi, createUser, fetchConversations } from '@/services/api';

function App() {
  const { user } = useUser();

  const { getToken } = useAuth();
  initApi(getToken);

  useEffect(() => {
    if (!user) {
      return;
    }

    const username = user.username || user.primaryEmailAddress?.emailAddress || user.id;
    const displayName = user.fullName || username;
    createUser(user.id, username, displayName).catch(() => {});
  }, [user]);

  const [conversations, setConversations] = useState<Conversation[]>(dummyConversations);
  const [currentConversationId, setCurrentConversationId] = useState<string | null>(null);
  const [localMessages, setLocalMessages] = useState<Record<string, Message[]>>(() => ({ ...messagesByConversation }));

  const currentConversation = conversations.find((c) => c.id === currentConversationId) ?? null;
  const messages = currentConversationId ? (localMessages[currentConversationId] ?? []) : [];

  const userId = user?.id || '';
  useEffect(() => {
    fetchConversations(userId)
      .then(setConversations)
      .catch(() => setConversations([]));
  }, [userId]);

  const handleSend = useCallback(
    (content: string) => {
      if (!currentConversationId || userId !== '') {
        return;
      }

      const msg: Message = {
        id: `m-local-${Date.now()}`,
        conversation_id: currentConversationId,
        sender_id: userId,
        content,
        created_at: new Date().toISOString(),
        updated_at: null,
      };
      setLocalMessages((prev) => ({
        ...prev,
        [currentConversationId]: [...(prev[currentConversationId] ?? []), msg],
      }));
    },
    [currentConversationId, userId],
  );

  const handleCreateConversation = useCallback((conversation: Conversation) => {
    setConversations((prev) => [conversation, ...prev]);
    setCurrentConversationId(conversation.id);
  }, []);

  function getConversationName(conv: Conversation): string {
    if (conv.name) return conv.name;
    const other = conv.participants.find((p) => p.user_id !== userId);
    return other?.display_name ?? 'Unknown';
  }

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
      <AppLayout
        activeConversationId={currentConversationId}
        onSelectConversation={setCurrentConversationId}
        conversations={conversations}
        currentUserId={userId}
        onCreateConversation={handleCreateConversation}
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
            <MessageList messages={messages} isGroup={currentConversation.conversation_type === 'group'} />
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
