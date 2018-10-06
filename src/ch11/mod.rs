//! 第十一章：安全并发
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第11章：{}", "安全并发");
/// }
/// title();
/// ```
pub fn title(){
    println!("第11章：{}", "安全并发");
}

pub mod thread_unsafe;
pub mod thread_management;
pub mod thread_sync;
pub mod atomics;
pub mod channels;
pub mod rayon;
pub mod crossbeam;
pub mod generator;
