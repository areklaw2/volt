import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { chatService } from '@/services/chatService';
import { useWebSocketStore } from './webSocketStore';
import { useChatStore } from './chatStore';
import { useAuthStore } from './authStore';
import type { Message } from '@/types';

export const useMessageStore = defineStore('message', () => {
  const messagesByChatId = ref<Record<string, Message[]>>({});
  const isLoading = ref(false);

  const chatStore = useChatStore();

  const currentMessages = computed(() => {
    if (!chatStore.currentChatId) return [];
    return messagesByChatId.value[chatStore.currentChatId] || [];
  });

  async function fetchMessages(chatId: string): Promise<void> {
    isLoading.value = true;
    try {
      const messages = await chatService.getMessages(chatId);
      messagesByChatId.value[chatId] = messages;
    } catch (error) {
      console.error('Failed to fetch messages:', error);
      throw error;
    } finally {
      isLoading.value = false;
    }
  }

  function addMessage(message: Message): void {
    const { chat_id } = message;

    if (!messagesByChatId.value[chat_id]) {
      messagesByChatId.value[chat_id] = [];
    }

    messagesByChatId.value[chat_id].push(message);
    chatStore.updateChatLastMessage(chat_id, message);
  }

  async function sendMessage(body: string): Promise<void> {
    const authStore = useAuthStore();
    const webSocketStore = useWebSocketStore();

    if (!chatStore.currentChatId) {
      console.error('No chat selected');
      return;
    }

    if (!authStore.currentUser) {
      console.error('No user logged in');
      return;
    }

    const message: Message = {
      id: crypto.randomUUID(),
      chat_id: chatStore.currentChatId,
      sender_id: authStore.currentUser.id,
      body,
      timestamp: new Date().toISOString(),
      status: 'sending',
    };

    addMessage(message);

    try {
      webSocketStore.sendMessage(message);
      message.status = 'sent';
    } catch (error) {
      console.error('Failed to send message:', error);
      message.status = 'sent';
    }
  }

  function clearMessages(chatId: string): void {
    delete messagesByChatId.value[chatId];
  }

  function initializeWebSocketHandler(): void {
    const webSocketStore = useWebSocketStore();
    webSocketStore.setMessageHandler((message: Message) => {
      addMessage(message);
    });
  }

  return {
    messagesByChatId,
    currentMessages,
    isLoading,
    fetchMessages,
    addMessage,
    sendMessage,
    clearMessages,
    initializeWebSocketHandler,
  };
});
