<script setup lang="ts">
import { watch } from 'vue';
import { useConversationStore } from '@/stores/conversationStore';
import { useMessageStore } from '@/stores/messageStore';
import ChatHeader from './chat/ChatHeader.vue';
import MessageList from './chat/MessageList.vue';
import MessageInput from './chat/MessageInput.vue';
import EmptyState from './common/EmptyState.vue';

const conversationStore = useConversationStore();
const messageStore = useMessageStore();

watch(
  () => conversationStore.currentConversationId,
  async (newConversationId) => {
    if (newConversationId) {
      try {
        await messageStore.fetchMessages(newConversationId);
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
    <template v-if="conversationStore.currentConversation">
      <ChatHeader :conversation="conversationStore.currentConversation" />
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
