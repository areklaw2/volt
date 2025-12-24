import { useRef, useState, useEffect, useCallback } from 'react';
import { env } from '@/env';
import {
  WebSocketContext,
  type ConnectionState,
  type WebSocketProviderProps,
} from '@/contexts/WebSocketContext';

export function WebSocketProvider({
  userId,
  onMessage,
  children,
}: WebSocketProviderProps) {
  const wsRef = useRef<WebSocket | null>(null);
  const [connectionState, setConnectionState] =
    useState<ConnectionState>('connecting');
  const [error, setError] = useState<string | null>(null);
  const reconnectTimeoutRef = useRef<number>(0);
  const messageQueueRef = useRef<string[]>([]);
  const connectRef = useRef<(() => void) | null>(null);

  const connect = useCallback(() => {
    if (wsRef.current?.readyState === WebSocket.OPEN || !userId) {
      return;
    }

    const ws = new WebSocket(env.SOCKET_URL);
    wsRef.current = ws;

    ws.onopen = () => {
      console.log('WebSocket connected');
      setConnectionState('connected');
      setError(null);

      // Send user_id as handshake
      ws.send(userId);

      // Send any queued messages
      while (messageQueueRef.current.length > 0) {
        const queuedMessage = messageQueueRef.current.shift();
        if (queuedMessage) {
          ws.send(queuedMessage);
        }
      }
    };

    ws.onmessage = (event) => {
      console.log('WebSocket message received:', event.data);
      onMessage(event.data);
    };

    ws.onerror = (event) => {
      console.error('WebSocket error:', event);
      setError('Connection error');
      setConnectionState('error');
    };

    ws.onclose = () => {
      console.log('WebSocket disconnected');
      setConnectionState('disconnected');
      wsRef.current = null;

      // Attempt to reconnect after 3 seconds
      reconnectTimeoutRef.current = window.setTimeout(() => {
        console.log('Attempting to reconnect...');
        setConnectionState('connecting');
        connectRef.current?.();
      }, 3000);
    };
  }, [userId, onMessage]);

  const sendMessage = useCallback((message: string) => {
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      wsRef.current.send(message);
    } else {
      // Queue message if not connected
      console.log('Queueing message (not connected):', message);
      messageQueueRef.current.push(message);
    }
  }, []);

  // Store connect function in ref so it can be called from onclose
  useEffect(() => {
    connectRef.current = connect;
  }, [connect]);

  useEffect(() => {
    connect();

    return () => {
      // Cleanup on unmount
      if (reconnectTimeoutRef.current) {
        clearTimeout(reconnectTimeoutRef.current);
      }
      if (wsRef.current) {
        wsRef.current.close();
        wsRef.current = null;
      }
    };
  }, [connect]);

  const value = {
    sendMessage,
    connectionState,
    error,
  };

  return (
    <WebSocketContext.Provider value={value}>
      {children}
    </WebSocketContext.Provider>
  );
}
