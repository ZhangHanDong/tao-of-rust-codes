//! 附录表：
//!
//! # rust-lldb debug示例
//!
//!  命令：
//!  ```rust
//! 	$ sudo rust-lldb ./lldb
//! ```
//!
//! 调试命令：
//! - b main // 设置断点
//! - r // 执行程序
//! - c // 继续执行


fn hello(){
    println!("新时代的语言：{}", "Rust");
}

pub mod lldb;
