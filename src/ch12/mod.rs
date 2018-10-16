//! 第十二章：元编程
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第12章：{}", "元编程");
/// }
/// title();
/// ```
pub fn title(){
    println!("第12章：{}", "元编程");
}

pub mod reflect;
pub mod macros;