<script setup lang="ts">
defineProps<{
  icon?: string;
  title: string;
  description?: string;
}>();
</script>

<template>
  <div class="flex flex-col items-center justify-center h-full p-8 text-center">
    <div v-if="icon" class="text-6xl mb-4">{{ icon }}</div>
    <h3 class="text-lg font-semibold mb-2">{{ title }}</h3>
    <p v-if="description" class="text-sm text-muted-foreground max-w-sm">
      {{ description }}
    </p>
  </div>
</template>
