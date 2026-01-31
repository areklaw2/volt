import type { Message } from '@/types';
import { env } from '@/lib/env';

const WS_BASE = `${env.WS_URL}/api/v1`;

let _ws: WebSocket | null = null;
let _pending: string[] = [];

export function connectWebSocket(userId: string, onMessage: (message: Message) => void): void {
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
      const msg: Message = JSON.parse(event.data);
      onMessage(msg);
    } catch {
      // ignore unparseable messages
    }
  };

  ws.onclose = () => {
    if (_ws === ws) _ws = null;
  };
}

export function sendMessage(conversationId: string, senderId: string, content: string): void {
  const payload = JSON.stringify({ conversation_id: conversationId, sender_id: senderId, content });
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
