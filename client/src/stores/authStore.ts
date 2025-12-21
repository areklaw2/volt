import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { userService } from '@/services/userService';
import type { User } from '@/types';

const STORAGE_KEY = 'chat_app_user';

export const useAuthStore = defineStore('auth', () => {
  const currentUser = ref<User | null>(null);

  const isAuthenticated = computed(() => currentUser.value !== null);

  async function login(username: string): Promise<User> {
    try {
      const user = await userService.login(username);
      currentUser.value = user;
      localStorage.setItem(STORAGE_KEY, JSON.stringify(user));
      return user;
    } catch (error) {
      console.error('Login failed:', error);
      throw error;
    }
  }

  function logout(): void {
    currentUser.value = null;
    localStorage.removeItem(STORAGE_KEY);
  }

  function loadUserFromStorage(): void {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      try {
        currentUser.value = JSON.parse(stored);
      } catch (error) {
        console.error('Failed to parse stored user:', error);
        localStorage.removeItem(STORAGE_KEY);
      }
    }
  }

  return {
    currentUser,
    isAuthenticated,
    login,
    logout,
    loadUserFromStorage,
  };
});
