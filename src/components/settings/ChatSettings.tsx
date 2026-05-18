import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Check, AlertCircle, RefreshCw } from 'lucide-react';
import { useSettingsStore } from '../../stores/settingsStore';
import '../../styles/settings/ChatSettings.css';

interface LocalModel {
  name: string;
  size: string;
  description: string;
  downloaded: boolean;
}

const ChatSettings = () => {
  const { settings, updateSettings } = useSettingsStore();
  const [activeTab, setActiveTab] = useState<'local' | 'online'>('local');
  const [localModels, setLocalModels] = useState<LocalModel[]>([]);
  const [isTesting, setIsTesting] = useState(false);
  const [testResult, setTestResult] = useState<boolean | null>(null);
  const [apiKey, setApiKey] = useState('');

  useEffect(() => {
    if (activeTab === 'local') {
      loadLocalModels();
    }
  }, [activeTab]);

  const loadLocalModels = async () => {
    try {
      const models = await invoke<LocalModel[]>('get_available_models');
      setLocalModels(models);
    } catch (error) {
      console.error('Failed to load local models:', error);
    }
  };

  const handleTestConnection = async () => {
    setIsTesting(true);
    setTestResult(null);
    try {
      const result = await invoke<boolean>('test_model_connection', {
        modelConfig: settings.modelConfig,
      });
      setTestResult(result);
    } catch (error) {
      setTestResult(false);
    } finally {
      setIsTesting(false);
    }
  };

  const handleSaveApiKey = async () => {
    try {
      await invoke('set_api_key', {
        provider: settings.modelConfig.provider,
        apiKey: apiKey || null,
      });
      setApiKey('');
    } catch (error) {
      console.error('Failed to save API key:', error);
    }
  };

  const recommendedModels = [
    { name: 'qwen2.5:0.5b', description: 'Ultra lightweight, good for low-end PCs', size: '~400MB' },
    { name: 'qwen2.5:1.5b', description: 'Recommended, good Chinese support', size: '~1.1GB' },
    { name: 'phi3.5:latest', description: 'Small size, high quality', size: '~2.2GB' },
    { name: 'gemma3:1b', description: 'Balanced and lightweight', size: '~815MB' },
  ];

  return (
    <div className="chat-settings">
      <h2>Chat Settings</h2>

      <div className="tabs">
        <button
          className={`tab ${activeTab === 'local' ? 'active' : ''}`}
          onClick={() => setActiveTab('local')}
        >
          Local Model (Ollama)
        </button>
        <button
          className={`tab ${activeTab === 'online' ? 'active' : ''}`}
          onClick={() => setActiveTab('online')}
        >
          Online API
        </button>
      </div>

      {activeTab === 'local' && (
        <div className="tab-content">
          <section className="settings-section">
            <div className="section-header">
              <h3>Ollama Status</h3>
              <button className="refresh-button" onClick={loadLocalModels}>
                <RefreshCw size={16} />
              </button>
            </div>

            {localModels.length === 0 ? (
              <div className="ollama-not-running">
                <AlertCircle size={24} />
                <p>Ollama is not running or no models installed.</p>
                <a
                  href="https://ollama.com/download"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="download-link"
                >
                  Download Ollama
                </a>
              </div>
            ) : (
              <div className="models-list">
                <h4>Installed Models</h4>
                {localModels.map((model) => (
                  <div
                    key={model.name}
                    className={`model-item ${
                      settings.modelConfig.modelName === model.name ? 'selected' : ''
                    }`}
                    onClick={() =>
                      updateSettings({
                        ...settings,
                        modelConfig: {
                          ...settings.modelConfig,
                          provider: 'ollama',
                          modelName: model.name,
                        },
                      })
                    }
                  >
                    <div className="model-info">
                      <span className="model-name">{model.name}</span>
                      <span className="model-size">{model.size}</span>
                    </div>
                    {settings.modelConfig.modelName === model.name && (
                      <Check size={16} />
                    )}
                  </div>
                ))}
              </div>
            )}
          </section>

          <section className="settings-section">
            <h3>Recommended Models</h3>
            <div className="recommended-models">
              {recommendedModels.map((model) => (
                <div key={model.name} className="recommended-model">
                  <div className="model-header">
                    <span className="model-name">{model.name}</span>
                    <span className="model-size">{model.size}</span>
                  </div>
                  <p className="model-description">{model.description}</p>
                </div>
              ))}
            </div>
          </section>
        </div>
      )}

      {activeTab === 'online' && (
        <div className="tab-content">
          <section className="settings-section">
            <h3>API Provider</h3>
            
            <div className="form-group">
              <label>Provider</label>
              <select
                value={settings.modelConfig.provider}
                onChange={(e) =>
                  updateSettings({
                    ...settings,
                    modelConfig: {
                      ...settings.modelConfig,
                      provider: e.target.value as 'openai' | 'gemini' | 'custom',
                    },
                  })
                }
              >
                <option value="openai">OpenAI</option>
                <option value="gemini">Google Gemini</option>
                <option value="custom">Custom API</option>
              </select>
            </div>

            {(settings.modelConfig.provider === 'custom' ||
              settings.modelConfig.provider === 'openai') && (
              <div className="form-group">
                <label>Base URL</label>
                <input
                  type="text"
                  value={settings.modelConfig.baseUrl || ''}
                  onChange={(e) =>
                    updateSettings({
                      ...settings,
                      modelConfig: {
                        ...settings.modelConfig,
                        baseUrl: e.target.value,
                      },
                    })
                  }
                  placeholder={
                    settings.modelConfig.provider === 'openai'
                      ? 'https://api.openai.com/v1'
                      : 'https://api.example.com/v1'
                  }
                />
              </div>
            )}

            <div className="form-group">
              <label>Model Name</label>
              <input
                type="text"
                value={settings.modelConfig.modelName}
                onChange={(e) =>
                  updateSettings({
                    ...settings,
                    modelConfig: {
                      ...settings.modelConfig,
                      modelName: e.target.value,
                    },
                  })
                }
                placeholder="gpt-3.5-turbo"
              />
            </div>

            <div className="form-group">
              <label>API Key</label>
              <div className="api-key-input">
                <input
                  type="password"
                  value={apiKey}
                  onChange={(e) => setApiKey(e.target.value)}
                  placeholder="Enter API key..."
                />
                <button onClick={handleSaveApiKey}>Save</button>
              </div>
              <p className="help-text">
                API key is securely stored in Windows Credential Manager.
              </p>
            </div>

            <div className="form-actions">
              <button
                className="test-button"
                onClick={handleTestConnection}
                disabled={isTesting}
              >
                {isTesting ? 'Testing...' : 'Test Connection'}
              </button>
              {testResult !== null && (
                <span className={`test-result ${testResult ? 'success' : 'error'}`}>
                  {testResult ? 'Connection successful!' : 'Connection failed'}
                </span>
              )}
            </div>
          </section>
        </div>
      )}
    </div>
  );
};

export default ChatSettings;
