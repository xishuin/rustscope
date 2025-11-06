# RustScope

## 项目简介

RustScope是一款轻量级的终端系统监控工具，使用Rust语言开发。它能够实时显示系统的CPU使用率、内存占用和网络流量等关键指标，并以图表形式直观展示历史数据趋势，帮助用户随时监控系统性能状态。

### 主要功能

- **实时监控**：动态显示CPU使用率、内存占用和网络流量
- **历史数据图表**：使用文本图表展示各项指标的变化趋势
- **多模块切换**：支持单独或组合查看不同监控模块
- **智能网络接口选择**：自动识别并选择主要网络接口（WiFi或以太网）
- **轻量高效**：资源占用低，适合长期运行

## 使用方法

### 命令行参数

- `-c, --cpu`：显示CPU使用率图表
- `-m, --memory`：显示内存占用图表
- `-n, --net`：显示网络流量图表
- 可以同时使用多个参数，如 `-c -m -n` 同时显示所有模块

### 交互式操作

- 按 `q` 键退出程序
- 支持实时自动刷新数据

## 快速开始

### 方法一：使用Rust开发环境构建

如果您已安装Rust开发环境，可以通过以下步骤构建并运行：

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

### 方法二：使用预编译版本（无需Rust环境）

对于没有Rust开发环境的用户，可以下载预编译的二进制文件并使用安装脚本：

1. **下载文件**
   - 前往项目的[Releases](https://github.com/yourusername/rustscope/releases)页面
   - 下载适合您系统的可执行文件（Linux或Windows）和安装脚本
   - 确保`scope`（或Windows上的`scope.exe`）和安装脚本在同一目录

2. **Linux系统安装**
   
   打开终端，导航到下载文件的目录，执行以下命令：
   
   ```bash
   # 赋予安装脚本执行权限
   chmod +x installer.sh
   
   # 运行安装脚本
   ./installer.sh
   ```
   
   安装脚本会：
   - 将可执行文件复制到`~/.local/bin`目录
   - 自动配置环境变量（更新`.bashrc`或`.zshrc`）
   - 安装完成后，您可以在任何目录下运行`scope -m`等命令
   
   > 注意：如果是首次配置环境变量，需要重启终端或执行`source ~/.bashrc`（或`source ~/.zshrc`）以应用更改

3. **Windows系统安装**
   
   下载Windows版本的安装脚本（install.bat），双击运行或在命令提示符中执行：
   
   ```cmd
   install.bat
   ```
   
   安装脚本会：
   - 创建`%USERPROFILE%\.rustscope`目录并复制可执行文件
   - 将该目录添加到系统环境变量PATH中
   - 安装完成后，您可以在任何命令提示符窗口中运行`scope -m`等命令
   
   > 注意：Windows下可能需要重启命令提示符或资源管理器以应用环境变量更改

## 系统要求

- **Linux**：支持大部分现代Linux发行版
- **Windows**：Windows 10及以上版本
- **macOS**：理论支持，具体功能可能有限制

## 项目结构
