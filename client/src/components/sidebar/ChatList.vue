<script setup lang="ts">
import { useConversationStore } from '@/stores/conversationStore';
import { ScrollArea } from '@/components/ui/scroll-area';
import ChatListItem from './ChatListItem.vue';
import EmptyState from '../common/EmptyState.vue';

const chatStore = useConversationStore();
</script>

<template>
  <div class="flex-1 overflow-hidden">
    <ScrollArea v-if="chatStore.sortedChats.length > 0" class="h-full">
      <div class="p-2 space-y-1">
        <ChatListItem
          v-for="chat in chatStore.sortedChats"
          :key="chat.id"
          :chat="chat"
          :is-active="chat.id === chatStore.currentChatId"
          @click="chatStore.selectChat(chat.id)"
        />
      </div>
    </ScrollArea>

    <EmptyState
      v-else
      icon="💬"
      title="No chats yet"
      description="Search for users above to start a conversation"
    />
  </div>
</template>
