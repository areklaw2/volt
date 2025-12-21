<script setup lang="ts">
import { computed } from 'vue';
import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import type { User } from '@/types';

const props = withDefaults(
  defineProps<{
    user: User;
    size?: 'sm' | 'md' | 'lg';
  }>(),
  {
    size: 'md',
  }
);

const sizeClasses = computed(() => {
  switch (props.size) {
    case 'sm':
      return 'w-8 h-8 text-xs';
    case 'lg':
      return 'w-12 h-12 text-lg';
    default:
      return 'w-10 h-10 text-sm';
  }
});

const initials = computed(() => {
  return props.user.username
    .split(' ')
    .map((n) => n[0])
    .join('')
    .toUpperCase()
    .slice(0, 2);
});
</script>

<template>
  <Avatar :class="sizeClasses">
    <AvatarImage v-if="user.avatar" :src="user.avatar" :alt="user.username" />
    <AvatarFallback>{{ initials }}</AvatarFallback>
  </Avatar>
</template>
