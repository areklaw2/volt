import type { Message } from '@/types';
import { Avatar } from '@/components/ui/avatar';
import { formatMessageTime } from '@/lib/dateUtils';
import { cn } from '@/lib/utils';

interface MessageBubbleProps {
  message: Message;
  isCurrentUser: boolean;
  showAvatar?: boolean;
}

export function MessageBubble({
  message,
  isCurrentUser,
  showAvatar = true,
}: MessageBubbleProps) {
  return (
    <div
      className={cn(
        'flex gap-3 mb-4',
        isCurrentUser ? 'flex-row-reverse' : 'flex-row'
      )}
    >
      {showAvatar ? (
        <Avatar className="h-8 w-8">
          <div className="h-full w-full bg-primary/10 flex items-center justify-center text-primary text-sm font-semibold">
            {message.sender_id[0]?.toUpperCase()}
          </div>
        </Avatar>
      ) : (
        <div className="h-8 w-8" /> // Spacer to maintain alignment
      )}

      <div
        className={cn(
          'flex flex-col gap-1 max-w-[70%]',
          isCurrentUser ? 'items-end' : 'items-start'
        )}
      >
        <div
          className={cn(
            'px-4 py-2 rounded-2xl',
            isCurrentUser
              ? 'bg-primary text-primary-foreground'
              : 'bg-muted text-foreground'
          )}
        >
          <p className="text-sm whitespace-pre-wrap wrap-break-word">
            {message.content}
          </p>
        </div>
        <span className="text-xs text-muted-foreground px-2">
          {formatMessageTime(message.created_at)}
        </span>
      </div>
    </div>
  );
}
