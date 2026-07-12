import type { Message, Conversation } from '@/types';
import { env } from '@/lib/env';

const WS_BASE = `${env.WS_URL}/api/v1`;

let _ws: WebSocket | null = null;
let _pending: string[] = [];

interface WsHandlers {
  onMessage: (message: Message) => void;
  onConversation: (conversation: Conversation) => void;
}

export function connectWebSocket(userId: string, handlers: WsHandlers): void {
  if (_ws) {
    disconnectWebSocket();
  }

  const ws = new WebSocket(`${WS_BASE}/chat/${userId}`);
  _ws = ws;

  ws.onopen = () => {
    for (const msg of _pending) {
      ws.send(msg);
    }
    _pending = [];
  };

  ws.onmessage = (event) => {
    try {
      const envelope = JSON.parse(event.data);
      if (envelope.type === 'message') {
        handlers.onMessage(envelope.message as Message);
      } else if (envelope.type === 'conversation') {
        handlers.onConversation(envelope.conversation as Conversation);
      }
    } catch {
      // ignore unparseable messages
    }
  };

  ws.onclose = () => {
    if (_ws === ws) _ws = null;
  };
}

export function sendMessage(conversationId: string, senderId: string, content: string, kind: 'text' | 'image' = 'text'): void {
  const payload = JSON.stringify({ conversation_id: conversationId, sender_id: senderId, content, kind });
  if (!_ws) {
    return;
  }

  if (_ws.readyState === WebSocket.OPEN) {
    _ws.send(payload);
  } else if (_ws.readyState === WebSocket.CONNECTING) {
    _pending.push(payload);
  }
}

export function disconnectWebSocket(): void {
  if (_ws) {
    _ws.close();
    _ws = null;
  }
  _pending = [];
}
