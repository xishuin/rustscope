// src/main.rs

mod app;
mod ui;
mod data;
use clap::Parser;
use std::io;

use app::App;

/// 一个简单的终端系统监控工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // ... Cli 结构体定义保持不变 ...
    #[arg(short = 'c', long)]
    cpu: bool,
    #[arg(short = 'm', long)]
    memory: bool,
    #[arg(short = 'g', long)]
    gpu: bool,
    #[arg(short = 'n', long)]
    net: bool,
}

fn main() -> io::Result<()> {
    // 1. 解析命令行参数
    let cli = Cli::parse();

    // 2. 决定显示哪些模块
    let show_all = !cli.cpu && !cli.memory && !cli.gpu && !cli.net;

    // 3. 创建 App 实例
    let mut app = if show_all {
        App::default()
    } else {
        App::new(cli.cpu, cli.memory, cli.gpu, cli.net)
    };

    // 4. 运行应用
    app.run()?;

    Ok(())
}
