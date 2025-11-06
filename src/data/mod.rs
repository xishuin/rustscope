// 修改模块声明
pub mod core;
pub mod gpu;
pub mod memory; // 新增
pub mod net;

// 更新use语句\pub use core::Cores;
pub use core::Cores;
pub use memory::Memory; // 新增
pub use net::NetInfo;
