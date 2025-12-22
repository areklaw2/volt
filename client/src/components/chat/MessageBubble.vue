<script setup lang="ts">
import { computed } from 'vue';
import { useAuthStore } from '@/stores/authStore';
import { Card, CardContent } from '@/components/ui/card';
import type { Message } from '@/types';

const props = defineProps<{
  message: Message;
}>();

const authStore = useAuthStore();

const isOwnMessage = computed(() => {
  return props.message.sender_id === authStore.currentUser?.id;
});

const formattedTime = computed(() => {
  const date = new Date(props.message.created_at);
  return date.toLocaleTimeString('en-US', {
    hour: '2-digit',
    minute: '2-digit',
  });
});
</script>

<template>
  <div class="flex" :class="isOwnMessage ? 'justify-end' : 'justify-start'">
    <div class="max-w-[70%]">
      <Card
        :class="
          isOwnMessage ? 'bg-primary text-primary-foreground' : 'bg-muted'
        "
      >
        <CardContent class="p-3">
          <p class="text-sm whitespace-pre-wrap wrap-break-word">
            {{ message.content }}
          </p>
          <div class="flex items-center justify-end gap-2 mt-1">
            <span class="text-xs opacity-70">{{ formattedTime }}</span>
            <span v-if="isOwnMessage" class="text-xs opacity-70">
              <!--TODO: May need to add message status to v-if at some point {{ message.status === 'sending' ? '⏳' : '✓' }} -->
            </span>
          </div>
        </CardContent>
      </Card>
    </div>
  </div>
</template>
