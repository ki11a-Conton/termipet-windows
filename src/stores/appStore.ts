import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface AppState {
  isInitialized: boolean;
  systemInfo: {
    osVersion: string;
    appVersion: string;
    accessibilityEnabled: boolean;
  } | null;
  init: () => Promise<void>;
}

export const useAppStore = create<AppState>((set) => ({
  isInitialized: false,
  systemInfo: null,
  init: async () => {
    try {
      const info = await invoke<{
        os_version: string;
        app_version: string;
        accessibility_enabled: boolean;
      }>('get_system_info');
      
      set({
        isInitialized: true,
        systemInfo: {
          osVersion: info.os_version,
          appVersion: info.app_version,
          accessibilityEnabled: info.accessibility_enabled,
        },
      });
    } catch (error) {
      console.error('Failed to initialize app:', error);
      set({ isInitialized: true });
    }
  },
}));
