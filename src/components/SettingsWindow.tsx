import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';
import { 
  X, 
  User, 
  Bot, 
  MessageSquare, 
  Terminal, 
  Palette,
  Globe,
  Bell
} from 'lucide-react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import GeneralSettings from './settings/GeneralSettings';
import PetSettings from './settings/PetSettings';
import ChatSettings from './settings/ChatSettings';
import TerminalSettings from './settings/TerminalSettings';
import AppearanceSettings from './settings/AppearanceSettings';
import '../styles/SettingsWindow.css';

type TabType = 'general' | 'pet' | 'chat' | 'terminal' | 'appearance';

interface Tab {
  id: TabType;
  label: string;
  icon: React.ElementType;
}

const tabs: Tab[] = [
  { id: 'general', label: 'General', icon: User },
  { id: 'pet', label: 'Pet', icon: Bot },
  { id: 'chat', label: 'Chat', icon: MessageSquare },
  { id: 'terminal', label: 'Terminal', icon: Terminal },
  { id: 'appearance', label: 'Appearance', icon: Palette },
];

const SettingsWindow = () => {
  const [activeTab, setActiveTab] = useState<TabType>('general');

  const handleClose = async () => {
    const window = getCurrentWindow();
    await window.hide();
  };

  const renderTabContent = () => {
    switch (activeTab) {
      case 'general':
        return <GeneralSettings />;
      case 'pet':
        return <PetSettings />;
      case 'chat':
        return <ChatSettings />;
      case 'terminal':
        return <TerminalSettings />;
      case 'appearance':
        return <AppearanceSettings />;
      default:
        return null;
    }
  };

  return (
    <div className="settings-window">
      <div className="settings-header">
        <h1>Settings</h1>
        <button className="close-button" onClick={handleClose}>
          <X size={20} />
        </button>
      </div>

      <div className="settings-content">
        <div className="settings-sidebar">
          {tabs.map((tab) => (
            <button
              key={tab.id}
              className={`tab-button ${activeTab === tab.id ? 'active' : ''}`}
              onClick={() => setActiveTab(tab.id)}
            >
              <tab.icon size={18} />
              <span>{tab.label}</span>
            </button>
          ))}
        </div>

        <div className="settings-panel">
          <AnimatePresence mode="wait">
            <motion.div
              key={activeTab}
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              exit={{ opacity: 0, x: -20 }}
              transition={{ duration: 0.2 }}
            >
              {renderTabContent()}
            </motion.div>
          </AnimatePresence>
        </div>
      </div>
    </div>
  );
};

export default SettingsWindow;
