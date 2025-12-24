import type { User } from '@/types';

export const MOCK_USERS: User[] = [
  { id: 'user-1', username: 'Alice Johnson', status: 'online' },
  { id: 'user-2', username: 'Bob Smith', status: 'offline' },
  { id: 'user-3', username: 'Carol Williams', status: 'online' },
  { id: 'user-4', username: 'David Brown', status: 'away' },
  { id: 'user-5', username: 'Emma Davis', status: 'online' },
  { id: 'user-6', username: 'Frank Miller', status: 'offline' },
  { id: 'user-7', username: 'Grace Wilson', status: 'online' },
  { id: 'user-8', username: 'Henry Moore', status: 'away' },
  { id: 'user-9', username: 'Ivy Taylor', status: 'online' },
  { id: 'user-10', username: 'Jack Anderson', status: 'offline' },
  { id: 'user-11', username: 'Kate Thomas', status: 'online' },
  { id: 'user-12', username: 'Leo Jackson', status: 'away' },
  { id: 'user-13', username: 'Mia White', status: 'online' },
  { id: 'user-14', username: 'Noah Harris', status: 'offline' },
  { id: 'user-15', username: 'Olivia Martin', status: 'online' },
];

export function searchMockUsers(query: string): User[] {
  if (!query.trim()) {
    return MOCK_USERS;
  }

  const lowercaseQuery = query.toLowerCase();
  return MOCK_USERS.filter((user) =>
    user.username.toLowerCase().includes(lowercaseQuery)
  );
}
