export interface Message {
  id: string;
  conversation_id: string;
  sender_id: string;
  content: string;
  type: 'text' | 'image';
  created_at: string;
  updated_at?: string;
}
