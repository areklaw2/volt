import { SidebarProvider, SidebarInset } from "@/components/ui/sidebar";
import { AppSidebar } from "@/components/app-sidebar";
import type { Conversation } from "@/types";

interface AppLayoutProps {
  activeConversationId: string | null;
  onSelectConversation: (id: string) => void;
  conversations: Conversation[];
  currentUserId: string;
  onCreateConversation: (conversation: Conversation) => void;
  children: React.ReactNode;
}

export function AppLayout({
  activeConversationId,
  onSelectConversation,
  conversations,
  currentUserId,
  onCreateConversation,
  children,
}: AppLayoutProps) {
  return (
    <SidebarProvider>
      <AppSidebar
        activeConversationId={activeConversationId}
        onSelectConversation={onSelectConversation}
        conversations={conversations}
        currentUserId={currentUserId}
        onCreateConversation={onCreateConversation}
      />
      <SidebarInset>{children}</SidebarInset>
    </SidebarProvider>
  );
}
