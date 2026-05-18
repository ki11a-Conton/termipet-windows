# TermiPet Windows 使用指南

## 快速开始

### 方法一：下载预编译版本（推荐）

1. 访问项目的 Releases 页面
2. 下载 `TermiPet-Windows-v0.1.0.msi` 或 `TermiPet-Windows-v0.1.0.exe`
3. 双击安装程序，按提示完成安装
4. 从开始菜单或桌面快捷方式启动 TermiPet

### 方法二：从源码构建

#### 前提条件

1. **安装 Node.js** (v18+)
   - 访问 https://nodejs.org/
   - 下载 LTS 版本并安装

2. **安装 Rust** (1.75+)
   - 访问 https://rustup.rs/
   - 运行安装命令

3. **安装 WebView2** (Windows 10/11 通常已预装)
   - 如未安装，访问 https://developer.microsoft.com/en-us/microsoft-edge/webview2/

#### 构建步骤

```powershell
# 1. 克隆或下载项目
cd termipet-windows

# 2. 使用 PowerShell 脚本自动设置和构建
.\Scripts\build.ps1 -Setup    # 首次设置
.\Scripts\build.ps1 -Dev      # 开发模式运行

# 或手动步骤：
# 安装依赖
npm install

# 开发模式（带热重载）
npm run tauri:dev

# 构建发布版本
npm run tauri:build
```

构建完成后，安装包位于 `src-tauri/target/release/bundle/`

## 基本使用

### 启动应用

启动后，TermiPet 会：
1. 在系统托盘显示图标（右下角）
2. 在屏幕边缘显示宠物窗口
3. 自动检测可用的 AI 工具

### 宠物交互

| 操作 | 效果 |
|------|------|
| **鼠标悬停** | 显示工具栏 |
| **拖拽** | 移动宠物位置 |
| **左键点击** | 触发互动动画 |

### 工具栏功能

鼠标悬停在宠物上时，会显示工具栏：

- 🖥️ **终端** - 打开快捷指令面板
- 📁 **文件夹** - 快速切换项目目录
- 💬 **聊天** - 打开宠物聊天窗口
- 🎨 **皮肤** - 切换外观主题
- ⏱️ **计时器** - 开始/停止番茄钟
- ☕ **休息** - 开始 5 分钟休息

### 系统托盘菜单

右键点击托盘图标：

- **显示宠物** - 显示/隐藏宠物窗口
- **打开聊天** - 快速打开聊天窗口
- **设置** - 打开设置面板
- **开始番茄钟** - 25 分钟专注计时
- **停止计时** - 停止当前计时
- **退出** - 关闭应用

## 功能详解

### 1. 宠物聊天

#### 使用本地模型（Ollama）

1. 安装 Ollama
   ```powershell
   # 下载安装程序
   winget install Ollama.Ollama
   # 或访问 https://ollama.com/download
   ```

2. 下载模型
   ```powershell
   ollama pull qwen2.5:1.5b
   ```

3. 在 TermiPet 设置中选择 "本地模型"

#### 使用在线 API

1. 打开设置 → 聊天 → 在线 API
2. 选择提供商：
   - **OpenAI**: 输入 API Key
   - **Google Gemini**: 输入 API Key
   - **自定义**: 输入 Base URL 和 API Key

3. 点击 "测试连接" 验证
4. API Key 安全存储在 Windows 凭证管理器中

### 2. 终端集成

#### 支持的终端

- Windows Terminal
- PowerShell / PowerShell 7
- 命令提示符 (CMD)
- Git Bash
- WSL (Windows Subsystem for Linux)

#### 快捷指令

1. 打开并聚焦终端窗口
2. 鼠标悬停在宠物上，点击 "终端" 按钮
3. 选择要发送的命令：
   - `claude` - 启动 Claude Code
   - `/compact` - 压缩上下文
   - `/clear` - 清除对话
   - 自定义命令...

#### 快速切换目录

1. 点击工具栏的 "文件夹" 按钮
2. 选择项目文件夹
3. TermiPet 会自动在终端中执行 `cd` 命令

### 3. AI 用量查看

TermiPet 可以显示以下 AI 工具的使用情况：

#### Claude Code
- 显示当前套餐等级
- 已用请求数 / 限额
- Token 使用量
- 重置时间

