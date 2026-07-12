import type { User, Conversation, Message } from '@/types';
import { env } from '@/lib/env';

const API_BASE = `${env.API_URL}/api/v1`;

async function sendHttpRequest(path: string, init?: RequestInit): Promise<Response> {
  return fetch(`${API_BASE}${path}`, init);
}

export async function createUser(id: string, username: string, displayName: string): Promise<void> {
  await sendHttpRequest('/user', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ id, username, display_name: displayName }),
  });
}

export async function fetchUsers(currentUserId: string): Promise<User[]> {
  const res = await sendHttpRequest('/users');
  const data: User[] = await res.json();
  return data.filter((u) => u.id !== currentUserId);
}

export async function createConversation(params: {
  conversation_type: 'direct' | 'group';
  sender_id: string;
  participants: string[];
  name: string | null;
}): Promise<Conversation> {
  const res = await sendHttpRequest('/conversation', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(params),
  });
  if (!res.ok) throw new Error('Failed to create conversation');
  return res.json();
}

export async function fetchConversations(userId: string): Promise<Conversation[]> {
  const res = await sendHttpRequest(`/conversations/${userId}`);
  return res.json();
}

export async function markAsRead(conversationId: string, userId: string): Promise<void> {
  await sendHttpRequest(`/conversation/${conversationId}/read/${userId}`, {
    method: 'POST',
  });
}

export async function leaveConversation(conversationId: string, userId: string): Promise<void> {
  await sendHttpRequest(`/conversation/${conversationId}/leave/${userId}`, {
    method: 'POST',
  });
}

export async function fetchMessages(
  conversationId: string,
  params?: { offset?: number; limit?: number },
): Promise<Message[]> {
  const query = new URLSearchParams();

  if (params?.offset != null) {
    query.set('offset', String(params.offset));
  }

  if (params?.limit != null) {
    query.set('limit', String(params.limit));
  }

  const queryString = query.toString();
  const res = await sendHttpRequest(`/messages/${conversationId}${queryString ? `?${queryString}` : ''}`);
  return res.json();
}

export async function editMessage(messageId: string, editorId: string, content: string): Promise<void> {
  await sendHttpRequest(`/messages/${messageId}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ editor_id: editorId, content }),
  });
}

export async function uploadImage(file: File): Promise<{ url: string }> {
  const form = new FormData();
  form.append('file', file);
  const res = await sendHttpRequest('/upload', { method: 'POST', body: form });
  if (!res.ok) throw new Error('Failed to upload image');
  return res.json();
}
