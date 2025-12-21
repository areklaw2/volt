<script setup lang="ts">
import { onMounted, onUnmounted, ref, nextTick } from 'vue';
import { env } from '@/env';
import Log from './Log.vue';
import type { LogEntry } from '@/types/log';
import type { Message } from '@/types/message';

const socket = ref<WebSocket | null>(null);
const isConnected = ref(false);
const logs = ref<LogEntry[]>([]);
const logsContainer = ref<HTMLDivElement | null>(null);

const serverUrl = ref('http://localhost:3000');
const echoInput = ref('');
const messageInput = ref('');

let logIdCounter = 0;

defineProps<{ msg: string }>();

function close(reason: number, message: string) {
  if (socket.value) {
    socket.value.close(reason, message);
    socket.value = null;
  }
}

function connect() {
  addLog(`Connected to ${serverUrl.value}`, 'connect');
  socket.value = new WebSocket(env.SOCKET_URL);

  socket.value.onopen = () => {
    addLog('Connected to server!', 'connect');
    isConnected.value = true;
  };

  socket.value.onmessage = (event) => {
    addLog(`Received: ${event.data}`, 'message');
  };

  socket.value.onerror = (error) => {
    addLog('WebSocket error occurred', 'error');
    console.log(error);
  };

  socket.value.onclose = () => {
    isConnected.value = false;
    addLog('Disconnected from server', 'disconnect');
  };
}

function disconnect() {
  close(1000, 'Client disconnected');
}

function sendEcho() {
  if (!echoInput.value) {
    return;
  }

  const echo: Message = {
    type: 'echo',
    body: echoInput.value,
  };
  socket.value?.send(JSON.stringify(echo));
}

function getStatus() {
  const status: Message = {
    type: 'status',
  };
  socket.value?.send(JSON.stringify(status));
}

function sendMessage() {
  if (!messageInput.value) {
    return;
  }

  const message: Message = {
    type: 'chat',
    body: messageInput.value,
    chat_id: 'chat',
  };
  socket.value?.send(JSON.stringify(message));
}

function addLog(message: string, type: LogEntry['type'] = 'info') {
  const timestamp = new Date().toLocaleTimeString();
  logs.value.push({
    id: logIdCounter++,
    message,
    type,
    timestamp,
  });

  nextTick(() => {
    if (logsContainer.value) {
      logsContainer.value.scrollTop = logsContainer.value.scrollHeight;
    }
  });
}

onMounted(() => {
  addLog('Client loaded. Enter server URL and click Connect.', 'info');
});

onUnmounted(() => {
  close(1001, 'Client closed page');
});
</script>

<template>
  <div class="container">
    <div class="header">
      <h1>Chat Test Client</h1>
    </div>

    <div class="status">
      <div class="status-indicator">
        <div class="status-dot" :class="{ connected: isConnected }"></div>
        <span id="statusText">{{
          !isConnected ? 'Disconnected' : 'Connected'
        }}</span>
      </div>
      <div>
        <span id="clientId">No Client ID</span> |
        <span id="clientCount">0 clients</span>
      </div>
    </div>

    <div class="main-content">
      <div class="panel">
        <h3>Connection Controls</h3>
        <div class="control-group">
          <label for="serverUrl">Server URL</label>
          <input type="text" v-model="serverUrl" />
        </div>
        <div class="button-group">
          <button class="btn-success" @click="connect" :disabled="isConnected">
            Connect
          </button>
          <button
            class="btn-danger"
            @click="disconnect"
            :disabled="!isConnected"
          >
            Disconnect
          </button>
        </div>
      </div>

      <div class="panel">
        <h3>Quick Actions</h3>
        <div class="button-group">
          <button
            class="btn-info"
            @click="getStatus"
            :disabled="!isConnected"
            style="width: 100%"
          >
            Get Status
          </button>
        </div>
      </div>

      <div class="panel">
        <h3>Echo</h3>
        <div class="control-group">
          <label for="echoInput">Echo Message</label>
          <input
            type="text"
            v-model="echoInput"
            placeholder="Type something to echo..."
          />
        </div>
        <button
          class="btn-primary"
          @click="sendEcho"
          :disabled="!isConnected"
          style="width: 100%"
        >
          Send Echo
        </button>
      </div>

      <div class="panel">
        <h3>Send Message</h3>
        <div class="control-group">
          <label for="messageInput">Message Content</label>
          <textarea
            v-model="messageInput"
            placeholder="Type your message..."
          ></textarea>
        </div>
        <button
          class="btn-primary"
          @click="sendMessage"
          :disabled="!isConnected"
          style="width: 100%"
        >
          Send Message
        </button>
      </div>

      <div class="panel log-container">
        <h3>Event Log</h3>
        <div class="logs" ref="logsContainer">
          <Log
            v-for="log in logs"
            :key="log.id"
            :message="log.message"
            :type="log.type"
            :timestamp="log.timestamp"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.container {
  max-width: 900px;
  margin: 0 auto;
  background: white;
  border-radius: 10px;
  box-shadow: 0 10px 40px rgba(0, 0, 0, 0.2);
  overflow: hidden;
}

.header {
  background: #667eea;
  color: white;
  padding: 20px;
  text-align: center;
}

.header h1 {
  margin-bottom: 5px;
}

.header p {
  opacity: 0.9;
  font-size: 14px;
}

.status {
  padding: 15px 20px;
  background: #f8f9fa;
  border-bottom: 1px solid #dee2e6;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.status-indicator {
  display: flex;
  align-items: center;
  gap: 10px;
}

.status-dot {
  width: 12px;
  height: 12px;
  border-radius: 50%;
  background: #dc3545;
  animation: pulse 2s infinite;
}

.status-dot.connected {
  background: #28a745;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.main-content {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  padding: 20px;
}

.panel {
  background: #f8f9fa;
  border-radius: 8px;
  padding: 15px;
}

.panel h3 {
  margin-bottom: 15px;
  color: #667eea;
  font-size: 16px;
}

.control-group {
  margin-bottom: 15px;
}

.control-group label {
  display: block;
  margin-bottom: 5px;
  font-weight: 500;
  font-size: 14px;
}

input[type='text'],
textarea {
  width: 100%;
  padding: 10px;
  border: 1px solid #ced4da;
  border-radius: 5px;
  font-size: 14px;
}

textarea {
  resize: vertical;
  min-height: 80px;
}

.button-group {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 10px;
}

button {
  padding: 10px 20px;
  border: none;
  border-radius: 5px;
  font-size: 14px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s;
}

button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-primary {
  background: #667eea;
  color: white;
}

.btn-primary:hover:not(:disabled) {
  background: #5568d3;
}

.btn-success {
  background: #28a745;
  color: white;
}

.btn-success:hover:not(:disabled) {
  background: #218838;
}

.btn-danger {
  background: #dc3545;
  color: white;
}

.btn-danger:hover:not(:disabled) {
  background: #c82333;
}

.btn-info {
  background: #17a2b8;
  color: white;
}

.btn-info:hover:not(:disabled) {
  background: #138496;
}

.log-container {
  grid-column: 1 / -1;
}

.logs {
  background: #212529;
  color: #00ff00;
  padding: 15px;
  border-radius: 5px;
  max-height: 300px;
  overflow-y: auto;
  font-family: 'Courier New', monospace;
  font-size: 12px;
  line-height: 1.5;
}
</style>
