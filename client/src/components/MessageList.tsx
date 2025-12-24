import { useEffect, useRef } from 'react';
import type { Message } from '@/types';
import { ScrollArea } from '@/components/ui/scroll-area';
import { MessageBubble } from './MessageBubble';

interface MessageListProps {
  messages: Message[];
  currentUserId: string;
}

export function MessageList({ messages, currentUserId }: MessageListProps) {
  const scrollAreaRef = useRef<HTMLDivElement>(null);
  const bottomRef = useRef<HTMLDivElement>(null);

  // Auto-scroll to bottom when messages change
  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages]);

  if (messages.length === 0) {
    return (
      <div className="flex-1 flex items-center justify-center">
        <p className="text-muted-foreground">
          No messages yet. Start the conversation!
        </p>
      </div>
    );
  }

  return (
    <ScrollArea className="flex-1 px-4" ref={scrollAreaRef}>
      <div className="py-4">
        {messages.map((message, index) => {
          const isCurrentUser = message.sender_id === currentUserId;
          const prevMessage = messages[index - 1];
          const showAvatar =
            !prevMessage || prevMessage.sender_id !== message.sender_id;

          return (
            <MessageBubble
              key={message.id}
              message={message}
              isCurrentUser={isCurrentUser}
              showAvatar={showAvatar}
            />
          );
        })}
        <div ref={bottomRef} />
      </div>
    </ScrollArea>
  );
}
