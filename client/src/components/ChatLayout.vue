<script setup lang="ts">
import { onMounted } from 'vue';
import { useAuthStore } from '@/stores/authStore';
import { useWebSocketStore } from '@/stores/webSocketStore';
import { useConversationStore } from '@/stores/conversationStore';
import { useMessageStore } from '@/stores/messageStore';
import ChatSidebar from './ChatSidebar.vue';
import ChatWindow from './ChatWindow.vue';

const authStore = useAuthStore();
const webSocketStore = useWebSocketStore();
const conversationStore = useConversationStore();
const messageStore = useMessageStore();

onMounted(async () => {
  authStore.loadUserFromStorage();

  if (authStore.currentUser) {
    webSocketStore.connect(authStore.currentUser.username);
    messageStore.initializeWebSocketHandler();

    try {
      await conversationStore.fetchConversations(authStore.currentUser?.id);
    } catch (error) {
      console.error('Failed to fetch chats:', error);
    }
  }
});
</script>

<template>
  <div class="flex h-screen bg-background">
    <ChatSidebar class="w-80 border-r" />
    <ChatWindow class="flex-1" />
  </div>
</template>
