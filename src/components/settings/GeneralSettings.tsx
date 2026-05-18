import { useState } from 'react';
import { useSettingsStore } from '../../stores/settingsStore';
import '../../styles/settings/GeneralSettings.css';

const GeneralSettings = () => {
  const { settings, updateSettings } = useSettingsStore();
  const [formData, setFormData] = useState({
    petName: settings.petName,
    ownerName: settings.ownerName,
    autoStart: settings.autoStart,
    showOnStartup: settings.showOnStartup,
  });

  const handleChange = (field: string, value: string | boolean) => {
    setFormData((prev) => ({ ...prev, [field]: value }));
  };

  const handleSave = () => {
    updateSettings({
      ...settings,
      ...formData,
    });
  };

  return (
    <div className="general-settings">
      <h2>General Settings</h2>

      <section className="settings-section">
        <h3>Pet Information</h3>
        
        <div className="form-group">
          <label htmlFor="petName">Pet Name</label>
          <input
            type="text"
            id="petName"
            value={formData.petName}
            onChange={(e) => handleChange('petName', e.target.value)}
            placeholder="Enter pet name"
          />
        </div>

        <div className="form-group">
          <label htmlFor="ownerName">Your Name</label>
          <input
            type="text"
            id="ownerName"
            value={formData.ownerName}
            onChange={(e) => handleChange('ownerName', e.target.value)}
            placeholder="Enter your name"
          />
        </div>
      </section>

      <section className="settings-section">
        <h3>Startup</h3>
        
        <div className="form-group checkbox">
          <label>
            <input
              type="checkbox"
              checked={formData.autoStart}
              onChange={(e) => handleChange('autoStart', e.target.checked)}
            />
            <span>Start automatically on login</span>
          </label>
        </div>

        <div className="form-group checkbox">
          <label>
            <input
              type="checkbox"
              checked={formData.showOnStartup}
              onChange={(e) => handleChange('showOnStartup', e.target.checked)}
            />
            <span>Show pet on startup</span>
          </label>
        </div>
      </section>

      <section className="settings-section">
        <h3>Pomodoro Timer</h3>
        
        <div className="form-row">
          <div className="form-group">
            <label htmlFor="pomodoroDuration">Focus Duration (minutes)</label>
            <input
              type="number"
              id="pomodoroDuration"
              min={1}
              max={60}
              value={settings.pomodoroDuration}
              onChange={(e) =>
                updateSettings({ ...settings, pomodoroDuration: parseInt(e.target.value) })
              }
            />
          </div>

          <div className="form-group">
            <label htmlFor="breakDuration">Break Duration (minutes)</label>
            <input
              type="number"
              id="breakDuration"
              min={1}
              max={30}
              value={settings.breakDuration}
              onChange={(e) =>
                updateSettings({ ...settings, breakDuration: parseInt(e.target.value) })
              }
            />
          </div>
        </div>
      </section>

      <div className="settings-actions">
        <button className="save-button" onClick={handleSave}>
          Save Changes
        </button>
      </div>
    </div>
  );
};

export default GeneralSettings;
