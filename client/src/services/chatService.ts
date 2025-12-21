import { apiClient } from './api';
import type { Conversation, CreateConversationRequest, Message } from '@/types';

export const chatService = {
  async createConversation(
    request: CreateConversationRequest
  ): Promise<Conversation> {
    return apiClient.post<Conversation>('/v1/conversation', request);
  },

  async getChats(userId: string): Promise<Conversation[]> {
    return apiClient.get<Conversation[]>(`/v1/conversations/${userId}`);
  },

  async getMessages(conversationId: string): Promise<Message[]> {
    return apiClient.get<Message[]>(`/v1/messages/${conversationId}`);
  },
};
