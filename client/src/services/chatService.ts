import { apiClient } from './api';
import type {
  CreateConversationRequest,
  CreateConversationResponse,
  Message,
  QueryConversationResponse,
} from '@/types';

export const chatService = {
  async createConversation(
    request: CreateConversationRequest
  ): Promise<CreateConversationResponse> {
    return apiClient.post<CreateConversationResponse>(
      '/v1/conversation',
      request
    );
  },

  async queryUserConversations(
    userId: string
  ): Promise<QueryConversationResponse> {
    return apiClient.get<QueryConversationResponse>(
      `/v1/conversations/${userId}`
    );
  },

  async getMessages(conversationId: string): Promise<Message[]> {
    return apiClient.get<Message[]>(`/v1/messages/${conversationId}`);
  },
};
