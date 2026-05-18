import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface TimerState {
  active: boolean;
  mode: 'pomodoro' | 'break';
  remainingSeconds: number;
  totalSeconds: number;
}

interface TimerStore {
  isRunning: boolean;
  mode: 'pomodoro' | 'break' | null;
  remainingSeconds: number;
  totalSeconds: number;
  startPomodoro: (minutes: number) => Promise<void>;
  stopPomodoro: () => Promise<void>;
  getState: () => Promise<void>;
}

export const useTimerStore = create<TimerStore>((set, get) => ({
  isRunning: false,
  mode: null,
  remainingSeconds: 0,
  totalSeconds: 0,
  
  startPomodoro: async (minutes: number) => {
    try {
      const state = await invoke<TimerState>('start_pomodoro', {
        durationMinutes: minutes,
      });
      set({
        isRunning: state.active,
        mode: state.mode,
        remainingSeconds: state.remainingSeconds,
        totalSeconds: state.totalSeconds,
      });
    } catch (error) {
      console.error('Failed to start pomodoro:', error);
    }
  },
  
  stopPomodoro: async () => {
    try {
      await invoke('stop_pomodoro');
      set({
        isRunning: false,
        mode: null,
        remainingSeconds: 0,
        totalSeconds: 0,
      });
    } catch (error) {
      console.error('Failed to stop pomodoro:', error);
    }
  },
  
  getState: async () => {
    try {
      const state = await invoke<TimerState>('get_timer_state');
      set({
        isRunning: state.active,
        mode: state.mode,
        remainingSeconds: state.remainingSeconds,
        totalSeconds: state.totalSeconds,
      });
    } catch (error) {
      console.error('Failed to get timer state:', error);
    }
  },
}));
