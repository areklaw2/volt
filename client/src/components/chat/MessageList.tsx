import { useEffect, useRef } from 'react';
import type { Message } from '@/types';
import { MessageItem } from './MessageItem';

interface MessageListProps {
  messages: Message[];
  isGroup?: boolean;
}

function formatDateSeparator(dateStr: string): string {
  const date = new Date(dateStr);
  const now = new Date();
  const diffDays = Math.floor(
    (now.getTime() - date.getTime()) / (1000 * 60 * 60 * 24),
  );
  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Yesterday';
  return date.toLocaleDateString(undefined, {
    weekday: 'long',
    month: 'short',
    day: 'numeric',
  });
}

export function MessageList({ messages, isGroup = false }: MessageListProps) {
  const bottomRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    bottomRef.current?.scrollIntoView({ behavior: 'smooth' });
  }, [messages.length]);

  // Group messages by date
  const groups: { date: string; messages: Message[] }[] = [];
  for (const msg of messages) {
    const dateKey = new Date(msg.created_at).toDateString();
    const last = groups[groups.length - 1];
    if (last && last.date === dateKey) {
      last.messages.push(msg);
    } else {
      groups.push({ date: dateKey, messages: [msg] });
    }
  }

  return (
    <div className="flex-1 overflow-y-auto bg-muted/30 px-4 py-4">
      <div className="flex flex-col gap-3">
        {groups.map((group) => (
          <div key={group.date}>
            <div className="my-3 flex items-center justify-center">
              <span className="text-xs font-medium text-muted-foreground">
                {formatDateSeparator(group.messages[0].created_at)}
              </span>
            </div>
            <div className="flex flex-col gap-2">
              {group.messages.map((msg) => (
                <MessageItem
                  key={msg.id}
                  message={msg}
                  showSenderName={isGroup}
                />
              ))}
            </div>
          </div>
        ))}
        <div ref={bottomRef} />
      </div>
    </div>
  );
}
