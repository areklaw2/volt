import type { Conversation } from '@/types';
import { getLastMessage, getUnreadCount } from '@/data/dummy';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';

const AVATAR_COLORS = [
  'bg-emerald-200 text-emerald-800',
  'bg-violet-200 text-violet-800',
  'bg-amber-200 text-amber-800',
  'bg-sky-200 text-sky-800',
  'bg-rose-200 text-rose-800',
];

function getAvatarColor(id: string): string {
  let hash = 0;
  for (const ch of id) hash = (hash * 31 + ch.charCodeAt(0)) | 0;
  return AVATAR_COLORS[Math.abs(hash) % AVATAR_COLORS.length];
}

function getConversationName(conversation: Conversation, currentUserId: string): string {
  if (conversation.name) {
    return conversation.name;
  }
  const others = conversation.participants.filter((p) => p.user_id !== currentUserId);
  if (others.length === 0) {
    return 'Unknown';
  }
  if (others.length === 1) {
    return others[0].display_name ?? 'Unknown';
  }
  return others.map((p) => p.display_name ?? 'Unknown').join(', ');
}

function getInitials(name: string): string {
  return name
    .split(' ')
    .map((w) => w[0])
    .join('')
    .toUpperCase()
    .slice(0, 2);
}

function formatTime(dateStr: string): string {
  const d = new Date(dateStr);
  return d.toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' });
}

interface ConversationItemProps {
  conversation: Conversation;
  currentUserId: string;
  isActive: boolean;
  onClick: () => void;
}

export function ConversationItem({ conversation, currentUserId, isActive, onClick }: ConversationItemProps) {
  const name = getConversationName(conversation, currentUserId);
  const initials = getInitials(name);
  const lastMsg = getLastMessage(conversation.id);
  const unreadCount = getUnreadCount(conversation.id, currentUserId);
  const isOwnLastMsg = lastMsg?.sender_id === currentUserId;

  return (
    <button
      onClick={onClick}
      className={`flex w-full items-center gap-3 border-b border-border/50 px-3 py-3 text-left text-sm transition-colors hover:bg-sidebar-accent ${
        isActive ? 'bg-sidebar-accent' : ''
      }`}
    >
      <Avatar className={`h-11 w-11 shrink-0 ${getAvatarColor(conversation.id)}`}>
        <AvatarFallback className={`text-xs font-semibold ${getAvatarColor(conversation.id)}`}>
          {initials}
        </AvatarFallback>
      </Avatar>
      <div className="flex-1 overflow-hidden">
        <div className="flex items-center justify-between">
          <span className="truncate font-medium">{name}</span>
          {lastMsg && (
            <span className="ml-2 shrink-0 text-xs text-muted-foreground">{formatTime(lastMsg.created_at)}</span>
          )}
        </div>
        {lastMsg && (
          <p className="truncate text-xs text-muted-foreground">
            {isOwnLastMsg && <span className="text-foreground/60">you: </span>}
            {lastMsg.content}
          </p>
        )}
      </div>
      {unreadCount > 0 && (
        <span className="flex h-5 w-5 shrink-0 items-center justify-center rounded-full bg-primary text-[10px] font-medium text-primary-foreground">
          {unreadCount}
        </span>
      )}
    </button>
  );
}
