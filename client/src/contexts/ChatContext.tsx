import type { Conversation } from '@/types';
import { createContext, useContext } from 'react';

export interface ChatState {
  currentUserId: string | null;
  conversations: Conversation[];
  isLoading: boolean;
}

export type ChatAction =
  | { type: 'SET_USER_ID'; payload: string }
  | { type: 'SET_CONVERSATIONS'; payload: Conversation[] }
  | { type: 'ADD_CONVERSATION'; payload: Conversation }
  | {
      type: 'UPDATE_CONVERSATION';
      conversationId: string;
      updates: Partial<Conversation>;
    }
  | { type: 'SET_LOADING'; payload: boolean };

export interface ChatContextType {
  state: ChatState;
  dispatch: React.Dispatch<ChatAction>;
  setUserId: (userId: string) => void;
  setConversations: (conversations: Conversation[]) => void;
  addConversation: (conversation: Conversation) => void;
  updateConversation: (
    conversationId: string,
    updates: Partial<Conversation>
  ) => void;
  setLoading: (isLoading: boolean) => void;
}

export const ChatContext = createContext<ChatContextType | undefined>(
  undefined
);

export function useChatContext() {
  const context = useContext(ChatContext);
  if (context === undefined) {
    throw new Error('useChatContext must be used within a ChatProvider');
  }
  return context;
}
