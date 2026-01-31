import type { User, Conversation } from '@/types';
import { env } from '@/lib/env';

const BASE = `${env.API_URL}/api/v1`;

let _getToken: (() => Promise<string | null>) | null = null;

export function initApi(getToken: () => Promise<string | null>) {
  _getToken = getToken;
}

async function sendRequest(path: string, init?: RequestInit): Promise<Response> {
  if (!_getToken) throw new Error('API not initialized. Call initApi() first.');

  const token = await _getToken();
  if (!token) throw new Error('Unable to get auth token');

  return fetch(`${BASE}${path}`, {
    ...init,
    headers: {
      ...init?.headers,
      Authorization: `Bearer ${token}`,
    },
  });
}

export async function createUser(id: string, username: string, displayName: string): Promise<void> {
  await sendRequest('/user', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ id, username, display_name: displayName }),
  });
}

export async function fetchUsers(currentUserId: string): Promise<User[]> {
  const res = await sendRequest('/users');
  const data: User[] = await res.json();
  return data.filter((u) => u.id !== currentUserId);
}

export async function createConversation(params: {
  conversation_type: 'direct' | 'group';
  sender_id: string;
  participants: string[];
  name: string | null;
}): Promise<Conversation> {
  const res = await sendRequest('/conversation', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  if (!res.ok) throw new Error('Failed to create conversation');
  return res.json();
}

export async function fetchConversations(userId: string): Promise<Conversation[]> {
  const res = await sendRequest(`/conversations/${userId}`);
  return res.json();
}
