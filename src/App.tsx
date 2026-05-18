import { useEffect, useState } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import PetWindow from './components/PetWindow';
import ChatWindow from './components/ChatWindow';
import SettingsWindow from './components/SettingsWindow';
import { useAppStore } from './stores/appStore';
import './styles/App.css';

function App() {
  const [windowLabel, setWindowLabel] = useState<string>('');
  const { init } = useAppStore();

  useEffect(() => {
    const initWindow = async () => {
      const currentWindow = getCurrentWindow();
      const label = currentWindow.label;
      setWindowLabel(label);
      
      // Initialize app data
      await init();
    };
    
    initWindow();
  }, [init]);

  // Render different content based on window label
  const renderContent = () => {
    switch (windowLabel) {
      case 'pet':
        return <PetWindow />;
      case 'chat':
        return <ChatWindow />;
      case 'settings':
        return <SettingsWindow />;
      default:
        return <div>Loading...</div>;
    }
  };

  return (
    <div className="app">
      {renderContent()}
    </div>
  );
}

export default App;
