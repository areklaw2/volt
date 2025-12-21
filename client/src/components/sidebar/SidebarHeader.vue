<script setup lang="ts">
import { useAuthStore } from '@/stores/authStore';
import Avatar from '../common/Avatar.vue';
import { Separator } from '@/components/ui/separator';

const authStore = useAuthStore();
</script>

<template>
  <div class="p-4">
    <div v-if="authStore.currentUser" class="flex items-center gap-3">
      <Avatar :user="authStore.currentUser" size="md" />
      <div class="flex-1 min-w-0">
        <h2 class="font-semibold truncate">{{ authStore.currentUser.username }}</h2>
        <p class="text-xs text-muted-foreground">
          {{ authStore.currentUser.status || 'online' }}
        </p>
      </div>
    </div>
    <Separator class="mt-4" />
  </div>
</template>
