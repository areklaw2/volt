export interface User {
  id: string;
  username: string;
  display_name: string;
  avatar_url: string;
  created_at: string;
}

export interface Conversation {
  id: string;
  conversation_type: "direct" | "group";
  title: string | null;
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

export interface Participant {
  user_id: string;
  conversation_id: string;
  joined_at: string | null;
  last_read_at: string | null;
}

export interface ConversationWithMeta extends Conversation {
  participants: User[];
  lastMessage: Message | null;
  unreadCount: number;
}
