# TermiPet Windows Porting Guide

This document explains the key differences between the macOS version and the Windows version of TermiPet.

## Architecture Overview

### macOS Version (Original)
- **Language**: Swift 6
- **UI Framework**: SwiftUI
- **Build System**: Swift Package Manager
- **Platform APIs**: AppKit, Foundation, Security

### Windows Version (This Port)
- **Frontend**: React 18 + TypeScript + Vite
- **Backend**: Rust + Tauri 2.0
- **UI Framework**: Web-based (React)
- **Platform APIs**: Windows API via Rust bindings

## Key Differences

### 1. Window Management

**macOS:**
```swift
// Uses NSPanel for floating window
let panel = NSPanel()
panel.styleMask = [.nonactivatingPanel, .borderless]
```

**Windows:**
```rust
// Uses Tauri's window API with transparent background
WindowBuilder::new(app, "pet", WindowUrl::App("/".into()))
    .transparent(true)
    .decorations(false)
    .always_on_top(true)
    .skip_taskbar(true)
```

### 2. Terminal Integration

**macOS:**
- Uses AppleScript to interact with terminals
- Accessibility APIs for reading window titles

**Windows:**
- Uses Windows UI Automation API
- Direct window handle manipulation
- Supports PowerShell, CMD, Windows Terminal, Git Bash, WSL

### 3. Secure Storage

**macOS:**
```swift
// Keychain Services
let query: [String: Any] = [
    kSecClass as String: kSecClassGenericPassword,
    kSecAttrService as String: service,
    kSecValueData as String: data
]
SecItemAdd(query as CFDictionary, nil)
```

**Windows:**
```rust
// Windows Credential Manager
CredWriteW(&credential, 0)?;
```

### 4. Configuration Storage

**macOS:**
- `~/Library/Application Support/TermiPet/`

**Windows:**
- `%APPDATA%/TermiPet/` (via Tauri's app_data_dir)

### 5. System Tray

**macOS:**
- NSStatusBar for menu bar icon

**Windows:**
- Tauri's tray icon API
- Supports both system tray and notification area

## Feature Mapping

| Feature | macOS | Windows | Status |
|---------|-------|---------|--------|
| Floating Pet Window | ✅ | ✅ | Implemented |
| Drag to Move | ✅ | ✅ | Implemented |
| Hover Toolbar | ✅ | ✅ | Implemented |
| Pet Animations | ✅ | ✅ | Implemented |
| Chat with Pet | ✅ | ✅ | Implemented |
| Local Models (Ollama) | ✅ | ✅ | Implemented |
| Online APIs | ✅ | ✅ | Implemented |
| Terminal Detection | ✅ | ⚠️ | Partial |
| Command Sending | ✅ | ⚠️ | Partial |
| AI Usage Cards | ✅ | ⚠️ | Partial |
| Pomodoro Timer | ✅ | ✅ | Implemented |
| Multiple Skins | ✅ | ✅ | Implemented |
| Multi-language | ✅ | ✅ | Implemented |
| Custom Pets | ✅ | ✅ | Implemented |
| Accessibility | ✅ | ⚠️ | Different API |

## Implementation Notes

### Terminal Detection on Windows

Windows uses a different approach for terminal detection:

```rust
// Enumerate windows to find terminals
EnumWindows(Some(enum_callback), LPARAM(&mut terminals as *mut _ as isize));

// Check window titles for terminal keywords
let terminal_keywords = ["PowerShell", "Command Prompt", "Windows Terminal", ...];
```

### Window Position Persistence

Both versions save window position, but Windows requires explicit position management:

```rust
// Save position
let position = window.outer_position()?;
store.set("window_position", json!({ "x": position.x, "y": position.y }));

// Restore position
window.set_position(PhysicalPosition { x, y })?;
```

### Pet Animation System

The sprite-based animation system is preserved:
- Same sprite sheet format (9 rows × N frames)
- Same animation states (idle, running, happy, etc.)
- Web-based rendering using CSS background-position

## Building and Running

### Prerequisites
1. Install Rust: https://rustup.rs/
2. Install Node.js: https://nodejs.org/
3. Install WebView2 runtime (usually pre-installed on Windows 10+)

### Development
```bash
# Install dependencies
npm install

# Run in development mode
npm run tauri:dev
```

### Production Build
```bash
# Build for distribution
npm run tauri:build
```

Output will be in `src-tauri/target/release/bundle/`.

## Known Limitations

1. **Terminal Integration**: Windows terminal detection is less reliable than macOS due to different window management models.

2. **Accessibility**: Windows UIA is more complex than macOS Accessibility APIs. Some features may require additional permissions.

3. **Code Signing**: Windows requires code signing for distribution outside of development. The build script includes self-signing for local testing.

4. **Pet Resources**: Pet sprite sheets need to be compatible with web rendering (WebP format recommended).

## Future Improvements

- [ ] Better Windows Terminal integration using WT's CLI
- [ ] WSL detection and support
- [ ] PowerShell 7+ specific features
- [ ] Windows 11 snap layout support
- [ ] Touch/pen input support for tablets

## Contributing

When porting features from macOS to Windows:

1. Check if there's a direct Windows equivalent
2. If not, look for Tauri plugins or Windows crates
3. Document any behavioral differences
4. Update this guide with your findings
