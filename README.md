# RustScope
  
一款轻量级、高效的终端系统监控工具，使用 Rust 语言编写。

</div>

## 项目简介

RustScope 是一款轻量级的终端系统监控工具，使用 Rust 语言开发。它能够实时显示系统的 CPU 使用率、内存占用和网络流量等关键指标，并以图表形式直观展示历史数据趋势，帮助用户随时监控系统性能状态。

## 主要功能

- **实时监控**：动态显示 CPU 使用率、内存占用和网络流量
- **历史数据图表**：使用文本图表在终端中展示各项指标的变化趋势
- **多模块切换**：支持单独或组合查看不同监控模块 ( -c , -m , -n )
- **智能网络接口选择**：自动识别并选择主要网络接口（WiFi 或以太网）
- **轻量高效**：资源占用极低，适合长期在后台运行

## 演示
![Demo GIF](https://github.com/user-attachments/assets/7373d1a3-3a4f-4066-a059-506620a6db91)

## 使用方法

### 命令行参数

- `-c, --cpu`：显示 CPU 使用率图表
- `-m, --memory`：显示内存占用图表
- `-n, --net`：显示网络流量图表
- 可以组合使用参数，例如 `scope -c -m` 同时显示 CPU 和内存

### 交互式操作

- 按 `q` 键退出程序
- 数据会自动实时刷新，无需手动操作

## 快速开始

### 方法一：使用预编译版本（推荐，无需 Rust 环境）

对于不想配置开发环境的用户，这是最简单的方式。

#### Linux 安装

打开终端，复制并执行以下命令即可完成自动下载、解压和安装：

```bash
# 下载、解压、进入目录并赋予安装脚本执行权限
wget https://github.com/user-attachments/files/23395750/linux-release.zip && unzip linux-release.zip && cd linux-release && chmod +x installer.sh

# 运行安装脚本
./installer.sh
```

安装脚本会：
- 将可执行文件复制到 `~/.local/bin` 目录
- 自动配置环境变量（更新 `.bashrc` 或 `.zshrc`）

安装完成后，您可以在任何目录下运行 `scope -m` 等命令。

> 注意：如果是首次配置环境变量，需要重启终端或执行 `source ~/.bashrc`（或 `source ~/.zshrc`）以应用更改。

#### Windows 安装

1. **下载文件**：
   - 访问项目的 Releases 页面，下载 `windows-release.zip` 文件

2. **解压文件**：
   - 右键点击 `windows-release.zip` 文件，选择"全部解压…"

3. **运行安装程序**：
   - 进入解压后的文件夹，右键点击 `install.bat` 文件，选择 "以管理员身份运行"

安装脚本会：
- 将可执行文件复制到系统目录
- 创建全局可用的 `scope` 命令

安装完成后，您可以在任何命令提示符窗口中运行 `scope -m` 等命令。

> 注意：安装后可能需要关闭并重新打开命令提示符窗口。

### 方法二：使用 Rust 开发环境构建

如果您已安装 Rust 开发环境，可以通过以下步骤从源码构建：

```bash
# 克隆仓库
git clone https://github.com/yourusername/rustscope.git
cd rustscope

# 构建并运行
cargo run -- -c -m -n  # 运行所有监控模块

# 或者只运行特定模块
cargo run -- -m       # 只运行内存监控模块

# 构建发布版本
cargo build --release
# 构建后的可执行文件位于 target/release/rustscope
```

## 系统要求

**操作系统**：
- Linux (大部分现代发行版)
- Windows 10 及以上版本
- macOS (理论上支持，部分功能可能受限)

**终端**：支持 ANSI 转义序列和 UTF-8 编码的现代终端。

## 项目结构

rustscope/
├── src/
│   ├── main.rs          # 程序入口
│   ├── modules/
│   │   ├── mod.rs       # 模块声明
│   │   ├── cpu.rs       # CPU 监控模块
│   │   ├── memory.rs    # 内存监控模块
│   │   └── network.rs   # 网络监控模块
│   └── utils/
│       ├── mod.rs       # 工具模块声明
│       └── chart.rs     # 图表绘制工具
├── Cargo.toml           # 项目配置和依赖
├── LICENSE              # 许可证文件
└── README.md            # 项目说明文档

