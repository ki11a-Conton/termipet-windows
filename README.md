# TermiPet for Windows 🐱💻

<p align="center">
  <img src="https://img.shields.io/badge/Windows-10+-0078D6?style=flat-square&logo=windows" alt="Windows">
  <img src="https://img.shields.io/badge/Tauri-2.0-24C8D8?style=flat-square&logo=tauri" alt="Tauri">
  <img src="https://img.shields.io/badge/Rust-1.75+-000000?style=flat-square&logo=rust" alt="Rust">
  <img src="https://img.shields.io/badge/React-18-61DAFB?style=flat-square&logo=react" alt="React">
  <img src="https://img.shields.io/badge/License-Apache%202.0-blue?style=flat-square" alt="License">
</p>

<p align="center">
  <b>一个运行在 Windows 桌面上的可爱宠物助手</b><br>
  陪伴你编程、提醒你休息、帮你提高效率
</p>

<p align="center">
  <a href="#快速开始">快速开始</a> •
  <a href="#功能特性">功能特性</a> •
  <a href="#使用方法">使用方法</a> •
  <a href="#开发指南">开发指南</a> •
  <a href="#文档">文档</a>
</p>

---

## ✨ 功能特性

| 功能 | 描述 | 状态 |
|------|------|------|
| 🐱 **悬浮宠物** | 可爱的猫咪悬浮在屏幕边缘，支持拖拽移动 | ✅ |
| 💬 **AI 聊天** | 支持本地 Ollama 和在线 API (OpenAI/Gemini) | ✅ |
| 🖥️ **终端集成** | 识别 Windows Terminal、PowerShell、CMD、WSL | ✅ |
| ⚡ **快捷指令** | 一键发送常用命令到终端 | ✅ |
| 📁 **快速切换目录** | 点击即可在终端中切换项目目录 | ✅ |
| 🍅 **番茄钟** | 25分钟专注 + 5分钟休息，完成有庆祝动画 | ✅ |
| 📊 **AI 用量卡片** | 查看 Claude Code、Copilot、Ollama 状态 | ✅ |
| 🎨 **多主题** | 玻璃、暗色、像素、浅色四种主题 | ✅ |
| 🌐 **多语言** | 简体中文、繁體中文、English、日本語、한국어 | ✅ |
| 🐾 **自定义宠物** | 支持导入 Petdex 格式的宠物资源包 | ✅ |

---

## 🚀 快速开始

### 普通用户

#### 方法一：下载安装包（最简单）

1. 访问 [Releases](../../releases) 页面
2. 下载 `TermiPet-Windows-v0.1.0.msi` 或 `.exe`
3. 双击安装，按提示完成
4. 从开始菜单启动 TermiPet

#### 方法二：使用包管理器

```powershell
# 使用 winget（推荐）
winget install TermiPet

# 或使用 scoop
scoop install termipet
```

### 开发者

```powershell
# 1. 克隆项目
git clone https://github.com/yourname/termipet-windows.git
cd termipet-windows

# 2. 一键设置和运行
.\Scripts\build.ps1 -Setup    # 首次设置
.\Scripts\build.ps1 -Dev      # 开发模式

# 或手动操作
npm install
npm run tauri:dev
```

**前提条件：**
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.75+
- WebView2 (Windows 10/11 通常已预装)

---

## 📖 使用方法

### 基本交互

| 操作 | 效果 |
|------|------|
| 🖱️ 鼠标悬停 | 显示工具栏 |
| 🖱️ 拖拽 | 移动宠物位置 |
| 🖱️ 右键托盘图标 | 打开菜单 |
| 🖱️ 左键点击宠物 | 触发互动 |

### 工具栏功能

悬停在宠物上时显示：

- 🖥️ **终端** - 发送快捷指令
- 📁 **文件夹** - 快速切换目录
- 💬 **聊天** - 和宠物对话
- 🎨 **皮肤** - 切换主题
- ⏱️ **计时器** - 番茄钟
- ☕ **休息** - 休息计时

### 配置聊天

**本地模型（免费，隐私）：**
```powershell
# 安装 Ollama
winget install Ollama.Ollama
ollama pull qwen2.5:1.5b

# 在 TermiPet 设置中选择 "本地模型"
```

**在线 API：**
1. 设置 → 聊天 → 在线 API
2. 输入 API Key（安全存储在 Windows 凭证管理器）
3. 点击 "测试连接"

---

## 🛠️ 开发指南

### 项目结构

```
termipet-windows/
├── src/                    # 前端 (React + TypeScript)
│   ├── components/         # UI 组件
│   ├── stores/            # Zustand 状态管理
│   └── styles/            # CSS 样式
├── src-tauri/             # 后端 (Rust)
│   ├── src/
│   │   ├── commands/      # Tauri 命令
│   │   ├── services/      # 业务逻辑
│   │   └── models/        # 数据模型
│   └── Cargo.toml
├── Pets/                  # 默认宠物资源
├── Scripts/               # 构建脚本
└── docs/                  # 文档
```

### 常用命令

```powershell
# 开发模式（热重载）
npm run tauri:dev

# 构建发布版本
npm run tauri:build

# 运行测试
cargo test

# 代码检查
npm run lint
```

### 技术栈

- **前端**: React 18 + TypeScript + Vite
- **后端**: Rust + Tauri 2.0
- **状态管理**: Zustand
- **样式**: CSS Modules
- **动画**: Framer Motion + CSS Animations

---

## 📚 文档

- [快速启动指南](./QUICKSTART.md) - 5 分钟上手
- [完整使用指南](./docs/USER_GUIDE.md) - 详细功能说明
- [开发文档](./docs/PORTING_GUIDE.md) - 架构和技术细节
- [API 文档](./docs/API.md) - 接口文档

---

## 🤝 参与贡献

我们欢迎各种形式的贡献！

1. 🐛 提交 Bug 报告
2. 💡 提出新功能建议
3. 🔧 提交代码修复
4. 📖 改进文档
5. 🎨 设计新宠物

查看 [CONTRIBUTING.md](./CONTRIBUTING.md) 了解详情。

---

## 📝 更新日志

### v0.1.0 (2024-01-XX)
- 🎉 初始版本发布
- ✨ 悬浮宠物窗口
- 💬 AI 聊天功能（Ollama/OpenAI/Gemini）
- 🖥️ 终端集成（PowerShell/CMD/WT/WSL）
- ⏱️ 番茄钟功能
- 🎨 4 种主题
- 🌐 5 种语言支持

---

## 📄 许可证

[Apache License 2.0](./LICENSE)

---

## 🙏 致谢

- 原项目 [TermiPet](https://github.com/bleeeet/TermiPet) by [@bleeeet](https://github.com/bleeeet)
- [Tauri](https://tauri.app/) - 优秀的 Rust 桌面应用框架
- [Petdex](https://petdex.crafter.run/) - 宠物资源社区

---

<p align="center">
  <b>让编程更有趣，让工作有陪伴</b> 🐱❤️
</p>
