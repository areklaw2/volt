import { useEffect, useRef, useState, type KeyboardEvent } from 'react';
import { Pencil } from 'lucide-react';
import type { Message as MessageType, Participant } from '@/types';
import { Avatar, AvatarFallback } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';

interface MessageProps {
  message: MessageType;
  currentUserId: string;
  participants: Participant[];
  showSenderName?: boolean;
  onEdit: (messageId: string, content: string) => Promise<void>;
}

function formatTime(dateStr: string): string {
  return new Date(dateStr).toLocaleTimeString([], {
    hour: '2-digit',
    minute: '2-digit',
  });
}

//TODO: fix dark mode colors
export function MessageItem({ message, currentUserId, participants, showSenderName = false, onEdit }: MessageProps) {
  const isOwn = message.sender_id === currentUserId;
  // the server accepts an image edit only if the new content is a URL, which is not
  // worth exposing — text only
  const canEdit = isOwn && message.kind === 'text';

  const [isEditing, setIsEditing] = useState(false);
  const [draft, setDraft] = useState(message.content);
  const [error, setError] = useState<string | null>(null);
  const [saving, setSaving] = useState(false);
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  useEffect(() => {
    if (isEditing) {
      textareaRef.current?.focus();
      textareaRef.current?.select();
    }
  }, [isEditing]);

  const sender = participants.find((p) => p.user_id === message.sender_id);
  const initials = sender
    ? sender.display_name
        .split(' ')
        .map((w) => w[0])
        .join('')
        .toUpperCase()
        .slice(0, 2)
    : '?';

  function startEditing() {
    setDraft(message.content);
    setError(null);
    setIsEditing(true);
  }

  function cancelEditing() {
    setIsEditing(false);
    setError(null);
  }

  async function saveEdit() {
    const trimmed = draft.trim();
    if (!trimmed || saving) {
      return;
    }
    if (trimmed === message.content) {
      cancelEditing();
      return;
    }

    setSaving(true);
    try {
      await onEdit(message.id, trimmed);
      setIsEditing(false);
      setError(null);
    } catch (err) {
      // keep the editor open so the draft survives a rejection
      setError(err instanceof Error ? err.message : 'Failed to edit message');
    } finally {
      setSaving(false);
    }
  }

  function handleKeyDown(e: KeyboardEvent<HTMLTextAreaElement>) {
    if (e.key === 'Enter' && !e.shiftKey) {
      e.preventDefault();
      void saveEdit();
    } else if (e.key === 'Escape') {
      e.preventDefault();
      cancelEditing();
    }
  }

  return (
    <div className={`group flex gap-2.5 ${isOwn ? 'flex-row-reverse' : 'flex-row'}`}>
      {!isOwn && (
        <Avatar className="mt-0.5 h-8 w-8 shrink-0">
          <AvatarFallback className="text-[11px]">{initials}</AvatarFallback>
        </Avatar>
      )}
      <div
        className={`flex max-w-[70%] flex-col ${isOwn ? 'items-end' : 'items-start'}`}
      >
        {!isOwn && showSenderName && sender && (
          <span className="mb-0.5 text-xs font-medium text-muted-foreground">
            {sender.display_name}
          </span>
        )}
        {isEditing ? (
          <div className="w-full min-w-64 rounded-2xl bg-muted px-3.5 py-2">
            <textarea
              ref={textareaRef}
              value={draft}
              onChange={(e) => setDraft(e.target.value)}
              onKeyDown={handleKeyDown}
              disabled={saving}
              rows={1}
              className="w-full resize-none bg-transparent text-sm outline-none"
            />
            <span className="text-[10px] text-muted-foreground">
              Enter to save · Esc to cancel
            </span>
          </div>
        ) : (
          <div className={`flex items-center gap-1 ${isOwn ? 'flex-row-reverse' : 'flex-row'}`}>
            {message.kind === 'image' ? (
              <img
                src={message.content}
                alt="Shared image"
                className="max-h-64 max-w-full rounded-2xl object-cover shadow-sm ring-1 ring-border/50"
              />
            ) : (
              <div
                className={`rounded-2xl px-3.5 py-2 text-sm ${
                  isOwn ? 'bg-muted' : 'bg-white shadow-sm ring-1 ring-border/50'
                }`}
              >
                {message.content}
              </div>
            )}
            {canEdit && (
              <Button
                variant="ghost"
                size="icon"
                title="Edit message"
                onClick={startEditing}
                className="h-6 w-6 shrink-0 text-muted-foreground opacity-0 transition-opacity group-hover:opacity-100 focus-visible:opacity-100"
              >
                <Pencil className="h-3 w-3" />
              </Button>
            )}
          </div>
        )}
        {error && <span className="mt-0.5 text-[10px] text-destructive">{error}</span>}
        <span className="mt-0.5 text-[10px] text-muted-foreground">
          {formatTime(message.created_at)}
          {message.edited && ' · edited'}
        </span>
      </div>
    </div>
  );
}
