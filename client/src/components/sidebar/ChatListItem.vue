<script setup lang="ts">
import { computed } from 'vue';
import { useAuthStore } from '@/stores/authStore';
import Avatar from '../common/Avatar.vue';
import { Badge } from '@/components/ui/badge';
import type { Chat } from '@/types';

const props = defineProps<{
  chat: Chat;
  isActive: boolean;
}>();

const authStore = useAuthStore();

const otherParticipant = computed(() => {
  return props.chat.participants.find(
    (p) => p.id !== authStore.currentUser?.id
  );
});

const lastMessagePreview = computed(() => {
  if (!props.chat.lastMessage) return 'No messages yet';
  return props.chat.lastMessage.body.length > 50
    ? props.chat.lastMessage.body.slice(0, 50) + '...'
    : props.chat.lastMessage.body;
});

const formattedTime = computed(() => {
  if (!props.chat.lastMessage) return '';

  const date = new Date(props.chat.lastMessage.timestamp);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffMins = Math.floor(diffMs / 60000);

  if (diffMins < 1) return 'Just now';
  if (diffMins < 60) return `${diffMins}m ago`;

  const diffHours = Math.floor(diffMins / 60);
  if (diffHours < 24) return `${diffHours}h ago`;

  const diffDays = Math.floor(diffHours / 24);
  return `${diffDays}d ago`;
});
</script>

<template>
  <button
    class="w-full flex items-center gap-3 p-3 rounded-lg transition-colors"
    :class="
      isActive
        ? 'bg-accent'
        : 'hover:bg-accent/50'
    "
  >
    <Avatar v-if="otherParticipant" :user="otherParticipant" size="md" />

    <div class="flex-1 min-w-0 text-left">
      <div class="flex items-center justify-between mb-1">
        <h3 class="font-semibold truncate">
          {{ otherParticipant?.username || 'Unknown' }}
        </h3>
        <span v-if="formattedTime" class="text-xs text-muted-foreground">
          {{ formattedTime }}
        </span>
      </div>
      <p class="text-sm text-muted-foreground truncate">
        {{ lastMessagePreview }}
      </p>
    </div>

    <Badge v-if="chat.unreadCount > 0" variant="default" class="ml-auto">
      {{ chat.unreadCount }}
    </Badge>
  </button>
</template>
