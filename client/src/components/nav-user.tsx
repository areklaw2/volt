import { LogOut } from "lucide-react";
import {
  SidebarMenu,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { Button } from "@/components/ui/button";
import { Avatar, AvatarFallback } from "@/components/ui/avatar";
import { ModeToggle } from "@/components/mode-toggle";

interface NavUserProps {
  displayName: string;
  onSignOut: () => void;
}

function getInitials(name: string): string {
  return name
    .split(" ")
    .map((w) => w[0])
    .join("")
    .toUpperCase()
    .slice(0, 2);
}

export function NavUser({ displayName, onSignOut }: NavUserProps) {
  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <div className="flex items-center gap-2 px-2 py-1.5">
          <Avatar className="h-7 w-7">
            <AvatarFallback className="text-xs">{getInitials(displayName)}</AvatarFallback>
          </Avatar>
          <span className="truncate text-sm font-medium group-data-[collapsible=icon]:hidden">
            {displayName}
          </span>
          <div className="ml-auto flex items-center gap-1 group-data-[collapsible=icon]:hidden">
            <ModeToggle />
            <Button variant="ghost" size="icon" title="Sign out" onClick={onSignOut}>
              <LogOut className="h-4 w-4" />
            </Button>
          </div>
        </div>
      </SidebarMenuItem>
    </SidebarMenu>
  );
}
