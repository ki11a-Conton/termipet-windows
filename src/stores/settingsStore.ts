import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface ModelConfig {
  provider: 'ollama' | 'openai' | 'gemini' | 'custom';
  modelName: string;
  baseUrl?: string;
}

interface Personality {
  preset: string;
  customPrompt?: string;
  additionalConstraints?: string;
}

interface CommandShortcut {
  id: string;
  name: string;
  command: string;
  pinned: boolean;
  order: number;
}

interface AppSettings {
  language: string;
  skin: 'glass' | 'dark' | 'pixel' | 'light';
  petId: string;
  petName: string;
  ownerName: string;
  personality: Personality;
  modelConfig: ModelConfig;
  shortcuts: CommandShortcut[];
  pomodoroDuration: number;
  breakDuration: number;
  autoStart: boolean;
  showOnStartup: boolean;
}

interface SettingsStore {
  settings: AppSettings;
  isLoading: boolean;
  loadSettings: () => Promise<void>;
  updateSettings: (settings: AppSettings) => Promise<void>;
}

const defaultSettings: AppSettings = {
  language: 'zh-CN',
  skin: 'glass',
  petId: 'terminal-cat',
  petName: 'Terminal Cat',
  ownerName: 'Master',
  personality: {
    preset: 'friendly',
  },
  modelConfig: {
    provider: 'ollama',
    modelName: 'qwen2.5:1.5b',
    baseUrl: 'http://localhost:11434',
  },
  shortcuts: [],
  pomodoroDuration: 25,
  breakDuration: 5,
  autoStart: false,
  showOnStartup: true,
};

export const useSettingsStore = create<SettingsStore>((set, get) => ({
  settings: defaultSettings,
  isLoading: false,
  
  loadSettings: async () => {
    set({ isLoading: true });
    try {
      const settings = await invoke<AppSettings>('get_settings');
      set({ settings: { ...defaultSettings, ...settings }, isLoading: false });
    } catch (error) {
      console.error('Failed to load settings:', error);
      set({ isLoading: false });
    }
  },
  
  updateSettings: async (settings: AppSettings) => {
    set({ isLoading: true });
    try {
      await invoke('save_settings', { settings });
      set({ settings, isLoading: false });
    } catch (error) {
      console.error('Failed to save settings:', error);
      set({ isLoading: false });
    }
  },
}));
