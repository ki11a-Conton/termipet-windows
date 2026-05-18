import { motion } from 'framer-motion';
import { LucideIcon } from 'lucide-react';
import '../styles/FloatingToolbar.css';

interface ToolbarItem {
  icon: LucideIcon;
  label: string;
  onClick: () => void;
}

interface FloatingToolbarProps {
  items: ToolbarItem[];
}

const FloatingToolbar = ({ items }: FloatingToolbarProps) => {
  return (
    <motion.div
      className="floating-toolbar"
      initial={{ opacity: 0, y: 10, scale: 0.9 }}
      animate={{ opacity: 1, y: 0, scale: 1 }}
      exit={{ opacity: 0, y: 10, scale: 0.9 }}
      transition={{ type: 'spring', stiffness: 300, damping: 25 }}
    >
      {items.map((item, index) => (
        <motion.button
          key={item.label}
          className="toolbar-button"
          onClick={item.onClick}
          initial={{ opacity: 0, scale: 0 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ delay: index * 0.05 }}
          whileHover={{ scale: 1.1, y: -2 }}
          whileTap={{ scale: 0.95 }}
          title={item.label}
        >
          <item.icon size={18} />
          <span className="toolbar-label">{item.label}</span>
        </motion.button>
      ))}
    </motion.div>
  );
};

export default FloatingToolbar;
