export interface User {
  id: string;
  username: string;
  avatar?: string;
  status?: 'online' | 'offline' | 'away';
}
