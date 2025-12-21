import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { chatService } from '@/services/chatService';
import type { Chat, Message } from '@/types';

export const useChatStore = defineStore('chat', () => {
  const chats = ref<Chat[]>([]);
  const currentChatId = ref<string | null>(null);
  const isLoading = ref(false);

  const currentChat = computed(
    () => chats.value.find((chat) => chat.id === currentChatId.value) || null
  );

  const sortedChats = computed(() =>
    [...chats.value].sort((a, b) => {
      const aTime = a.lastMessage?.timestamp || a.updatedAt;
      const bTime = b.lastMessage?.timestamp || b.updatedAt;
      return new Date(bTime).getTime() - new Date(aTime).getTime();
    })
  );

  async function fetchChats(): Promise<void> {
    isLoading.value = true;
    try {
      chats.value = await chatService.getChats();
    } catch (error) {
      console.error('Failed to fetch chats:', error);
      throw error;
    } finally {
      isLoading.value = false;
    }
  }

  function selectChat(chatId: string): void {
    currentChatId.value = chatId;
  }

  async function createChat(userId: string): Promise<Chat> {
    try {
      const newChat = await chatService.createConversation(userId);
      chats.value.push(newChat);
      currentChatId.value = newChat.id;
      return newChat;
    } catch (error) {
      console.error('Failed to create chat:', error);
      throw error;
    }
  }

  function updateChatLastMessage(chatId: string, message: Message): void {
    const chat = chats.value.find((c) => c.id === chatId);
    if (chat) {
      chat.lastMessage = message;
      chat.updatedAt = message.timestamp;
    }
  }

  return {
    chats,
    currentChatId,
    currentChat,
    sortedChats,
    isLoading,
    fetchChats,
    selectChat,
    createChat,
    updateChatLastMessage,
  };
});
