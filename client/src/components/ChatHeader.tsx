import type { Conversation } from '@/types';
import { Avatar } from '@/components/ui/avatar';
import { Separator } from '@/components/ui/separator';

interface ChatHeaderProps {
  conversation: Conversation;
  currentUserId: string;
}

export function ChatHeader({ conversation, currentUserId }: ChatHeaderProps) {
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
  const participantCount = conversation.participants.length;
  const isGroup = conversation.type === 'group';

  return (
    <>
      <div className="p-4 flex items-center gap-3">
        <Avatar className="h-10 w-10">
          <div className="h-full w-full bg-primary/10 flex items-center justify-center text-primary font-semibold">
            {title[0]?.toUpperCase()}
          </div>
        </Avatar>
        <div className="flex-1 min-w-0">
          <h2 className="font-semibold truncate">{title}</h2>
          <p className="text-sm text-muted-foreground">
            {isGroup ? `${participantCount} participants` : 'Online'}
          </p>
        </div>
      </div>
      <Separator />
    </>
  );
}
