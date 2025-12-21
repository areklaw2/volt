<script setup lang="ts">
import { watch } from 'vue';
import { useChatStore } from '@/stores/chatStore';
import { useMessageStore } from '@/stores/messageStore';
import ChatHeader from './chat/ChatHeader.vue';
import MessageList from './chat/MessageList.vue';
import MessageInput from './chat/MessageInput.vue';
import EmptyState from './common/EmptyState.vue';

const chatStore = useChatStore();
const messageStore = useMessageStore();

watch(
  () => chatStore.currentChatId,
  async (newChatId) => {
    if (newChatId) {
      try {
        await messageStore.fetchMessages(newChatId);
      } catch (error) {
        console.error('Failed to fetch messages:', error);
      }
    }
  },
  { immediate: true }
);
</script>

<template>
  <div class="flex flex-col h-full">
    <template v-if="chatStore.currentChat">
      <ChatHeader :chat="chatStore.currentChat" />
      <MessageList class="flex-1" />
      <MessageInput />
    </template>

    <EmptyState
      v-else
      icon="👋"
      title="Select a chat to start messaging"
      description="Choose a conversation from the sidebar or search for users to start a new chat"
    />
  </div>
</template>