#### GitHub Copilot
- 建议接受数
- 建议显示数
- 激活状态

#### Ollama
- 运行状态
- 版本信息
- 已安装的模型列表

### 4. 番茄钟

#### 使用方法

1. **开始专注**
   - 点击工具栏的 "计时器" 按钮
   - 或右键托盘菜单 → "开始番茄钟"
   - 默认 25 分钟

2. **开始休息**
   - 点击 "休息" 按钮
   - 默认 5 分钟

3. **停止计时**
   - 点击 "停止" 按钮

4. **自定义时长**
   - 打开设置 → 通用
   - 修改 "专注时长" 和 "休息时长"

### 5. 个性化设置

#### 更换宠物

1. 打开设置 → 宠物
2. 点击选择喜欢的宠物
3. 支持导入自定义宠物包（Petdex 格式）

#### 修改宠物性格

1. 设置 → 宠物 → 性格
2. 选择预设：
   - **友好** - 温和亲切
   - **专业** - 简洁高效
   - **活泼** - 充满活力
   - **冷静** - 沉稳淡定
   - **自定义** - 输入自定义 Prompt

#### 更换主题

1. 设置 → 外观
2. 选择主题：
   - **玻璃** - 现代透明效果
   - **暗色** - 深色主题
   - **像素** - 复古像素风格
   - **浅色** - 简洁明亮

#### 多语言

支持：简体中文、繁體中文、English、日本語、한국어

设置 → 外观 → 语言

## 高级功能

### 导入自定义宠物

1. 准备宠物资源包，包含：
   ```
   my-pet/
   ├── pet.json          # 宠物配置
   └── spritesheet.webp  # 精灵图
   ```

2. 设置 → 宠物 → 导入宠物
3. 选择宠物文件夹
4. 导入成功后即可使用

### 添加自定义快捷指令

1. 设置 → 终端
2. 点击 "添加命令"
3. 输入命令名称和内容
4. 点击置顶按钮可固定到顶部

### 快捷键

| 快捷键 | 功能 |
|--------|------|
| `Ctrl + Shift + T` | 显示/隐藏宠物 |
| `Ctrl + Shift + C` | 打开聊天 |
| `Ctrl + Shift + S` | 打开设置 |

## 故障排除

### 宠物窗口不显示

1. 检查系统托盘是否有 TermiPet 图标
2. 右键托盘图标 → "显示宠物"
3. 检查是否被其他窗口遮挡

### 终端检测失败

1. 确保终端窗口标题可见
2. 以管理员身份运行 TermiPet
3. 检查辅助功能权限（设置 → 终端 → 请求权限）

### 聊天连接失败

**Ollama:**
```powershell
# 检查 Ollama 是否运行
ollama list

# 重启 Ollama
ollama serve
```

**在线 API:**
1. 检查网络连接
2. 验证 API Key 是否正确
3. 点击 "测试连接" 查看错误信息

### 应用崩溃或卡顿

1. 重启 TermiPet
2. 清除配置缓存：
   ```powershell
   # 删除配置目录
   Remove-Item -Recurse -Force "$env:APPDATA\TermiPet"
   ```
3. 重新安装应用

## 数据存储位置

| 数据类型 | 存储位置 |
|---------|---------|
| 应用配置 | `%APPDATA%\TermiPet\settings.json` |
| API 密钥 | Windows 凭证管理器 |
| 聊天记录 | `%APPDATA%\TermiPet\chat_history.json` |
| 导入的宠物 | `%APPDATA%\TermiPet\ImportedPets\` |
| 日志文件 | `%APPDATA%\TermiPet\logs\` |

## 隐私说明

- **本地优先**: 所有配置和聊天记录存储在本地
- **API 密钥**: 使用 Windows 凭证管理器安全存储
- **无遥测**: 不向任何服务器发送使用数据
- **可选联网**: 仅在使用在线 AI 模型时连接外部 API

## 获取帮助

- **GitHub Issues**: 报告 Bug 或请求功能
- **文档**: 查看项目文档了解更多信息
- **日志**: 设置 → 关于 → 查看日志

## 更新日志

### v0.1.0
- 初始版本发布
- 悬浮宠物窗口
- 终端集成
- AI 聊天功能
- 番茄钟
- 多主题支持
