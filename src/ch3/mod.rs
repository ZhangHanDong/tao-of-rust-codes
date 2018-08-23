//! 第三章：类型系统
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第3章：{}", "类型系统");
/// }
/// title();
/// ```
pub fn title(){
    println!("第3章：{}", "类型系统");
}

pub mod type_size;
pub mod zero_size;
pub mod bottom_type;
