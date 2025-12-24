import { useEffect, useState } from 'react';
import { useParams, Navigate } from 'react-router-dom';
import { chatService } from '@/services/chatService';
import { WebSocketProvider } from '@/components/WebSocketProvider';
import { ConversationSidebar } from '@/components/ConversationSidebar';
import { ChatWindow } from '@/components/ChatWindow';
import { EmptyState } from '@/components/EmptyState';
import { useChatContext } from '@/contexts/ChatContext';

export function ChatPage() {
  const { conversationId } = useParams();
  const { state, setConversations, setLoading } = useChatContext();
  const [incomingMessage, setIncomingMessage] = useState<string | null>(null);

  //TODO: move this state to the chat window
  const handleWebSocketMessage = (message: string) => {
    console.log('Received WebSocket message:', message);
    setIncomingMessage(message);
  };

  useEffect(() => {
    // Load conversations on mount
    if (!state.currentUserId) return;

    const loadConversations = async () => {
      try {
        setLoading(true);
        const response = await chatService.queryUserConversations(
          state.currentUserId!
        );
        setConversations(response.items);
      } catch (error) {
        console.error('Failed to load conversations:', error);
      } finally {
        setLoading(false);
      }
    };

    loadConversations();
  }, [setConversations, setLoading, state.currentUserId]);

  // Redirect to login if no user
  if (!state.currentUserId) {
    return <Navigate to="/" replace />;
  }

  return (
    <div className="flex h-screen bg-background">
      {/* Sidebar */}
      <ConversationSidebar />

      {/* Main content */}
      {conversationId ? (
        <WebSocketProvider
          userId={state.currentUserId}
          onMessage={handleWebSocketMessage}
        >
          <ChatWindow
            conversationId={conversationId}
            incomingMessage={incomingMessage}
          />
        </WebSocketProvider>
      ) : (
        <EmptyState />
      )}
    </div>
  );
}
