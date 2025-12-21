export interface Message {
  type: 'echo' | 'status' | 'chat';
  body?: string;
  chat_id?: string;
}
