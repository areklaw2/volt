import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { chatService } from '@/services/chatService';
import { searchMockUsers } from '@/lib/mockUsers';
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { Input } from '@/components/ui/input';
import { Avatar } from '@/components/ui/avatar';
import { ScrollArea } from '@/components/ui/scroll-area';
import { Separator } from '@/components/ui/separator';
import type { User } from '@/types';
import { useChatContext } from '@/contexts/ChatContext';

interface UserSearchDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
}

export function UserSearchDialog({
  open,
  onOpenChange,
}: UserSearchDialogProps) {
  const [searchQuery, setSearchQuery] = useState('');
  const [isCreating, setIsCreating] = useState(false);
  const { state, addConversation } = useChatContext();
  const navigate = useNavigate();

  const filteredUsers = searchMockUsers(searchQuery);

  const handleSelectUser = async (user: User) => {
    if (!state.currentUserId || isCreating) return;

    try {
      setIsCreating(true);

      // Create new conversation
      const newConversation = await chatService.createConversation({
        conversation_type: 'direct',
        message_type: 'text',
        first_message: `Started conversation with ${user.username}`,
        sender_id: state.currentUserId,
        participants: [state.currentUserId, user.id],
      });

      // Add to context
      addConversation(newConversation);

      // Close dialog and navigate
      onOpenChange(false);
      navigate(`/chat/${newConversation.id}`);
      setSearchQuery('');
    } catch (error) {
      console.error('Failed to create conversation:', error);
    } finally {
      setIsCreating(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>Start a new chat</DialogTitle>
        </DialogHeader>
        <div className="space-y-4">
          <Input
            placeholder="Search users..."
            value={searchQuery}
            onChange={(e) => setSearchQuery(e.target.value)}
            autoFocus
          />
          <ScrollArea className="h-75">
            {filteredUsers.length === 0 ? (
              <div className="flex items-center justify-center p-8 text-center">
                <p className="text-sm text-muted-foreground">No users found</p>
              </div>
            ) : (
              filteredUsers.map((user, index) => (
                <div key={user.id}>
                  {index > 0 && <Separator />}
                  <button
                    onClick={() => handleSelectUser(user)}
                    disabled={isCreating}
                    className="w-full p-3 flex items-center gap-3 hover:bg-accent transition-colors text-left disabled:opacity-50"
                  >
                    <Avatar className="h-10 w-10">
                      <div className="h-full w-full bg-primary/10 flex items-center justify-center text-primary font-semibold">
                        {user.username[0]?.toUpperCase()}
                      </div>
                    </Avatar>
                    <div className="flex-1 min-w-0">
                      <p className="font-medium truncate">{user.username}</p>
                      <p className="text-xs text-muted-foreground capitalize">
                        {user.status}
                      </p>
                    </div>
                  </button>
                </div>
              ))
            )}
          </ScrollArea>
        </div>
      </DialogContent>
    </Dialog>
  );
}
