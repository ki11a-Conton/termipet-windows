import { useSettingsStore } from '../../stores/settingsStore';
import '../../styles/settings/AppearanceSettings.css';

const skins = [
  { id: 'glass', name: 'Glass', description: 'Modern glass effect', preview: 'linear-gradient(135deg, rgba(255,255,255,0.1), rgba(255,255,255,0.05))' },
  { id: 'dark', name: 'Dark', description: 'Dark theme', preview: '#1a1a2e' },
  { id: 'pixel', name: 'Pixel', description: 'Retro pixel style', preview: '#2d2d2d' },
  { id: 'light', name: 'Light', description: 'Clean light theme', preview: '#f5f5f5' },
];

const languages = [
  { code: 'zh-CN', name: '简体中文', flag: 'CN' },
  { code: 'zh-TW', name: '繁體中文', flag: 'TW' },
  { code: 'en', name: 'English', flag: 'US' },
  { code: 'ja', name: '日本語', flag: 'JP' },
  { code: 'ko', name: '한국어', flag: 'KR' },
];

const AppearanceSettings = () => {
  const { settings, updateSettings } = useSettingsStore();

  return (
    <div className="appearance-settings">
      <h2>Appearance Settings</h2>

      <section className="settings-section">
        <h3>Theme</h3>
        <div className="skins-grid">
          {skins.map((skin) => (
            <div
              key={skin.id}
              className={`skin-card ${settings.skin === skin.id ? 'selected' : ''}`}
              onClick={() => updateSettings({ ...settings, skin: skin.id as any })}
            >
              <div
                className="skin-preview"
                style={{ background: skin.preview }}
              />
              <div className="skin-info">
                <span className="skin-name">{skin.name}</span>
                <span className="skin-description">{skin.description}</span>
              </div>
            </div>
          ))}
        </div>
      </section>

      <section className="settings-section">
        <h3>Language</h3>
        <div className="languages-list">
          {languages.map((lang) => (
            <div
              key={lang.code}
              className={`language-item ${settings.language === lang.code ? 'selected' : ''}`}
              onClick={() => updateSettings({ ...settings, language: lang.code })}
            >
              <span className="language-flag">{lang.flag}</span>
              <span className="language-name">{lang.name}</span>
            </div>
          ))}
        </div>
        <p className="help-text">Restart required for language changes to take full effect.</p>
      </section>
    </div>
  );
};

export default AppearanceSettings;
