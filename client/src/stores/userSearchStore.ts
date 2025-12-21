import { defineStore } from 'pinia';
import { ref } from 'vue';
import { userService } from '@/services/userService';
import type { User } from '@/types';

export const useUserSearchStore = defineStore('userSearch', () => {
  const searchQuery = ref('');
  const searchResults = ref<User[]>([]);
  const isSearching = ref(false);

  let debounceTimeout: ReturnType<typeof setTimeout> | null = null;

  async function searchUsers(query: string): Promise<void> {
    searchQuery.value = query;

    if (debounceTimeout) {
      clearTimeout(debounceTimeout);
    }

    if (!query.trim()) {
      searchResults.value = [];
      return;
    }

    debounceTimeout = setTimeout(async () => {
      isSearching.value = true;
      try {
        searchResults.value = await userService.searchUsers(query);
      } catch (error) {
        console.error('Failed to search users:', error);
        searchResults.value = [];
      } finally {
        isSearching.value = false;
      }
    }, 300);
  }

  function clearSearch(): void {
    searchQuery.value = '';
    searchResults.value = [];
    if (debounceTimeout) {
      clearTimeout(debounceTimeout);
    }
  }

  return {
    searchQuery,
    searchResults,
    isSearching,
    searchUsers,
    clearSearch,
  };
});
