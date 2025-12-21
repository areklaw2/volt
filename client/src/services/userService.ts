import { apiClient } from './api';
import type { User } from '@/types';

export const userService = {
  async searchUsers(query: string): Promise<User[]> {
    return apiClient.get<User[]>(`/users/search?q=${encodeURIComponent(query)}`);
  },

  async getUser(userId: string): Promise<User> {
    return apiClient.get<User>(`/users/${userId}`);
  },

  async login(username: string): Promise<User> {
    return apiClient.post<User>('/users/login', { username });
  },
};
