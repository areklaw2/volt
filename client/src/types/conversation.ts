import type { Message } from './message';
import type { User } from './user';

export interface Conversation {
  id: string;
  type: 'direct' | 'group';
  title?: string;
  participants: User[];
  lastMessage: Message;
  createdAt: string;
  updatedAt: string;
}

export interface CreateConversationRequest {
  conversation_type: 'direct' | 'group';
  message_type: 'text' | 'image';
  first_message: string;
  sender_id: string;
  participants: string[];
  title?: string;
}
