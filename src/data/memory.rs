// src/memory.rs

use sysinfo::{System};
use std::collections::VecDeque;

// 用于存储内存的静态信息，类似 CoreInfo
pub struct MemoryInfo {
    pub total_memory: u64,
}

// 负责跟踪内存使用率历史
pub struct Memory {
    pub info: MemoryInfo,
    pub usages: VecDeque<f64>, // 内存使用率历史
    pub max_history: usize,
    system: System,
}

impl MemoryInfo {
    // 获取总内存（GB）
    pub fn get_total_memory_gb(&self) -> f64 {
        self.total_memory as f64 / 1024.0 / 1024.0 / 1024.0
    }
}

impl Memory {
    pub fn new() -> Memory {
        let mut system = System::new();
        system.refresh_memory(); // 初始化时刷新一次内存信息
        let info = MemoryInfo {
            total_memory: system.total_memory(),
        };

        Memory {
            info,
            usages: VecDeque::new(),
            max_history: 60, // 内存历史可以设置长一点，比如60个点
            system,
        }
    }

    pub fn update(&mut self) {
        // 1. 刷新内存数据
        self.system.refresh_memory();

        // 2. 计算内存使用率 (已用 / 总计) * 100
        let used_memory = self.system.used_memory();
        let total_memory = self.system.total_memory();
        let usage = (used_memory as f64 / total_memory as f64) * 100.0;

        // 3. 更新使用率历史
        self.usages.push_back(usage);

        // 4. 如果历史记录超过最大长度，移除最旧的数据
        if self.usages.len() > self.max_history {
            self.usages.pop_front();
        }
    }




}
