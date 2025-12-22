import type { Message } from './message';

export interface CreateConversationRequest {
  conversation_type: 'direct' | 'group';
  message_type: 'text' | 'image';
  first_message: string;
  sender_id: string;
  participants: string[];
  title?: string;
}

export interface QueryConversationResponse {
  items: Conversation[];
}

export interface Conversation {
  id: string;
  type: 'direct' | 'group';
  title?: string;
  participants: string[];
  last_message: Message;
  unread_message_count: number;
  created_at: string;
  updated_at: string;
}
