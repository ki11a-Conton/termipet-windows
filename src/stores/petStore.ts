import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';

interface Pet {
  id: string;
  display_name: string;
  description: string;
  spritesheet_path: string;
}

type PetState = 'idle' | 'running' | 'moving' | 'happy' | 'alert' | 'error' | 'sleeping' | 'thinking' | 'celebrating';

interface PetStore {
  currentPet: Pet | null;
  currentState: PetState;
  availablePets: Pet[];
  loadPet: (petId: string) => Promise<void>;
  setState: (state: PetState) => Promise<void>;
  loadAvailablePets: () => Promise<void>;
}

export const usePetStore = create<PetStore>((set, get) => ({
  currentPet: null,
  currentState: 'idle',
  availablePets: [],
  
  loadPet: async (petId: string) => {
    try {
      const pet = await invoke<Pet>('load_pet', { petId });
      set({ currentPet: pet });
    } catch (error) {
      console.error('Failed to load pet:', error);
    }
  },
  
  setState: async (state: PetState) => {
    try {
      await invoke('set_pet_state', { state });
      set({ currentState: state });
    } catch (error) {
      console.error('Failed to set pet state:', error);
      set({ currentState: state });
    }
  },
  
  loadAvailablePets: async () => {
    try {
      const pets = await invoke<Pet[]>('get_available_pets');
      set({ availablePets: pets });
    } catch (error) {
      console.error('Failed to load available pets:', error);
    }
  },
}));
