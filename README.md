# DS本地化

一个基于 Tauri 的桌面应用，提供 DeepSeek Chat 的本地化体验。

## 功能特性

- 🖥️ 原生桌面体验
- 🎨 无边框窗口设计
- 📱 响应式界面 (1200×800)
- 🔗 直接访问 DeepSeek Chat
- 快捷键支持：Alt + Z 关闭和打开窗口

## 技术栈

- **前端**: DeepSeek Chat (Web)
- **框架**: Tauri 2.0
- **语言**: Rust + Web Technologies

## 开发

### 环境要求

- Node.js
- Rust
- Tauri CLI

### 安装依赖

```bash
npm install
```

### 开发模式

```bash
npm run tauri dev
```

### 构建应用

```bash
npm run tauri build
```

## 配置

应用配置位于 `src-tauri/tauri.conf.json`，主要设置：

- 产品名称: "ds本地化"
- 窗口尺寸: 1200×800
- 无边框设计
- 直接加载远程内容

## 许可证

MIT License
