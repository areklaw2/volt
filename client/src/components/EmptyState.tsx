import { MessageSquare } from 'lucide-react';

export function EmptyState() {
  return (
    <div className="flex-1 flex items-center justify-center p-8">
      <div className="text-center space-y-4">
        <div className="flex justify-center">
          <div className="h-16 w-16 rounded-full bg-primary/10 flex items-center justify-center">
            <MessageSquare className="h-8 w-8 text-primary" />
          </div>
        </div>
        <div className="space-y-2">
          <h2 className="text-xl font-semibold">Welcome to Volt Chat</h2>
          <p className="text-muted-foreground">
            Select a conversation from the sidebar or start a new chat
          </p>
        </div>
      </div>
    </div>
  );
}
