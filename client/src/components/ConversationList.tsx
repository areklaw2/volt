import { useParams } from 'react-router-dom';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Separator } from '@/components/ui/separator';
import { Skeleton } from '@/components/ui/skeleton';
import { ConversationListItem } from './ConversationListItem';
import { useChatContext } from '@/contexts/ChatContext';

export function ConversationList() {
  const { conversationId } = useParams();
  const { state } = useChatContext();

  console.log(state);
  if (state.isLoading) {
    return (
      <div className="p-4 space-y-4">
        {[1, 2, 3].map((i) => (
          <div key={i} className="flex gap-3">
            <Skeleton className="h-12 w-12 rounded-full" />
            <div className="flex-1 space-y-2">
              <Skeleton className="h-4 w-3/4" />
              <Skeleton className="h-3 w-1/2" />
            </div>
          </div>
        ))}
      </div>
    );
  }

  if (state.conversations.length === 0) {
    return (
      <div className="flex items-center justify-center p-8 text-center">
        <p className="text-sm text-muted-foreground">
          No conversations yet. Start a new chat!
        </p>
      </div>
    );
  }

  return (
    <ScrollArea className="flex-1">
      {state.conversations.map((conversation, index) => (
        <div key={conversation.id}>
          {index > 0 && <Separator />}
          <ConversationListItem
            conversation={conversation}
            isSelected={conversation.id === conversationId}
            currentUserId={state.currentUserId!}
          />
        </div>
      ))}
    </ScrollArea>
  );
}
