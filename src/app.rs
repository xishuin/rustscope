// src/app.rs
use sysinfo::{System};
use std::io;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::{CrosstermBackend},
    Terminal,
};
use crate::ui::draw;
use crate::data::{Cores, Memory}; // 新增 Memory 模块
use crate::data::net::NetInfo; // 新增 NetInfo 模块
pub struct SystemInfo {
    pub cores: Cores, // 改为 pub，方便 DrawUi 访问
    //后期可以添加更多，比如内存、GPU等
    pub memory: Memory, // 新增 Memory 字段
    pub net: NetInfo, // 新增 NetInfo 字段
    pub sys: System, // 新增 System 字段
}

pub struct App {
    pub system_info: SystemInfo, // 改为 pub
    pub show_cpu: bool,
    pub show_memory: bool,
    pub show_gpu: bool,
    pub show_net: bool,

}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        let cores = Cores::new();
        let memory = Memory::new(); // 初始化 Memory
        let mut net = NetInfo::new(); // 初始化 NetInfo
        let mut system = System::new(); // <--- 新增：创建 System 实例
        system.refresh_all(); // <--- 新增：初始化时刷新所有信息
        net.networks.refresh(true);
        net.initialize();
        SystemInfo {cores, memory, net, sys: system}
    }
}

impl App {
    pub fn new(show_cpu: bool, show_memory: bool, show_gpu: bool, show_net: bool) -> App {
        App {
            system_info: SystemInfo::new(),

            show_cpu,
            show_memory,
            show_gpu,
            show_net,
        }
    }
    pub fn default() -> App {
        App {
            system_info: SystemInfo::new(),

            show_cpu: true,
            show_memory: true,
            show_gpu: true,
            show_net: true,
        }
    }

    pub fn run(&mut self) -> io::Result<()> {
        // 创建终端
       
        let mut terminal;
    match setup_terminal() {
        Ok(term) => {
            terminal = term;
        },
        Err(e) => {
            eprintln!("Error setting up terminal: {}", e);
            //这里的错误逻辑可以进一步处理，比如返回一个错误码或者退出程序
            return Err(e);
        }
    }
        // 运行主循环(单次循环)
        self.main_loop(&mut terminal)?;
        // 恢复终端


        restore_terminal(&mut terminal)?;

        Ok(())
    }
   // 一个更完整的主循环
fn main_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    let mut last_update = Instant::now();
    let tick_rate = Duration::from_millis(250); // 每 250ms 刷新一次UI
    loop {
        // 1. 检查用户输入 (非阻塞)
        if event::poll(tick_rate)? { // <-- 关键点
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break, // 按下 'q' 就退出循环
                    _ => {}
                }
            }
        }
        // 更新系统信息
        // 2. 更新界面 (无论有没有输入都执行)
        // 检查是否到了该刷新的时间
        if last_update.elapsed() >= tick_rate {
            self.system_info.sys.refresh_all(); // <--- 新增：刷新所有系统信息
            self.system_info.cores.update(); // 更新CPU信息
            self.system_info.memory.update(); // 更新内存信息
            self.system_info.net.networks.refresh(true);
            self.system_info.net.update(); // 更新网络信息


            terminal.draw(|f| {
        // 这里的 f 就是 &mut Frame
        draw::draw(f, &self);
    })?;
            last_update = Instant::now(); // 重置计时器
        }
    }
    Ok(())
}

}
fn setup_terminal() -> io::Result<Terminal<CrosstermBackend<io::Stdout>>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let terminal = Terminal::new(backend)?;
    Ok(terminal)
}
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;
    Ok(())

}
