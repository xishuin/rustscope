// src/net.rs

use std::collections::VecDeque;
use std::time::{Duration, Instant};
use sysinfo::{System, Networks};

pub struct NetInfo {
    pub interface_name: String,  // 选择的网络接口名称
    pub received: u64,           // 总接收字节数
    pub transmitted: u64,        // 总发送字节数
    pub download_rate: f64,      // 下载速率 (KB/s)
    pub upload_rate: f64,        // 上传速率 (KB/s)
    pub download_rates: VecDeque<f64>, // 下载速率历史
    pub upload_rates: VecDeque<f64>,   // 上传速率历史
    pub last_updated: Instant,    // 上次更新时间
    pub networks: Networks,
}

impl NetInfo {
    pub fn new() -> Self {
        Self {
            interface_name: String::new(),
            received: 0,
            transmitted: 0,
            download_rate: 0.0,
            upload_rate: 0.0,
            download_rates: VecDeque::with_capacity(60),
            upload_rates: VecDeque::with_capacity(60),
            last_updated: Instant::now(),
            networks: Networks::new(),
        }
    }

    /// 初始化网络接口
pub fn initialize(&mut self) {
    // 定义一些常见的主要接口关键词，不区分大小写
    let primary_keywords = vec!["wlan", "eth", "en", "wl"];

    // --- 第一步：尝试智能选择一个主要接口 ---
    for (name, _) in self.networks.iter() {
        let name_lower = name.to_lowercase();
        for keyword in &primary_keywords {
            if name_lower.contains(keyword) {
                self.interface_name = name.to_string();
                
                // 找到接口后，立即初始化数据并返回
                if let Some(data) = self.networks.get(&self.interface_name) {
                    self.received = data.total_received();
                    self.transmitted = data.total_transmitted();
                    self.last_updated = Instant::now();
                }
                return; // 关键：找到后就退出函数
            }
        }
    }

    // --- 第二步：如果没找到主要接口，则回退到使用第一个接口 ---
    if let Some((name, data)) = self.networks.iter().next() {
        self.interface_name = name.to_string();
        
        self.received = data.total_received();
        self.transmitted = data.total_transmitted();
        self.last_updated = Instant::now();
    }
    if self.interface_name.is_empty() {
        self.interface_name = "N/A".to_string(); // 给一个明确的提示
    }


}
    /// 更新网络信息
    pub fn update(&mut self)->Result<(), String> {
        if self.interface_name == "N/A" {
            // 如果没有有效接口，直接返回
            return Err("No valid network interface found".to_string());
        }

        let now = Instant::now();
        let duration = now.duration_since(self.last_updated);

        if let Some((_, network_data)) = self.networks.iter().find(|(name, _)| name == &self.interface_name.as_str()) {
            let new_received = network_data.total_received();
            let new_transmitted = network_data.total_transmitted();

            let delta_received = new_received.saturating_sub(self.received);
            let delta_transmitted = new_transmitted.saturating_sub(self.transmitted);

            // 计算速率 (KB/s)
            if duration.as_secs_f64() > 0.0 {
                self.download_rate = (delta_received as f64 / duration.as_secs_f64()) / 1024.0;
                self.upload_rate = (delta_transmitted as f64 / duration.as_secs_f64()) / 1024.0;
            }

            // 更新历史数据
            self.download_rates.push_back(self.download_rate);
            self.upload_rates.push_back(self.upload_rate);

            if self.download_rates.len() > 60 {
                self.download_rates.pop_front();
            }
            if self.upload_rates.len() > 60 {
                self.upload_rates.pop_front();
            }

            // 保存当前总值，供下次计算
            self.received = new_received;
            self.transmitted = new_transmitted;
            self.last_updated = now;
        }
        Ok(())
    }

    /// 获取当前下载速率
    pub fn current_download_rate(&self) -> f64 {
        self.download_rates.back().copied().unwrap_or(0.0)
    }

    /// 获取当前上传速率
    pub fn current_upload_rate(&self) -> f64 {
        self.upload_rates.back().copied().unwrap_or(0.0)
    }

    /// 获取接口名称
    pub fn get_interface_name(&self) -> &str {
        &self.interface_name
    }

    /// 格式化速率显示
    pub fn format_rate(rate: f64) -> String {
        if rate < 1024.0 {
            format!("{:.1} KB/s", rate)
        } else if rate < 1024.0 * 1024.0 {
            format!("{:.1} MB/s", rate / 1024.0)
        } else {
            format!("{:.1} GB/s", rate / (1024.0 * 1024.0))
        }
    }
}
