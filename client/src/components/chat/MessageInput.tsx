import { useState, useRef, type KeyboardEvent } from "react";
import { Paperclip } from "lucide-react";
import { Button } from "@/components/ui/button";

interface MessageInputProps {
  onSend: (content: string) => void;
}

export function MessageInput({ onSend }: MessageInputProps) {
  const [value, setValue] = useState("");
  const textareaRef = useRef<HTMLTextAreaElement>(null);

  function handleSend() {
    const trimmed = value.trim();
    if (!trimmed) return;
    onSend(trimmed);
    setValue("");
    textareaRef.current?.focus();
  }

  function handleKeyDown(e: KeyboardEvent<HTMLTextAreaElement>) {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSend();
    }
  }

  return (
    <div className="border-t px-4 py-3">
      <div className="flex items-end gap-2">
        <Button
          variant="ghost"
          size="icon"
          className="shrink-0 text-muted-foreground"
          title="Attach file"
        >
          <Paperclip className="h-4 w-4" />
        </Button>
        <textarea
          ref={textareaRef}
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Type a message..."
          rows={1}
          className="flex-1 resize-none bg-transparent px-2 py-2 text-sm outline-none placeholder:text-muted-foreground"
        />
        <Button
          variant="ghost"
          onClick={handleSend}
          disabled={!value.trim()}
          className="shrink-0 font-semibold text-primary hover:text-primary"
        >
          Send message
        </Button>
      </div>
    </div>
  );
}
