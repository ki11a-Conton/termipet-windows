import { useState, useEffect, useCallback } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';
import { 
  Terminal, 
  Folder, 
  MessageCircle, 
  Palette, 
  Timer, 
  Coffee,
  Settings,
  X
} from 'lucide-react';
import PetSprite from './PetSprite';
import FloatingToolbar from './FloatingToolbar';
import { usePetStore } from '../stores/petStore';
import { useTimerStore } from '../stores/timerStore';
import '../styles/PetWindow.css';

const PetWindow = () => {
  const [isToolbarVisible, setIsToolbarVisible] = useState(false);
  const [isDragging, setIsDragging] = useState(false);
  const { currentState, setState } = usePetStore();
  const { startPomodoro, stopPomodoro, isRunning } = useTimerStore();

  useEffect(() => {
    const handleMouseEnter = () => setIsToolbarVisible(true);
    const handleMouseLeave = () => {
      if (!isDragging) {
        setIsToolbarVisible(false);
      }
    };

    const container = document.querySelector('.pet-container');
    if (container) {
      container.addEventListener('mouseenter', handleMouseEnter);
      container.addEventListener('mouseleave', handleMouseLeave);
    }

    return () => {
      if (container) {
        container.removeEventListener('mouseenter', handleMouseEnter);
        container.removeEventListener('mouseleave', handleMouseLeave);
      }
    };
  }, [isDragging]);

  const handleDragStart = () => {
    setIsDragging(true);
  };

  const handleDragEnd = async () => {
    setIsDragging(false);
    // Save position
    const window = getCurrentWindow();
    const position = await window.outerPosition();
    await invoke('set_window_position', {
      label: 'pet',
      position: { x: position.x, y: position.y }
    });
  };

  const handleShowChat = async () => {
    await invoke('show_window', { label: 'chat' });
  };

  const handleShowSettings = async () => {
    await invoke('show_window', { label: 'settings' });
  };

  const handleStartTimer = async () => {
    if (isRunning) {
      await stopPomodoro();
    } else {
      await startPomodoro(25);
      setState('celebrating');
    }
  };

  const handleStartBreak = async () => {
    await startPomodoro(5);
  };

  const toolbarItems = [
    { icon: Terminal, label: 'Commands', onClick: () => {} },
    { icon: Folder, label: 'Folder', onClick: () => {} },
    { icon: MessageCircle, label: 'Chat', onClick: handleShowChat },
    { icon: Palette, label: 'Skin', onClick: () => {} },
    { icon: Timer, label: 'Timer', onClick: handleStartTimer },
    { icon: Coffee, label: 'Break', onClick: handleStartBreak },
    { icon: Settings, label: 'Settings', onClick: handleShowSettings },
  ];

  return (
    <div className="pet-window">
      <motion.div
        className="pet-container"
        drag
        dragMomentum={false}
        onDragStart={handleDragStart}
        onDragEnd={handleDragEnd}
        whileDrag={{ scale: 1.05 }}
      >
        <PetSprite state={currentState} />
        
        <AnimatePresence>
          {isToolbarVisible && (
            <FloatingToolbar items={toolbarItems} />
          )}
        </AnimatePresence>
      </motion.div>
    </div>
  );
};

export default PetWindow;
