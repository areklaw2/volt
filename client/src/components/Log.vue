<script setup lang="ts">
import type { LogEntry } from '@/types/log';

defineProps<{
  message: string;
  type: LogEntry['type'];
  timestamp: string;
}>();
</script>

<template>
  <div class="log-entry" :class="`log-${type}`">
    <span class="log-timestamp">[{{ timestamp }}]</span> {{ message }}
  </div>
</template>

<style scoped>
.log-entry {
  margin-bottom: 5px;
}

.log-timestamp {
  color: #888;
}

.log-connect {
  color: #00ff00;
}

.log-disconnect {
  color: #ff0000;
}

.log-message {
  color: #0000ff;
}

.log-error {
  color: #ff0000;
}
</style>
