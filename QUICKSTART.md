# TermiPet Windows 快速启动指南

## 🚀 5 分钟快速开始

### 如果你是普通用户

1. **下载安装包**（等待发布）
   - 下载 `TermiPet-Windows-v0.1.0.msi`
   - 双击安装
   - 从开始菜单启动

2. **首次启动**
   - 宠物会出现在屏幕右下角
   - 系统托盘会出现猫咪图标

3. **开始使用**
   - 🖱️ **鼠标悬停**宠物 → 显示工具栏
   - 🖱️ **拖拽**宠物 → 移动位置
   - 🖱️ **右键托盘图标** → 菜单选项

### 如果你是开发者

#### 环境准备（一次性）

```powershell
# 1. 安装 Node.js (v18+)
winget install OpenJS.NodeJS

# 2. 安装 Rust
# 访问 https://rustup.rs/ 运行安装命令
# 或在 PowerShell 运行:
winget install Rustlang.Rustup

# 3. 重启终端，验证安装
node --version    # 应显示 v18+
rustc --version   # 应显示 1.75+
```

#### 运行项目

```powershell
# 1. 进入项目目录
cd termipet-windows

# 2. 一键设置（首次）
.\Scripts\build.ps1 -Setup

# 3. 开发模式运行
.\Scripts\build.ps1 -Dev

# 或手动:
npm install
npm run tauri:dev
```

## 🎯 核心功能速览

### 💬 和宠物聊天

**方式一：本地模型（免费，隐私）**
```powershell
# 1. 安装 Ollama
winget install Ollama.Ollama

# 2. 下载模型
ollama pull qwen2.5:1.5b

# 3. 在 TermiPet 设置中选择 "本地模型"
```

**方式二：在线 API**
1. 打开设置 → 聊天 → 在线 API
2. 输入 OpenAI / Gemini API Key
3. 点击 "测试连接"

### 🖥️ 终端快捷指令

1. 打开 PowerShell / CMD / Windows Terminal
2. 鼠标悬停在宠物上
3. 点击 "终端" 按钮
4. 选择命令发送

### ⏱️ 番茄钟

- 点击工具栏 "计时器" → 开始 25 分钟专注
- 点击 "休息" → 开始 5 分钟休息
- 完成时宠物会庆祝动画 🎉

## 🛠️ 常见问题

### Q: 宠物窗口不见了？
A: 右键系统托盘图标 → "显示宠物"

### Q: 终端检测不到？
A: 以管理员身份运行 TermiPet

### Q: 聊天连接失败？
A: 
- 本地模型：检查 Ollama 是否运行 (`ollama list`)
- 在线 API：检查网络连接和 API Key

### Q: 如何彻底卸载？
A: 
```powershell
# 删除应用数据
Remove-Item -Recurse -Force "$env:APPDATA\TermiPet"
# 然后正常卸载程序
```

## 📚 详细文档

- [完整使用指南](./docs/USER_GUIDE.md)
- [开发文档](./docs/PORTING_GUIDE.md)
- [API 文档](./docs/API.md) (待补充)

## 🤝 参与贡献

1. Fork 项目
2. 创建分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'Add amazing feature'`)
4. 推送分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 📞 获取帮助

- 提交 Issue: GitHub Issues 页面
- 查看日志: 设置 → 关于 → 查看日志

---

**享受你的编程伴侣！** 🐱💻
