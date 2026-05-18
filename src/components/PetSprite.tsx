import { useEffect, useState, useRef } from 'react';
import { motion } from 'framer-motion';
import '../styles/PetSprite.css';

interface PetSpriteProps {
  state: string;
}

// Animation configuration for different states
const ANIMATION_CONFIG: Record<string, { row: number; frames: number; duration: number }> = {
  idle: { row: 0, frames: 4, duration: 1 },
  running: { row: 1, frames: 6, duration: 0.6 },
  moving: { row: 2, frames: 4, duration: 0.6 },
  happy: { row: 3, frames: 4, duration: 0.8 },
  alert: { row: 4, frames: 2, duration: 1 },
  error: { row: 5, frames: 4, duration: 0.8 },
  sleeping: { row: 6, frames: 4, duration: 2 },
  thinking: { row: 7, frames: 4, duration: 1.2 },
  celebrating: { row: 8, frames: 6, duration: 0.9 },
};

const PetSprite = ({ state }: PetSpriteProps) => {
  const [currentFrame, setCurrentFrame] = useState(0);
  const animationRef = useRef<number | null>(null);
  const config = ANIMATION_CONFIG[state] || ANIMATION_CONFIG.idle;

  useEffect(() => {
    const animate = () => {
      setCurrentFrame((prev) => (prev + 1) % config.frames);
      animationRef.current = window.setTimeout(
        animate,
        (config.duration * 1000) / config.frames
      );
    };

    animate();

    return () => {
      if (animationRef.current) {
        clearTimeout(animationRef.current);
      }
    };
  }, [state, config]);

  // Calculate background position for sprite sheet
  const frameWidth = 64; // Assuming 64x64 sprites
  const frameHeight = 64;
  const bgX = -currentFrame * frameWidth;
  const bgY = -config.row * frameHeight;

  return (
    <motion.div
      className="pet-sprite"
      initial={{ scale: 0.8, opacity: 0 }}
      animate={{ scale: 1, opacity: 1 }}
      transition={{ type: 'spring', stiffness: 260, damping: 20 }}
    >
      <div
        className="sprite-frame"
        style={{
          width: frameWidth,
          height: frameHeight,
          backgroundImage: 'url(/pets/terminal-cat/spritesheet.webp)',
          backgroundPosition: `${bgX}px ${bgY}px`,
          backgroundSize: `${frameWidth * config.frames}px ${frameHeight * 9}`,
        }}
      />
      
      {/* Fallback emoji display if sprite not loaded */}
      <div className="pet-emoji-fallback">
        {state === 'idle' && '🐱'}
        {state === 'running' && '🏃'}
        {state === 'moving' && '🚶'}
        {state === 'happy' && '😊'}
        {state === 'alert' && '👀'}
        {state === 'error' && '😵'}
        {state === 'sleeping' && '😴'}
        {state === 'thinking' && '🤔'}
        {state === 'celebrating' && '🎉'}
      </div>
    </motion.div>
  );
};

export default PetSprite;
