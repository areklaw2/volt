<script setup lang="ts">
import { ref } from 'vue';
import { useWebSocketStore } from '@/stores/webSocketStore';
import { useMessageStore } from '@/stores/messageStore';
import { Input } from '@/components/ui/input';
import { Button } from '@/components/ui/button';

const webSocketStore = useWebSocketStore();
const messageStore = useMessageStore();

const messageInput = ref('');
const isSending = ref(false);

async function handleSend() {
  if (!messageInput.value.trim() || isSending.value) return;

  isSending.value = true;
  try {
    await messageStore.sendMessage(messageInput.value);
    messageInput.value = '';
  } catch (error) {
    console.error('Failed to send message:', error);
  } finally {
    isSending.value = false;
  }
}

function handleKeydown(event: KeyboardEvent) {
  if (event.key === 'Enter' && !event.shiftKey) {
    event.preventDefault();
    handleSend();
  }
}
</script>

<template>
  <div class="p-4 border-t">
    <div class="flex gap-2">
      <Input
        v-model="messageInput"
        placeholder="Type a message..."
        :disabled="!webSocketStore.isConnected || isSending"
        @keydown="handleKeydown"
        class="flex-1"
      />
      <Button
        @click="handleSend"
        :disabled="!webSocketStore.isConnected || isSending || !messageInput.trim()"
      >
        {{ isSending ? 'Sending...' : 'Send' }}
      </Button>
    </div>
  </div>
</template>
