export interface User {
  id: string;
  username: string;
  display_name: string;
  created_at: string;
}

export interface Participant {
  user_id: string;
  username: string;
  display_name: string;
  joined_at: string | null;
  last_read_at: string | null;
}

type ConversationType = 'direct' | 'group';

export interface Conversation {
  id: string;
  conversation_type: ConversationType;
  name: string | null;
  participants: Participant[];
  created_at: string;
  updated_at: string | null;
}

export interface Message {
  id: string;
  conversation_id: string;
  sender_id: string;
  content: string;
  created_at: string;
  updated_at: string | null;
}
