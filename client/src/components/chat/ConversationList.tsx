import type { Conversation } from '@/types';
import { ConversationItem } from './ConversationItem';

interface ConversationListProps {
  conversations: Conversation[];
  activeId: string | null;
  onSelect: (id: string) => void;
}

export function ConversationList({
  conversations,
  activeId,
  onSelect,
}: ConversationListProps) {
  if (conversations.length === 0) {
    return (
      <div className="px-3 py-8 text-center text-sm text-muted-foreground">
        No conversations found
      </div>
    );
  }

  return (
    <div className="flex flex-col gap-0.5">
      {conversations.map((conv) => (
        <ConversationItem
          key={conv.id}
          conversation={conv}
          isActive={conv.id === activeId}
          onClick={() => onSelect(conv.id)}
        />
      ))}
    </div>
  );
}
