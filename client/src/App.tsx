import { useState, useCallback } from 'react';
import { ThemeProvider } from '@/components/theme-provider';
import { AppLayout } from '@/components/layout/AppLayout';
import { MessageList } from '@/components/chat/MessageList';
import { MessageInput } from '@/components/chat/MessageInput';
import {
  conversations,
  messagesByConversation,
  currentUser,
} from '@/data/dummy';
import type { Message, ConversationWithMeta } from '@/types';
import { MessageSquare, Info } from 'lucide-react';
import { Button } from '@/components/ui/button';

function getConversationName(conv: ConversationWithMeta): string {
  if (conv.title) return conv.title;
  const other = conv.participants.find((p) => p.id !== currentUser.id);
  return other?.display_name ?? 'Unknown';
}

function App() {
  const [activeId, setActiveId] = useState<string | null>(null);
  const [localMessages, setLocalMessages] = useState<Record<string, Message[]>>(
    () => ({ ...messagesByConversation }),
  );

  const activeConv = conversations.find((c) => c.id === activeId) ?? null;
  const messages = activeId ? (localMessages[activeId] ?? []) : [];

  const handleSend = useCallback(
    (content: string) => {
      if (!activeId) return;
      const msg: Message = {
        id: `m-local-${Date.now()}`,
        conversation_id: activeId,
        sender_id: currentUser.id,
        content,
        created_at: new Date().toISOString(),
        updated_at: null,
      };
      setLocalMessages((prev) => ({
        ...prev,
        [activeId]: [...(prev[activeId] ?? []), msg],
      }));
    },
    [activeId],
  );

  return (
    <ThemeProvider defaultTheme="dark" storageKey="vite-ui-theme">
    <AppLayout
      activeConversationId={activeId}
      onSelectConversation={setActiveId}
    >
      {activeConv ? (
        <div className="flex h-dvh flex-col">
          <header className="flex items-center justify-between border-b px-4 py-3">
            <div>
              <h1 className="text-lg font-semibold">
                {getConversationName(activeConv)}
              </h1>
              {activeConv.conversation_type === 'group' ? (
                <span className="text-xs text-muted-foreground">
                  {activeConv.participants.length} members
                </span>
              ) : (
                <span className="text-xs font-medium text-primary">Online</span>
              )}
            </div>
            <Button
              variant="ghost"
              size="icon"
              className="text-muted-foreground"
            >
              <Info className="h-5 w-5" />
            </Button>
          </header>
          <MessageList
            messages={messages}
            isGroup={activeConv.conversation_type === 'group'}
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
