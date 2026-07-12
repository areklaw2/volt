import { SidebarProvider, SidebarInset } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import type { Conversation } from "@/types";

interface AppLayoutProps {
  activeConversationId: string | null;
  onSelectConversation: (id: string) => void;
  conversations: Conversation[];
  currentUserId: string;
  displayName: string;
  onSignOut: () => void;
  onCreateConversation: (conversation: Conversation) => void;
  unreadCounts?: Record<string, number>;
  children: React.ReactNode;
}

export function AppLayout({
  activeConversationId,
  onSelectConversation,
  conversations,
  currentUserId,
  displayName,
  onSignOut,
  onCreateConversation,
  unreadCounts,
  children,
}: AppLayoutProps) {
  return (
    <SidebarProvider>
      <AppSidebar
        activeConversationId={activeConversationId}
        onSelectConversation={onSelectConversation}
        conversations={conversations}
        currentUserId={currentUserId}
        displayName={displayName}
        onSignOut={onSignOut}
        onCreateConversation={onCreateConversation}
        unreadCounts={unreadCounts}
      />
      <SidebarInset>{children}</SidebarInset>
    </SidebarProvider>
  );
}
