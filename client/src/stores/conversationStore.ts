import { defineStore } from 'pinia';
import { ref, computed } from 'vue';
import { chatService } from '@/services/chatService';
import type { Conversation, CreateConversationRequest, Message } from '@/types';

export const useConversationStore = defineStore('chat', () => {
  const conversations = ref<Conversation[]>([]);
  const currentConversationId = ref<string | null>(null);
  const isLoading = ref(false);

  const currentConversation = computed(
    () =>
      conversations.value.find(
        (conversation) => conversation.id === currentConversationId.value
      ) || null
  );

  const sortedConversations = computed(() =>
    [...conversations.value].sort((a, b) => {
      const aTime = a.last_message?.created_at || a.updated_at;
      const bTime = b.last_message?.created_at || b.updated_at;
      return new Date(bTime).getTime() - new Date(aTime).getTime();
    })
  );

  async function fetchConversations(userId: string): Promise<void> {
    isLoading.value = true;
    try {
      let response = await chatService.queryUserConversations(userId);
      conversations.value = response.items;
    } catch (error) {
      console.error('Failed to fetch chats:', error);
      throw error;
    } finally {
      isLoading.value = false;
    }
  }

  function selectConversation(conversationId: string): void {
    currentConversationId.value = conversationId;
  }

  async function createConversation(
    request: CreateConversationRequest
  ): Promise<Conversation> {
    try {
      const response = await chatService.createConversation(request);
      conversations.value.push(response);
      currentConversationId.value = response.id;
      return response;
    } catch (error) {
      console.error('Failed to create chat:', error);
      throw error;
    }
  }

  function updateConversationLastMessage(
    conversationId: string,
    message: Message
  ): void {
    const conversation = conversations.value.find(
      (c) => c.id === conversationId
    );
    if (conversation) {
      conversation.last_message = message;
      conversation.updated_at = message.created_at;
    }

    //TODO: send an update with socket
  }

  return {
    conversations,
    currentConversationId,
    currentConversation,
    sortedConversations,
    isLoading,
    fetchConversations,
    selectConversation,
    createConversation,
    updateConversationLastMessage,
  };
});
