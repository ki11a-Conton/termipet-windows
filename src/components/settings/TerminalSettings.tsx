import { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { Plus, Trash2, GripVertical } from 'lucide-react';
import { useSettingsStore } from '../../stores/settingsStore';
import '../../styles/settings/TerminalSettings.css';

interface TerminalInfo {
  id: string;
  name: string;
  process_name: string;
  window_title: string;
  current_directory: string | null;
  is_active: boolean;
}

const defaultCommands = [
  { id: '1', name: 'claude', command: 'claude', pinned: true },
  { id: '2', name: 'claude --enable-auto-mode', command: 'claude --enable-auto-mode', pinned: false },
  { id: '3', name: '/compact', command: '/compact', pinned: true },
  { id: '4', name: '/clear', command: '/clear', pinned: false },
  { id: '5', name: '/status', command: '/status', pinned: false },
  { id: '6', name: '/diff', command: '/diff', pinned: false },
];

const TerminalSettings = () => {
  const { settings, updateSettings } = useSettingsStore();
  const [terminals, setTerminals] = useState<TerminalInfo[]>([]);
  const [activeTerminal, setActiveTerminal] = useState<TerminalInfo | null>(null);
  const [newCommand, setNewCommand] = useState({ name: '', command: '' });
  const [showAddForm, setShowAddForm] = useState(false);

  useEffect(() => {
    loadTerminals();
    const interval = setInterval(loadTerminals, 5000);
    return () => clearInterval(interval);
  }, []);

  const loadTerminals = async () => {
    try {
      const list = await invoke<TerminalInfo[]>('get_terminal_list');
      setTerminals(list);
      
      const active = await invoke<TerminalInfo | null>('get_active_terminal');
      setActiveTerminal(active);
    } catch (error) {
      console.error('Failed to load terminals:', error);
    }
  };

  const handleAddCommand = () => {
    if (!newCommand.name.trim() || !newCommand.command.trim()) return;

    const command = {
      id: Date.now().toString(),
      name: newCommand.name,
      command: newCommand.command,
      pinned: false,
      order: settings.shortcuts.length,
    };

    updateSettings({
      ...settings,
      shortcuts: [...settings.shortcuts, command],
    });

    setNewCommand({ name: '', command: '' });
    setShowAddForm(false);
  };

  const handleDeleteCommand = (id: string) => {
    updateSettings({
      ...settings,
      shortcuts: settings.shortcuts.filter((cmd) => cmd.id !== id),
    });
  };

  const handleTogglePin = (id: string) => {
    updateSettings({
      ...settings,
      shortcuts: settings.shortcuts.map((cmd) =>
        cmd.id === id ? { ...cmd, pinned: !cmd.pinned } : cmd
      ),
    });
  };

  const commands = settings.shortcuts.length > 0 ? settings.shortcuts : defaultCommands;

  return (
    <div className="terminal-settings">
      <h2>Terminal Settings</h2>

      <section className="settings-section">
        <h3>Active Terminal</h3>
        {activeTerminal ? (
          <div className="active-terminal">
            <div className="terminal-info">
              <span className="terminal-name">{activeTerminal.name}</span>
              <span className="terminal-title">{activeTerminal.window_title}</span>
            </div>
            <span className="status-badge active">Active</span>
          </div>
        ) : (
          <div className="no-terminal">
            <p>No active terminal detected</p>
            <p className="help-text">
              Open PowerShell, CMD, or Windows Terminal to enable terminal features.
            </p>
          </div>
        )}
      </section>

      <section className="settings-section">
        <h3>Detected Terminals</h3>
        {terminals.length > 0 ? (
          <div className="terminals-list">
            {terminals.map((terminal) => (
              <div
                key={terminal.id}
                className={`terminal-item ${terminal.is_active ? 'active' : ''}`}
              >
                <span className="terminal-name">{terminal.name}</span>
                <span className="terminal-process">{terminal.process_name}</span>
              </div>
            ))}
          </div>
        ) : (
          <p className="no-terminals">No terminals detected</p>
        )}
      </section>

      <section className="settings-section">
        <div className="section-header">
          <h3>Quick Commands</h3>
          <button
            className="add-button"
            onClick={() => setShowAddForm(!showAddForm)}
          >
            <Plus size={16} />
            Add Command
          </button>
        </div>

        {showAddForm && (
          <div className="add-command-form">
            <div className="form-row">
              <input
                type="text"
                placeholder="Command name"
                value={newCommand.name}
                onChange={(e) =>
                  setNewCommand({ ...newCommand, name: e.target.value })
                }
              />
              <input
                type="text"
                placeholder="Command"
                value={newCommand.command}
                onChange={(e) =>
                  setNewCommand({ ...newCommand, command: e.target.value })
                }
              />
              <button onClick={handleAddCommand}>Add</button>
            </div>
          </div>
        )}

        <div className="commands-list">
          {commands
            .sort((a, b) => (b.pinned ? 1 : 0) - (a.pinned ? 1 : 0))
            .map((command) => (
              <div key={command.id} className="command-item">
                <GripVertical size={16} className="drag-handle" />
                <button
                  className={`pin-button ${command.pinned ? 'pinned' : ''}`}
                  onClick={() => handleTogglePin(command.id)}
                >
                  {command.pinned ? '★' : '☆'}
                </button>
                <div className="command-info">
                  <span className="command-name">{command.name}</span>
                  <code className="command-text">{command.command}</code>
                </div>
                <button
                  className="delete-button"
                  onClick={() => handleDeleteCommand(command.id)}
                >
                  <Trash2 size={14} />
                </button>
              </div>
            ))}
        </div>
      </section>

      <section className="settings-section">
        <h3>Accessibility Permission</h3>
        <p className="help-text">
          Terminal integration requires accessibility permissions to read window titles
          and send commands.
        </p>
        <button
          className="permission-button"
          onClick={() => invoke('request_accessibility_permission')}
        >
          Request Permission
        </button>
      </section>
    </div>
  );
};

export default TerminalSettings;
