import type { Message as MessageType, User } from '@/types';
import { currentUser, getUserById } from '@/data/dummy';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';

interface MessageProps {
  message: MessageType;
  showSenderName?: boolean;
}

function formatTime(dateStr: string): string {
  return new Date(dateStr).toLocaleTimeString([], {
    hour: '2-digit',
    minute: '2-digit',
  });
}

//TODO: fix dark mode colors
export function Message({ message, showSenderName = false }: MessageProps) {
  const isOwn = message.sender_id === currentUser.id;
  const sender: User | undefined = getUserById(message.sender_id);
  const initials = sender
    ? sender.display_name
        .split(' ')
        .map((w) => w[0])
        .join('')
        .toUpperCase()
        .slice(0, 2)
    : '?';

  return (
    <div className={`flex gap-2.5 ${isOwn ? 'flex-row-reverse' : 'flex-row'}`}>
      {!isOwn && (
        <Avatar className="mt-0.5 h-8 w-8 shrink-0">
          <AvatarFallback className="text-[11px]">{initials}</AvatarFallback>
        </Avatar>
      )}
      <div
        className={`flex max-w-[70%] flex-col ${isOwn ? 'items-end' : 'items-start'}`}
      >
        {!isOwn && showSenderName && sender && (
          <span className="mb-0.5 text-xs font-medium text-muted-foreground">
            {sender.display_name}
          </span>
        )}
        <div
          className={`rounded-2xl px-3.5 py-2 text-sm ${
            isOwn ? 'bg-muted' : 'bg-white shadow-sm ring-1 ring-border/50'
          }`}
        >
          {message.content}
        </div>
        <span className="mt-0.5 text-[10px] text-muted-foreground">
          {formatTime(message.created_at)}
        </span>
      </div>
    </div>
  );
}
