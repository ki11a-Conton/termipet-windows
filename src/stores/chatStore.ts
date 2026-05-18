import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface Message {
  id: string;
  role: 'user' | 'assistant';
  content: string;
  timestamp: Date;
}

interface ChatStore {
  messages: Message[];
  isLoading: boolean;
  addMessage: (message: Message) => void;
  clearMessages: () => void;
  loadHistory: () => Promise<void>;
}

export const useChatStore = create<ChatStore>((set, get) => ({
  messages: [],
  isLoading: false,
  
  addMessage: (message: Message) => {
    set((state) => ({
      messages: [...state.messages, message],
    }));
  },
  
  clearMessages: () => {
    set({ messages: [] });
  },
  
  loadHistory: async () => {
    try {
      const history = await invoke<Message[]>('get_chat_history');
      set({ messages: history });
    } catch (error) {
      console.error('Failed to load chat history:', error);
    }
  },
}));
