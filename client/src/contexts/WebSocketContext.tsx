import { createContext, useContext, type ReactNode } from 'react';

export type ConnectionState =
  | 'connecting'
  | 'connected'
  | 'disconnected'
  | 'error';

export interface WebSocketProviderProps {
  userId: string;
  onMessage: (message: string) => void;
  children: ReactNode;
}

export interface WebSocketContextType {
  sendMessage: (message: string) => void;
  connectionState: 'connecting' | 'connected' | 'disconnected' | 'error';
  error: string | null;
}

export const WebSocketContext = createContext<WebSocketContextType | undefined>(
  undefined
);

export function useWebSocketContext() {
  const context = useContext(WebSocketContext);
  if (context === undefined) {
    throw new Error(
      'useWebSocketContext must be used within WebSocketProvider'
    );
  }
  return context;
}
