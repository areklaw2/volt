import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { chatService } from '@/services/chatService';
import { useWebSocketStore } from './webSocketStore';
import { useConversationStore } from './conversationStore';
import { useAuthStore } from './authStore';
import type { Message } from '@/types';

export const useMessageStore = defineStore('message', () => {
  const messagesByConversationId = ref<Record<string, Message[]>>({});
  const isLoading = ref(false);

  const conversationStore = useConversationStore();

  const currentMessages = computed(() => {
    if (!conversationStore.currentConversationId) return [];
    return (
      messagesByConversationId.value[conversationStore.currentConversationId] ||
      []
    );
  });

  async function fetchMessages(conversationId: string): Promise<void> {
    isLoading.value = true;
    try {
      const messages = await chatService.getMessages(conversationId);
      messagesByConversationId.value[conversationId] = messages;
    } catch (error) {
      console.error('Failed to fetch messages:', error);
      throw error;
    } finally {
      isLoading.value = false;
    }
  }

  function addMessage(message: Message): void {
    const { conversation_id } = message;

    if (!messagesByConversationId.value[conversation_id]) {
      messagesByConversationId.value[conversation_id] = [];
    }

    messagesByConversationId.value[conversation_id].push(message);
    conversationStore.updateConversationLastMessage(conversation_id, message);
  }

  async function sendMessage(content: string): Promise<void> {
    const authStore = useAuthStore();
    const webSocketStore = useWebSocketStore();

    if (!conversationStore.currentConversationId) {
      console.error('No conversation selected');
      return;
    }

    if (!authStore.currentUser) {
      console.error('No user logged in');
      return;
    }

    //TODO: check if i should be doing this here?
    const message: Message = {
      id: crypto.randomUUID(),
      conversation_id: conversationStore.currentConversationId,
      sender_id: authStore.currentUser.id,
      content,
      type: 'text',
      created_at: new Date().toISOString(),
    };

    addMessage(message);

    try {
      webSocketStore.sendMessage(message);
    } catch (error) {
      console.error('Failed to send message:', error);
    }
  }

  function clearMessages(chatId: string): void {
    delete messagesByConversationId.value[chatId];
  }

  function initializeWebSocketHandler(): void {
    const webSocketStore = useWebSocketStore();
    webSocketStore.setMessageHandler((message: Message) => {
      addMessage(message);
    });
  }

  return {
    messagesByChatId: messagesByConversationId,
    currentMessages,
    isLoading,
    fetchMessages,
    addMessage,
    sendMessage,
    clearMessages,
    initializeWebSocketHandler,
  };
});
