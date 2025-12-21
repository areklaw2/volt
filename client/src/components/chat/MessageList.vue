<script setup lang="ts">
import { ref, watch, nextTick } from 'vue';
import { useMessageStore } from '@/stores/messageStore';
import { ScrollArea } from '@/components/ui/scroll-area';
import MessageBubble from './MessageBubble.vue';
import EmptyState from '../common/EmptyState.vue';

const messageStore = useMessageStore();
const scrollAreaRef = ref<HTMLElement | null>(null);

async function scrollToBottom() {
  await nextTick();
  if (scrollAreaRef.value) {
    const scrollContainer = scrollAreaRef.value.querySelector('[data-radix-scroll-area-viewport]');
    if (scrollContainer) {
      scrollContainer.scrollTop = scrollContainer.scrollHeight;
    }
  }
}

watch(
  () => messageStore.currentMessages.length,
  () => {
    scrollToBottom();
  }
);
</script>

<template>
  <ScrollArea ref="scrollAreaRef" class="flex-1 p-4">
    <template v-if="messageStore.currentMessages.length > 0">
      <div class="space-y-4">
        <MessageBubble
          v-for="message in messageStore.currentMessages"
          :key="message.id"
          :message="message"
        />
      </div>
    </template>

    <EmptyState
      v-else
      icon="✉️"
      title="No messages yet"
      description="Start the conversation by sending a message below"
    />
  </ScrollArea>
</template>
