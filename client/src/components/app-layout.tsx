import { SidebarProvider, SidebarInset } from '@/components/ui/sidebar';
import { AppSidebar } from '@/components/app-sidebar';

interface AppLayoutProps {
  activeConversationId: string | null;
  onSelectConversation: (id: string) => void;
  children: React.ReactNode;
}

export function AppLayout({
  activeConversationId,
  onSelectConversation,
  children,
}: AppLayoutProps) {
  return (
    <SidebarProvider>
      <AppSidebar
        activeConversationId={activeConversationId}
        onSelectConversation={onSelectConversation}
      />
      <SidebarInset>{children}</SidebarInset>
    </SidebarProvider>
  );
}
