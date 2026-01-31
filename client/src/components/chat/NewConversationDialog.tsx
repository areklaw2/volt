import { X } from 'lucide-react';
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogTrigger } from '@/components/ui/dialog';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { Badge } from '@/components/ui/badge';
import type { User, Conversation } from '@/types';
import { fetchUsers, createConversation } from '@/services/api';
import { useEffect, useMemo, useState } from 'react';

interface NewConversationDialogProps {
  currentUserId: string;
  trigger: React.ReactNode;
  onCreate: (conversation: Conversation) => void;
}

export function NewConversationDialog({ currentUserId, trigger, onCreate }: NewConversationDialogProps) {
  const [open, setOpen] = useState(false);
  const [users, setUsers] = useState<User[]>([]);
  const [search, setSearch] = useState('');
  const [selected, setSelected] = useState<User[]>([]);
  const [conversationName, setConversationName] = useState('');
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    if (!open) return;
    setSearch('');
    setSelected([]);
    setConversationName('');

    fetchUsers(currentUserId)
      .then(setUsers)
      .catch(() => setUsers([]));
  }, [open, currentUserId]);

  const filtered = useMemo(() => {
    const selectedIds = new Set(selected.map((u) => u.id));
    const available = users.filter((u) => !selectedIds.has(u.id));
    if (!search.trim()) return available;
    const q = search.toLowerCase();
    return available.filter((u) => u.username.toLowerCase().includes(q) || u.display_name.toLowerCase().includes(q));
  }, [users, search, selected]);

  const addUser = (user: User) => {
    setSelected((prev) => [...prev, user]);
    setSearch('');
  };

  const removeUser = (userId: string) => {
    setSelected((prev) => prev.filter((u) => u.id !== userId));
  };

  const handleCreate = async () => {
    if (selected.length === 0) return;
    setLoading(true);
    try {
      const participants = selected.map((u) => u.id);
      participants.push(currentUserId);
      const isGroup = participants.length > 2;
      const name = isGroup ? conversationName : selected[0].display_name;

      const conversation = await createConversation({
        conversation_type: isGroup ? 'group' : 'direct',
        sender_id: currentUserId,
        participants: participants,
        name: name,
      });
      onCreate(conversation);
      setOpen(false);
    } finally {
      setLoading(false);
    }
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger asChild>{trigger}</DialogTrigger>
      <DialogContent className="sm:max-w-md">
        <DialogHeader>
          <DialogTitle>New Conversation</DialogTitle>
        </DialogHeader>
        <div className="flex flex-col gap-3">
          {selected.length > 0 && (
            <div className="flex flex-wrap gap-1">
              {selected.map((u) => (
                <Badge key={u.id} variant="secondary" className="gap-1 pr-1">
                  {u.display_name}
                  <button
                    type="button"
                    onClick={() => removeUser(u.id)}
                    className="ml-0.5 rounded-full p-0.5 hover:bg-muted"
                  >
                    <X className="h-3 w-3" />
                  </button>
                </Badge>
              ))}
            </div>
          )}

          <Input placeholder="Search users..." value={search} onChange={(e) => setSearch(e.target.value)} />

          {search.trim() && filtered.length > 0 && (
            <div className="max-h-40 overflow-y-auto rounded-md border">
              {filtered.map((u) => (
                <button
                  key={u.id}
                  type="button"
                  className="flex w-full items-center gap-2 px-3 py-2 text-sm hover:bg-accent"
                  onClick={() => addUser(u)}
                >
                  <span className="font-medium">{u.display_name}</span>
                  <span className="text-muted-foreground">@{u.username}</span>
                </button>
              ))}
            </div>
          )}

          {search.trim() && filtered.length === 0 && <p className="text-sm text-muted-foreground">No users found</p>}

          {selected.length >= 2 && (
            <Input
              placeholder="Group name (optional)"
              value={conversationName}
              onChange={(e) => setConversationName(e.target.value)}
            />
          )}

          <Button onClick={handleCreate} disabled={selected.length === 0 || loading}>
            {loading ? 'Creating...' : 'Create'}
          </Button>
        </div>
      </DialogContent>
    </Dialog>
  );
}
