<script setup lang="ts">
import { computed } from 'vue';
import { useUserSearchStore } from '@/stores/userSearchStore';
import SidebarHeader from './sidebar/SidebarHeader.vue';
import UserSearchInput from './sidebar/UserSearchInput.vue';
import ChatList from './sidebar/ChatList.vue';

const userSearchStore = useUserSearchStore();

const showSearchResults = computed(() => userSearchStore.searchQuery.trim().length > 0);
</script>

<template>
  <div class="flex flex-col h-full bg-background">
    <SidebarHeader />
    <UserSearchInput />
    <ChatList v-if="!showSearchResults" />
    <div v-else class="flex-1 overflow-hidden">
      <!-- Search results will be shown by UserSearchInput -->
    </div>
  </div>
</template>
