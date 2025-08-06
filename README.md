# DS本地化

一个基于 Tauri 的桌面应用，提供 DeepSeek Chat 的本地化体验。

## 项目动因

平时一直有使用网页版 DeepSeek Chat 的习惯，但在浏览器中使用时总感觉不太舒服，经常关闭浏览器的时候就会退出，然后再次载入的时候又丢失了聊天记录，所以就想做一个本地化的桌面应用，让这个网页一直挂载后台，随时可以打开和关闭。为了极致的性能、大小和体验，选择了 Tauri 框架来构建这个应用。

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
npm run build
```

### 多平台构建

#### 本地构建（Windows）
```bash
npm run build                    # Windows x64
```

#### 苹果 M 芯片构建
由于交叉编译的复杂性，建议使用 GitHub Actions 进行构建：

1. 推送代码到 GitHub
2. 创建 tag 触发发布构建：
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
3. GitHub Actions 会自动为所有平台（包括苹果 M 芯片）构建并发布

#### 支持的平台
- Windows x64
- macOS Intel (x86_64)
- macOS Apple Silicon (ARM64)
- Linux x64

## 配置

应用配置位于 `src-tauri/tauri.conf.json`，主要设置：

- 产品名称: "ds本地化"
- 窗口尺寸: 1200×800
- 无边框设计
- 直接加载远程内容

## 许可证

MIT License
