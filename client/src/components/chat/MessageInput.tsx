import { useState, useRef, type KeyboardEvent, type ChangeEvent } from "react";
import { Paperclip } from "lucide-react";
import { Button } from "@/components/ui/button";
import { uploadImage } from "@/services/api";

interface MessageInputProps {
  onSend: (content: string, kind?: "text" | "image") => void;
}

export function MessageInput({ onSend }: MessageInputProps) {
  const [value, setValue] = useState("");
  const [uploading, setUploading] = useState(false);
  const textareaRef = useRef<HTMLTextAreaElement>(null);
  const fileInputRef = useRef<HTMLInputElement>(null);

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

  async function handleFileSelected(e: ChangeEvent<HTMLInputElement>) {
    const file = e.target.files?.[0];
    e.target.value = "";
    if (!file) return;

    setUploading(true);
    try {
      const { url } = await uploadImage(file);
      onSend(url, "image");
    } catch {
      // upload failed — nothing to send, user can retry
    } finally {
      setUploading(false);
    }
  }

  return (
    <div className="border-t px-4 py-3">
      <div className="flex items-end gap-2">
        <input ref={fileInputRef} type="file" accept="image/png,image/jpeg,image/gif,image/webp" className="hidden" onChange={handleFileSelected} />
        <Button
          variant="ghost"
          size="icon"
          className="shrink-0 text-muted-foreground"
          title="Attach image"
          disabled={uploading}
          onClick={() => fileInputRef.current?.click()}
        >
          <Paperclip className="h-4 w-4" />
        </Button>
        <textarea
          ref={textareaRef}
          value={value}
          onChange={(e) => setValue(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder={uploading ? "Uploading image..." : "Type a message..."}
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
