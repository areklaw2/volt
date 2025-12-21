<script setup lang="ts">
import { computed } from 'vue';
import { useAuthStore } from '@/stores/authStore';
import { useWebSocketStore } from '@/stores/webSocketStore';
import Avatar from '../common/Avatar.vue';
import { Separator } from '@/components/ui/separator';
import type { Chat } from '@/types';

const props = defineProps<{
  chat: Chat;
}>();

const authStore = useAuthStore();
const webSocketStore = useWebSocketStore();

const otherParticipant = computed(() => {
  return props.chat.participants.find(
    (p) => p.id !== authStore.currentUser?.id
  );
});

const connectionStatusText = computed(() => {
  switch (webSocketStore.connectionStatus) {
    case 'connected':
      return 'Connected';
    case 'connecting':
      return 'Connecting...';
    case 'error':
      return 'Connection error';
    default:
      return 'Disconnected';
  }
});

const connectionStatusColor = computed(() => {
  return webSocketStore.isConnected ? 'bg-green-500' : 'bg-red-500';
});
</script>

<template>
  <div>
    <div class="p-4 flex items-center gap-3">
      <Avatar v-if="otherParticipant" :user="otherParticipant" size="md" />

      <div class="flex-1">
        <h2 class="font-semibold">
          {{ otherParticipant?.username || 'Unknown' }}
        </h2>
        <div class="flex items-center gap-2 text-xs text-muted-foreground">
          <div class="flex items-center gap-1">
            <div :class="['w-2 h-2 rounded-full', connectionStatusColor]" />
            <span>{{ connectionStatusText }}</span>
          </div>
        </div>
      </div>
    </div>
    <Separator />
  </div>
</template>
