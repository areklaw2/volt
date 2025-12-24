import { useState } from 'react';
import { Button } from '@/components/ui/button';
import { Separator } from '@/components/ui/separator';
import { ConversationList } from './ConversationList';
import { UserSearchDialog } from './UserSearchDialog';
import { MessageSquarePlus } from 'lucide-react';

export function ConversationSidebar() {
  const [isSearchOpen, setIsSearchOpen] = useState(false);

  return (
    <div className="w-80 border-r border-sidebar-border bg-sidebar flex flex-col h-full">
      {/* Header */}
      <div className="p-4 space-y-4">
        <div className="flex items-center justify-between">
          <h2 className="text-lg font-semibold">Chats</h2>
          <Button
            size="sm"
            variant="ghost"
            onClick={() => setIsSearchOpen(true)}
            title="New Chat"
          >
            <MessageSquarePlus className="h-5 w-5" />
          </Button>
        </div>
      </div>

      <Separator />

      {/* Conversation List */}
      <ConversationList />

      {/* User Search Dialog */}
      <UserSearchDialog open={isSearchOpen} onOpenChange={setIsSearchOpen} />
    </div>
  );
}
