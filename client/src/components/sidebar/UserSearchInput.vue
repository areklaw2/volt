<script setup lang="ts">
import { ref } from 'vue';
import { useUserSearchStore } from '@/stores/userSearchStore';
import { useConversationStore } from '@/stores/conversationStore';
import { Input } from '@/components/ui/input';
import { ScrollArea } from '@/components/ui/scroll-area';
import Avatar from '../common/Avatar.vue';
import type { User } from '@/types';

const userSearchStore = useUserSearchStore();
const chatStore = useConversationStore();

const searchInput = ref('');

function handleSearch(event: Event) {
  const value = (event.target as HTMLInputElement).value;
  searchInput.value = value;
  userSearchStore.searchUsers(value);
}

async function handleUserSelect(user: User) {
  try {
    await chatStore.createChat(user.id);
    searchInput.value = '';
    userSearchStore.clearSearch();
  } catch (error) {
    console.error('Failed to create chat:', error);
  }
}
</script>

<template>
  <div class="px-4 pb-2">
    <Input
      v-model="searchInput"
      placeholder="Search users..."
      @input="handleSearch"
      class="w-full"
    />

    <!-- Search Results Dropdown -->
    <div
      v-if="
        userSearchStore.searchQuery && userSearchStore.searchResults.length > 0
      "
      class="mt-2 border rounded-md bg-background"
    >
      <ScrollArea class="max-h-64">
        <div class="p-1">
          <button
            v-for="user in userSearchStore.searchResults"
            :key="user.id"
            @click="handleUserSelect(user)"
            class="w-full flex items-center gap-3 p-2 rounded-md hover:bg-accent transition-colors"
          >
            <Avatar :user="user" size="sm" />
            <div class="flex-1 text-left">
              <p class="text-sm font-medium">{{ user.username }}</p>
              <p class="text-xs text-muted-foreground">Start a chat</p>
            </div>
          </button>
        </div>
      </ScrollArea>
    </div>

    <!-- No Results -->
    <div
      v-else-if="userSearchStore.searchQuery && !userSearchStore.isSearching"
      class="mt-2 p-3 text-center text-sm text-muted-foreground"
    >
      No users found
    </div>

    <!-- Loading -->
    <div
      v-else-if="userSearchStore.isSearching"
      class="mt-2 p-3 text-center text-sm text-muted-foreground"
    >
      Searching...
    </div>
  </div>
</template>
