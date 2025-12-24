import { useNavigate } from 'react-router-dom';
import type { Conversation } from '@/types';
import { Avatar } from '@/components/ui/avatar';
import { Badge } from '@/components/ui/badge';
import { formatTimestamp } from '@/lib/dateUtils';
import { cn } from '@/lib/utils';

interface ConversationListItemProps {
  conversation: Conversation;
  isSelected: boolean;
  currentUserId: string;
}

export function ConversationListItem({
  conversation,
  isSelected,
  currentUserId,
}: ConversationListItemProps) {
  const navigate = useNavigate();

  const handleClick = () => {
    navigate(`/chat/${conversation.id}`);
  };

  // Get conversation title - use title for groups, or participant username for direct chats
  const getTitle = () => {
    if (conversation.title) {
      return conversation.title;
    }
    // For direct chats, show the other participant's ID (will be username in real app)
    const otherParticipant = conversation.participants.find(
      (p) => p !== currentUserId
    );
    return otherParticipant || 'Unknown';
  };

  const title = getTitle();
  const lastMessagePreview =
    conversation.last_message?.content || 'No messages yet';
  const timestamp = conversation.last_message?.created_at
    ? formatTimestamp(conversation.last_message.created_at)
    : '';

  return (
    <button
      onClick={handleClick}
      className={cn(
        'w-full p-4 flex items-start gap-3 hover:bg-accent transition-colors text-left',
        isSelected && 'bg-accent'
      )}
    >
      <Avatar className="h-12 w-12">
        <div className="h-full w-full bg-primary/10 flex items-center justify-center text-primary font-semibold">
          {title[0]?.toUpperCase()}
        </div>
      </Avatar>

      <div className="flex-1 min-w-0">
        <div className="flex items-center justify-between gap-2 mb-1">
          <h3 className="font-semibold truncate">{title}</h3>
          {timestamp && (
            <span className="text-xs text-muted-foreground shrink-0">
              {timestamp}
            </span>
          )}
        </div>
        <p className="text-sm text-muted-foreground truncate">
          {lastMessagePreview}
        </p>
      </div>

      {conversation.unread_message_count > 0 && (
        <Badge variant="default" className="shrink-0">
          {conversation.unread_message_count}
        </Badge>
      )}
    </button>
  );
}
