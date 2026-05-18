import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Upload, Trash2, Check } from 'lucide-react';
import { useSettingsStore } from '../../stores/settingsStore';
import '../../styles/settings/PetSettings.css';

interface Pet {
  id: string;
  display_name: string;
  description: string;
  spritesheet_path: string;
}

const PetSettings = () => {
  const { settings, updateSettings } = useSettingsStore();
  const [pets, setPets] = useState<Pet[]>([]);
  const [selectedPet, setSelectedPet] = useState(settings.petId);
  const [isImporting, setIsImporting] = useState(false);

  useEffect(() => {
    loadPets();
  }, []);

  const loadPets = async () => {
    try {
      const availablePets = await invoke<Pet[]>('get_available_pets');
      setPets(availablePets);
    } catch (error) {
      console.error('Failed to load pets:', error);
    }
  };

  const handleSelectPet = async (petId: string) => {
    setSelectedPet(petId);
    try {
      await invoke('load_pet', { petId });
      updateSettings({ ...settings, petId });
    } catch (error) {
      console.error('Failed to load pet:', error);
    }
  };

  const handleImportPet = async () => {
    // Open file dialog to select pet folder
    // This would use Tauri's dialog API
    setIsImporting(true);
    try {
      // await invoke('import_pet', { path: selectedPath });
      await loadPets();
    } catch (error) {
      console.error('Failed to import pet:', error);
    } finally {
      setIsImporting(false);
    }
  };

  return (
    <div className="pet-settings">
      <h2>Pet Settings</h2>

      <section className="settings-section">
        <div className="section-header">
          <h3>Available Pets</h3>
          <button 
            className="import-button"
            onClick={handleImportPet}
            disabled={isImporting}
          >
            <Upload size={16} />
            Import Pet
          </button>
        </div>

        <div className="pets-grid">
          {pets.map((pet) => (
            <div
              key={pet.id}
              className={`pet-card ${selectedPet === pet.id ? 'selected' : ''}`}
              onClick={() => handleSelectPet(pet.id)}
            >
              <div className="pet-preview">
                {/* Pet preview image would go here */}
                <span className="pet-emoji">🐱</span>
                {selectedPet === pet.id && (
                  <div className="selected-indicator">
                    <Check size={16} />
                  </div>
                )}
              </div>
              <div className="pet-info">
                <h4>{pet.display_name}</h4>
                <p>{pet.description}</p>
              </div>
            </div>
          ))}
        </div>
      </section>

      <section className="settings-section">
        <h3>Personality</h3>
        
        <div className="form-group">
          <label htmlFor="personalityPreset">Personality Preset</label>
          <select
            id="personalityPreset"
            value={settings.personality.preset}
            onChange={(e) =>
              updateSettings({
                ...settings,
                personality: { ...settings.personality, preset: e.target.value },
              })
            }
          >
            <option value="friendly">Friendly</option>
            <option value="professional">Professional</option>
            <option value="playful">Playful</option>
            <option value="calm">Calm</option>
            <option value="custom">Custom</option>
          </select>
        </div>

        {settings.personality.preset === 'custom' && (
          <div className="form-group">
            <label htmlFor="customPrompt">Custom Prompt</label>
            <textarea
              id="customPrompt"
              rows={4}
              value={settings.personality.customPrompt || ''}
              onChange={(e) =>
                updateSettings({
                  ...settings,
                  personality: {
                    ...settings.personality,
                    customPrompt: e.target.value,
                  },
                })
              }
              placeholder="Enter custom personality prompt..."
            />
          </div>
        )}

        <div className="form-group">
          <label htmlFor="constraints">Additional Constraints</label>
          <textarea
            id="constraints"
            rows={3}
            value={settings.personality.additionalConstraints || ''}
            onChange={(e) =>
              updateSettings({
                ...settings,
                personality: {
                  ...settings.personality,
                  additionalConstraints: e.target.value,
                },
              })
            }
            placeholder="Enter additional constraints for the pet..."
          />
        </div>
      </section>
    </div>
  );
};

export default PetSettings;
