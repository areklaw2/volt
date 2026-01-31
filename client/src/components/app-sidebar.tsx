import * as React from "react";
import { MessageSquarePlus } from "lucide-react";
import { ConversationList } from "@/components/chat/ConversationList";
import { NewConversationDialog } from "@/components/chat/NewConversationDialog";
import { NavUser } from "@/components/nav-user";
import { Button } from "@/components/ui/button";
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarHeader,
  SidebarInput,
  SidebarGroup,
  SidebarGroupContent,
  SidebarRail,
} from "@/components/ui/sidebar";
import type { Conversation } from "@/types";

interface AppSidebarProps extends React.ComponentProps<typeof Sidebar> {
  activeConversationId: string | null;
  onSelectConversation: (id: string) => void;
  conversations: Conversation[];
  currentUserId: string;
  onCreateConversation: (conversation: Conversation) => void;
}

export function AppSidebar({
  activeConversationId,
  onSelectConversation,
  conversations: conversationsProp,
  currentUserId,
  onCreateConversation,
  ...props
}: AppSidebarProps) {
  const [search, setSearch] = React.useState("");

  const filtered = React.useMemo(() => {
    if (!search.trim()) return conversationsProp;

    const q = search.toLowerCase();
    return conversationsProp.filter((conv) => {
      const name =
        conv.name ??
        conv.participants
          .filter((p) => p.user_id !== currentUserId)
          .map((p) => p.display_name)
          .join(", ");
      return name.toLowerCase().includes(q);
    });
  }, [search, conversationsProp, currentUserId]);

  return (
    <Sidebar collapsible="icon" {...props}>
      <SidebarHeader>
        <div className="flex items-center justify-between px-2 py-1">
          <span className="text-lg font-semibold group-data-[collapsible=icon]:hidden">Messages</span>
          <NewConversationDialog
            currentUserId={currentUserId}
            onCreate={onCreateConversation}
            trigger={
              <Button
                variant="ghost"
                size="icon"
                className="h-8 w-8 group-data-[collapsible=icon]:hidden"
                title="New Chat"
              >
                <MessageSquarePlus className="h-4 w-4" />
              </Button>
            }
          />
        </div>
        <div className="px-2 group-data-[collapsible=icon]:hidden">
          <SidebarInput
            placeholder="Search conversations..."
            value={search}
            onChange={(e) => setSearch(e.target.value)}
          />
        </div>
      </SidebarHeader>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupContent>
            <ConversationList
              conversations={filtered}
              currentUserId={currentUserId}
              activeId={activeConversationId}
              onSelect={onSelectConversation}
            />
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter>
        <NavUser />
      </SidebarFooter>
      <SidebarRail />
    </Sidebar>
  );
}
