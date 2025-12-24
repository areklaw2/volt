import { useCallback, useMemo, useReducer, type ReactNode } from 'react';
import type { Conversation } from '@/types';
import {
  ChatContext,
  type ChatAction,
  type ChatState,
} from '@/contexts/ChatContext';

const initialState: ChatState = {
  currentUserId: null,
  conversations: [],
  isLoading: false,
};

function chatReducer(state: ChatState, action: ChatAction): ChatState {
  switch (action.type) {
    case 'SET_USER_ID':
      return { ...state, currentUserId: action.payload };
    case 'SET_CONVERSATIONS':
      return { ...state, conversations: action.payload };
    case 'ADD_CONVERSATION':
      return {
        ...state,
        conversations: [action.payload, ...state.conversations],
      };
    case 'UPDATE_CONVERSATION':
      return {
        ...state,
        conversations: state.conversations.map((conv) =>
          conv.id === action.conversationId
            ? { ...conv, ...action.updates }
            : conv
        ),
      };
    case 'SET_LOADING':
      return { ...state, isLoading: action.payload };
    default:
      return state;
  }
}

export function ChatProvider({ children }: { children: ReactNode }) {
  const [state, dispatch] = useReducer(chatReducer, initialState, (base) => ({
    ...base,
    currentUserId: localStorage.getItem('currentUserId') || null,
  }));

  const setUserId = useCallback((userId: string) => {
    dispatch({ type: 'SET_USER_ID', payload: userId });
    localStorage.setItem('currentUserId', userId);
  }, []);

  const setConversations = useCallback((conversations: Conversation[]) => {
    dispatch({ type: 'SET_CONVERSATIONS', payload: conversations });
  }, []);

  const addConversation = useCallback((conversation: Conversation) => {
    dispatch({ type: 'ADD_CONVERSATION', payload: conversation });
  }, []);

  const updateConversation = useCallback(
    (conversationId: string, updates: Partial<Conversation>) => {
      dispatch({ type: 'UPDATE_CONVERSATION', conversationId, updates });
    },
    []
  );

  const setLoading = useCallback((isLoading: boolean) => {
    dispatch({ type: 'SET_LOADING', payload: isLoading });
  }, []);

  const value = useMemo(
    () => ({
      state,
      dispatch,
      setUserId,
      setConversations,
      addConversation,
      updateConversation,
      setLoading,
    }),
    [
      state,
      setUserId,
      setConversations,
      addConversation,
      updateConversation,
      setLoading,
    ]
  );

  return <ChatContext.Provider value={value}>{children}</ChatContext.Provider>;
}
