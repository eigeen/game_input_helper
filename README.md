# Game Input Helper

游戏中文输入工具，目前仅支持 Helldivers 2 （绝地潜兵2）

## 用法

按下Enter，游戏内会打开输入框，同时也会弹出应用窗口（确保这时候游戏的输入框是激活的）

在应用窗口内输入文本，按回车即发送到游戏，并自动发送消息。

在任何时候都可以通过F7显示或隐藏窗口。

## 构建

### 环境需求

- Rust 1.88
- nodejs 20+
- pnpm

### 构建步骤

```bash
# 开发环境
pnpm tauri dev

# 构建发布版本
pnpm tauri build
```
