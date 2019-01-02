//! 第五章：所有权系统
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第5章：{}", "所有权系统");
/// }
/// title();
/// ```
pub fn title(){
    println!("第5章：{}", "所有权系统");
}

pub mod semantic;
pub mod share_mutable;
pub mod borrow;
pub mod lifetime;
pub mod nll;
pub mod smart_pointer;
