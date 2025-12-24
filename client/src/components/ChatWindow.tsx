import { useState, useEffect } from 'react';
import { useWebSocketContext } from '@/contexts/WebSocketContext';
import { chatService } from '@/services/chatService';
import type { Message } from '@/types';
import { ChatHeader } from './ChatHeader';
import { MessageList } from './MessageList';
import { MessageInput } from './MessageInput';
import { useChatContext } from '@/contexts/ChatContext';

interface ChatWindowProps {
  conversationId: string;
  incomingMessage: string | null;
}

export function ChatWindow({
  conversationId,
  incomingMessage,
}: ChatWindowProps) {
  const { state } = useChatContext();
  const { sendMessage: wsSendMessage } = useWebSocketContext();
  const [messages, setMessages] = useState<Message[]>([]);
  const [isLoading, setIsLoading] = useState(false);
  const [isSending, setIsSending] = useState(false);

  const conversation = state.conversations.find((c) => c.id === conversationId);

  // Load messages when conversation changes
  useEffect(() => {
    const loadMessages = async () => {
      try {
        setIsLoading(true);
        const fetchedMessages = await chatService.getMessages(conversationId);
        setMessages(fetchedMessages);
      } catch (error) {
        console.error('Failed to load messages:', error);
      } finally {
        setIsLoading(false);
      }
    };

    loadMessages();
  }, [conversationId]);

  // Handle incoming WebSocket messages
  useEffect(() => {
    if (incomingMessage) {
      console.log('Processing incoming message:', incomingMessage);

      // Parse the message (format: "username: message text")
      // For now, just add it as a new message
      const newMessage: Message = {
        id: `ws-${Date.now()}`,
        conversation_id: conversationId,
        sender_id: 'other-user', // Will be parsed from message
        content: incomingMessage,
        type: 'text',
        created_at: new Date().toISOString(),
      };

      setMessages((prev) => [...prev, newMessage]);
    }
  }, [incomingMessage, conversationId]);

  const handleSendMessage = async (content: string) => {
    if (!state.currentUserId) return;

    try {
      setIsSending(true);

      // Optimistic update
      const tempMessage: Message = {
        id: `temp-${Date.now()}`,
        conversation_id: conversationId,
        sender_id: state.currentUserId,
        content,
        type: 'text',
        created_at: new Date().toISOString(),
      };
      setMessages((prev) => [...prev, tempMessage]);

      // Send via WebSocket
      wsSendMessage(content);

      // Also send via REST API for persistence
      // await chatService.createMessage({
      //   conversation_id: conversationId,
      //   sender_id: state.currentUserId,
      //   content,
      //   kind: 'text',
      // });
    } catch (error) {
      console.error('Failed to send message:', error);
      // Remove optimistic message on error
      setMessages((prev) => prev.filter((m) => !m.id.startsWith('temp-')));
    } finally {
      setIsSending(false);
    }
  };

  if (!conversation) {
    return (
      <div className="flex-1 flex items-center justify-center">
        <p className="text-muted-foreground">Conversation not found</p>
      </div>
    );
  }

  return (
    <div className="flex-1 flex flex-col h-full">
      <ChatHeader
        conversation={conversation}
        currentUserId={state.currentUserId!}
      />
      {isLoading ? (
        <div className="flex-1 flex items-center justify-center">
          <p className="text-muted-foreground">Loading messages...</p>
        </div>
      ) : (
        <MessageList messages={messages} currentUserId={state.currentUserId!} />
      )}
      <MessageInput onSend={handleSendMessage} disabled={isSending} />
    </div>
  );
}
