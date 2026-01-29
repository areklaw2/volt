import { UserButton, useUser } from "@clerk/react-router";
import {
  SidebarMenu,
  SidebarMenuItem,
} from "@/components/ui/sidebar";
import { ModeToggle } from "@/components/mode-toggle";

export function NavUser() {
  const { user } = useUser();
  const displayName = user?.fullName ?? user?.username ?? "User";

  return (
    <SidebarMenu>
      <SidebarMenuItem>
        <div className="flex items-center gap-2 px-2 py-1.5">
          <UserButton />
          <span className="truncate text-sm font-medium group-data-[collapsible=icon]:hidden">
            {displayName}
          </span>
          <div className="ml-auto group-data-[collapsible=icon]:hidden">
            <ModeToggle />
          </div>
        </div>
      </SidebarMenuItem>
    </SidebarMenu>
  );
}
