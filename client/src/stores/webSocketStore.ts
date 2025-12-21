import { defineStore } from 'pinia';
import { ref } from 'vue';
import { env } from '@/env';
import type { Message } from '@/types';

export const useWebSocketStore = defineStore('webSocket', () => {
  const socket = ref<WebSocket | null>(null);
  const isConnected = ref(false);
  const connectionStatus = ref<'connecting' | 'connected' | 'disconnected' | 'error'>('disconnected');

  let messageHandler: ((message: Message) => void) | null = null;

  function connect(username: string): void {
    if (socket.value) {
      console.warn('WebSocket already connected');
      return;
    }

    connectionStatus.value = 'connecting';
    socket.value = new WebSocket(env.SOCKET_URL);

    socket.value.onopen = () => {
      console.log('WebSocket connected');
      socket.value?.send(username);
      isConnected.value = true;
      connectionStatus.value = 'connected';
    };

    socket.value.onmessage = (event: MessageEvent) => {
      console.log('WebSocket message received:', event.data);

      try {
        const message: Message = JSON.parse(event.data);
        if (messageHandler) {
          messageHandler(message);
        }
      } catch (error) {
        console.error('Failed to parse WebSocket message:', error);
      }
    };

    socket.value.onerror = (error) => {
      console.error('WebSocket error:', error);
      connectionStatus.value = 'error';
    };

    socket.value.onclose = () => {
      console.log('WebSocket disconnected');
      isConnected.value = false;
      connectionStatus.value = 'disconnected';
      socket.value = null;
    };
  }

  function disconnect(): void {
    if (socket.value) {
      socket.value.close(1000, 'Client disconnected');
      socket.value = null;
      isConnected.value = false;
      connectionStatus.value = 'disconnected';
    }
  }

  function sendMessage(message: Message): void {
    if (!socket.value || !isConnected.value) {
      console.error('Cannot send message: WebSocket not connected');
      return;
    }

    try {
      socket.value.send(JSON.stringify(message));
    } catch (error) {
      console.error('Failed to send message:', error);
      throw error;
    }
  }

  function setMessageHandler(handler: (message: Message) => void): void {
    messageHandler = handler;
  }

  return {
    socket,
    isConnected,
    connectionStatus,
    connect,
    disconnect,
    sendMessage,
    setMessageHandler,
  };
});
