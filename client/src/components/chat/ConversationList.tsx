import type { Conversation } from '@/types';
import { ConversationItem } from './ConversationItem';

interface ConversationListProps {
  conversations: Conversation[];
  currentUserId: string;
  activeId: string | null;
  onSelect: (id: string) => void;
  unreadCounts?: Record<string, number>;
}

export function ConversationList({
  conversations,
  currentUserId,
  activeId,
  onSelect,
  unreadCounts,
}: ConversationListProps) {
  if (conversations.length === 0) {
    return <div className="px-3 py-8 text-center text-sm text-muted-foreground">No conversations found</div>;
  }

  return (
    <div className="flex flex-col gap-0.5">
      {conversations.map((conversation) => (
        <ConversationItem
          key={conversation.id}
          conversation={conversation}
          currentUserId={currentUserId}
          isActive={conversation.id === activeId}
          onClick={() => onSelect(conversation.id)}
          unreadCount={unreadCounts?.[conversation.id] ?? 0}
        />
      ))}
    </div>
  );
}
