export interface Message {
  id: string;
  conversation_id: string;
  sender_id: string;
  content: string;
  type: 'text' | 'image';
  createdAt: string;
  updatedAt: string;
}
