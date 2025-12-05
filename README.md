# FileOG - 智能文件整理工具

<p align="center">
  <img src="src-tauri/icons/icon.svg" width="128" height="128" alt="FileOG Logo">
</p>

<p align="center">
  <strong>AI 驱动的桌面文件整理应用</strong>
</p>

<p align="center">
  <a href="#功能特性">功能特性</a> •
  <a href="#技术栈">技术栈</a> •
  <a href="#安装使用">安装使用</a> •
  <a href="#开发指南">开发指南</a> •
  <a href="#许可证">许可证</a>
</p>

---

## 功能特性

- **目录扫描** - 快速扫描指定目录，支持递归扫描子目录
- **文件预览** - 支持图片、文档等多种文件类型的预览
- **智能分类** - 基于 LLM 的智能文件分类，支持多种 AI 提供商
- **批量操作** - 支持批量移动、复制、删除等文件操作
- **重复检测** - 基于内容哈希检测重复文件
- **操作历史** - 完整的操作历史记录，支持撤销操作
- **自定义分类** - 支持自定义文件分类规则和颜色标记

## 技术栈

### 前端
- **React 18** - 用户界面框架
- **TypeScript** - 类型安全的 JavaScript
- **Vite** - 快速的前端构建工具
- **Tailwind CSS** - 原子化 CSS 框架
- **shadcn/ui** - 精美的 UI 组件库
- **Lucide React** - 图标库

### 后端
- **Tauri 2.0** - 跨平台桌面应用框架
- **Rust** - 高性能系统编程语言
- **SQLite** - 本地数据库存储
- **reqwest** - HTTP 客户端

### AI 集成
- **OpenAI API** - GPT 系列模型
- **Anthropic API** - Claude 系列模型
- **Ollama** - 本地 AI 模型运行

## 安装使用

### 下载安装

从 [Releases](https://github.com/Kalstro/FileOG/releases) 页面下载最新版本：

- **Windows**: `FileOG_x.x.x_x64-setup.exe` (NSIS 安装程序) 或 `FileOG_x.x.x_x64_en-US.msi`
- **macOS**: 即将支持
- **Linux**: 即将支持

### 配置 LLM

1. 打开应用，点击右上角设置按钮
2. 在 LLM 设置页面启用智能分类
3. 选择 AI 提供商并填写 API Key
4. 点击"测试连接"验证配置

支持的 AI 提供商：
- OpenAI (GPT-4o, GPT-4o-mini 等)
- Anthropic (Claude 3.5 Sonnet, Claude 3 Opus 等)
- Ollama (本地运行，无需 API Key)
- 自定义 OpenAI 兼容端点

## 开发指南

### 环境要求

- Node.js 18+
- pnpm 8+
- Rust 1.70+
- Windows 10/11 (开发环境)

### 本地开发

```bash
# 克隆仓库
git clone https://github.com/Kalstro/FileOG.git
cd FileOG

# 安装依赖
pnpm install

# 启动开发服务器
pnpm tauri dev

# 构建发布版本
pnpm tauri build
```

### 项目结构

```
FileOG/
├── src/                    # 前端源码
│   ├── components/         # React 组件
│   │   ├── file/          # 文件相关组件
│   │   ├── layout/        # 布局组件
│   │   ├── operation/     # 操作相关组件
│   │   ├── settings/      # 设置相关组件
│   │   └── ui/            # 基础 UI 组件
│   ├── services/          # 前端服务
│   ├── types/             # TypeScript 类型定义
│   └── App.tsx            # 应用入口
├── src-tauri/             # Rust 后端源码
│   ├── src/
│   │   ├── commands/      # Tauri 命令
│   │   ├── models/        # 数据模型
│   │   ├── services/      # 后端服务
│   │   └── lib.rs         # 库入口
│   ├── icons/             # 应用图标
│   └── Cargo.toml         # Rust 依赖配置
├── package.json           # Node.js 依赖配置
└── README.md              # 项目说明
```

## 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件

## 致谢

- [Tauri](https://tauri.app/) - 优秀的跨平台桌面应用框架
- [shadcn/ui](https://ui.shadcn.com/) - 精美的 React 组件库
- [Lucide](https://lucide.dev/) - 开源图标库
