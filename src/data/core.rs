use sysinfo::{System};
use std::collections::VecDeque;
pub struct CoreInfo {
    pub brand : String,
    pub frequency : u64,
    pub vendor_id : String,
    pub usages : VecDeque<f64>,
    max_history : usize,
}
pub struct Cores {
    pub number : usize,
    pub cores : Vec<CoreInfo>,
    pub usages : VecDeque<f64>, //这是所有核心的平均使用率
    pub max_history : usize,
    system : System,
}

impl Cores {
    pub fn new()->Cores{
        let mut system=System::new();
        system.refresh_cpu_all();
        let mut cores_info:Vec<CoreInfo>=Vec::new();
        for cpu in system.cpus(){
            let cpu_info=CoreInfo{
                brand:cpu.brand().to_string(),
                frequency:cpu.frequency(),
                vendor_id:cpu.vendor_id().to_string(),
                usages:VecDeque::new(),
                max_history:10,
            };
            cores_info.push(cpu_info);
        }
        Cores{
            number:system.cpus().len(),
            cores:cores_info,
            usages:VecDeque::new(),
            max_history:10,
            system:system,
        }
    }
    
    pub fn update(&mut self) {
        // 1. 刷新所有CPU数据
        self.system.refresh_cpu_all();

        // 2. 更新每个核心的使用率历史
        for (i, cpu) in self.system.cpus().iter().enumerate() {
            let usage = cpu.cpu_usage() as f64;
            self.cores[i].usages.push_back(usage);

            if self.cores[i].usages.len() > self.cores[i].max_history {
                self.cores[i].usages.pop_front();
            }
        }

        // 3. 计算并更新总体平均使用率历史
        let total_usage = self.system.global_cpu_usage() as f64;
        self.usages.push_back(total_usage);

        if self.usages.len() > self.max_history {
            self.usages.pop_front();
        }
    }
}