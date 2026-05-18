# Claude Code 桌面端集成指南

TermiPet 完美支持 Claude Code 桌面端，可以检测状态、发送命令、查看用量。

## 支持的 Claude Code 版本

- ✅ Claude Code CLI (`claude` 命令)
- ✅ Claude Code Desktop App (Windows 桌面应用)
- ✅ 通过 Windows Terminal 运行的 Claude Code

## 功能特性

### 1. 自动检测

TermiPet 会自动检测：
- Claude Code 是否已安装
- Claude Code 是否正在运行
- Claude Code 窗口状态

### 2. 快捷指令

悬停宠物 → 点击 "终端" 按钮，可快速发送命令：

| 命令 | 说明 |
|------|------|
| `claude` | 启动 Claude Code |
| `/compact` | 压缩上下文 |
| `/clear` | 清除对话 |
| `/status` | 查看状态 |
| `/diff` | 查看代码差异 |
| `/review` | 代码审查 |
| `/cost` | 查看费用 |

### 3. 状态卡片

当 Claude Code 运行时，宠物会显示状态卡片：
- 🟢 正在思考
- 🔵 等待授权
- 🟡 调用工具
- ✅ 任务完成

### 4. 用量查看

在设置 → AI 用量中查看：
- 当前套餐等级
- 已用请求数 / 限额
- Token 使用量
- 重置时间

## 配置步骤

### 第一步：安装 Claude Code

如果你还没有安装：

```powershell
# 通过 npm 安装
npm install -g @anthropic-ai/claude-code

# 或通过官方安装程序
# 访问 https://claude.ai/code 下载
```

### 第二步：配置 TermiPet

1. 打开 TermiPet 设置
2. 进入 "终端" 页面
3. 确保 "Claude Code 集成" 已启用
4. 点击 "安装 Hook"（可选，用于状态同步）

### 第三步：使用快捷指令

1. 打开 Claude Code（在终端中运行 `claude`）
2. 鼠标悬停在 TermiPet 宠物上
3. 点击 "终端" 按钮
4. 选择要发送的命令

## Hook 功能（高级）

### 什么是 Hook？

Hook 可以让 TermiPet 实时同步 Claude Code 的状态：
- 当 Claude 开始思考时，宠物显示 "思考中"
- 当需要授权时，宠物提醒你
- 当任务完成时，宠物庆祝

### 安装 Hook

```powershell
# 在 TermiPet 中
# 设置 → 终端 → 安装 Claude Code Hook
```

或手动安装：

编辑 `~/.claude/settings.json`：

```json
{
  "hooks": {
    "termipet": {
      "url": "http://127.0.0.1:8765/hook",
      "events": ["thinking", "tool_use", "permission_request", "completion"]
    }
  }
}
```

### Hook 事件类型

| 事件 | 说明 | 宠物反应 |
|------|------|---------|
| `thinking` | Claude 正在思考 | 🤔 思考动画 |
| `tool_use` | 调用工具 | ⚙️ 运行动画 |
| `permission_request` | 需要授权 | ⚠️ 提醒动画 |
| `completion` | 任务完成 | 🎉 庆祝动画 |

## 常见问题

### Q: TermiPet 检测不到 Claude Code？

**A:** 检查以下几点：

1. **确认安装**
   ```powershell
   claude --version
   ```

2. **检查窗口标题**
   - Claude Code 窗口标题应包含 "Claude" 或 "Anthropic"
   - 如果窗口标题被修改，可能无法检测

3. **以管理员身份运行 TermiPet**
   - 右键 TermiPet → "以管理员身份运行"

4. **手动添加路径**
   - 确保 `claude` 命令在系统 PATH 中

### Q: 发送命令没有反应？

**A:** 

1. 确保 Claude Code 窗口是可见的（不是最小化）
2. 确保 Claude Code 窗口有焦点
3. 尝试先点击 Claude Code 窗口，再发送命令

### Q: Hook 安装失败？

**A:**

1. 检查 `.claude` 目录是否存在：
   ```powershell
   ls ~/.claude
   ```

2. 手动创建配置：
   ```powershell
   mkdir -p ~/.claude
   echo '{"hooks":{}}' > ~/.claude/settings.json
   ```

3. 重新安装 Hook

### Q: 用量信息显示不正确？

**A:** 

- 免费版可能不显示具体用量
- 尝试运行 `claude status --json` 查看原始数据
- 确保已登录 Claude Code（运行 `claude login`）

## 使用示例

### 示例 1：快速启动 Claude Code

```powershell
# 方式 1：通过 TermiPet 工具栏
# 1. 鼠标悬停宠物
# 2. 点击 "终端" 按钮
# 3. 选择 "claude"

# 方式 2：通过托盘菜单
# 1. 右键托盘图标
# 2. 选择 "启动 Claude Code"
```

### 示例 2：代码审查工作流

```
1. 在 Claude Code 中加载项目
2. 在 TermiPet 中点击 "/review"
3. Claude Code 开始代码审查
4. TermiPet 宠物显示 "思考中" 动画
5. 审查完成后宠物庆祝 🎉
```

### 示例 3：上下文压缩

```
1. 当对话过长时
2. 在 TermiPet 中点击 "/compact"
3. Claude Code 自动压缩上下文
4. 节省 Token 用量
```

## 故障排除

### 启用调试日志

在 TermiPet 设置 → 关于 → 启用调试日志

查看日志文件：`%APPDATA%\TermiPet\logs\`

### 重置 Claude Code 集成

1. 卸载 Hook：
   ```powershell
   # 删除 ~/.claude/settings.json 中的 hooks.termipet
   ```

2. 重新安装 Hook

3. 重启 TermiPet

## 最佳实践

1. **保持 Claude Code 更新**
   ```powershell
   npm update -g @anthropic-ai/claude-code
   ```

2. **定期查看用量**
   - 在 TermiPet 设置中查看用量卡片
   - 避免超出限额

3. **使用快捷指令**
   - 将常用命令添加到快捷指令面板
   - 提高工作效率

4. **启用 Hook**
   - 获得更好的状态同步体验
   - 宠物会根据 Claude 状态变化动画

## 相关链接

- [Claude Code 官方文档](https://docs.anthropic.com/en/docs/claude-code/overview)
- [Claude Code GitHub](https://github.com/anthropics/claude-code)
- [TermiPet 使用指南](./USER_GUIDE.md)
