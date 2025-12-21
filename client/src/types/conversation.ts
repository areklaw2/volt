import type { Message } from './message';
import type { User } from './user';

export interface CreateConversationResponse {
  id: string;
  type: 'direct' | 'group';
  title?: string;
  participants: User[];
  lastMessage: Message;
  created_at: string;
  updated_at: string;
}

export interface CreateConversationRequest {
  conversation_type: 'direct' | 'group';
  message_type: 'text' | 'image';
  first_message: string;
  sender_id: string;
  participants: string[];
  title?: string;
}

export interface QueryConversationResponse {
  items: ConversationItems;
}

export interface ConversationItems {
  conversation_id: string;
  title?: string;
  participants: string;
  last_message: LastMessage;
  unread_count: never;
  updated_at: string;
}

export interface LastMessage {
  message_id: string;
  sender_id: string;
  content: string;
  created_at: String;
}
