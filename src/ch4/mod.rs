//! 第四章：内存管理
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第4章：{}", "内存管理");
/// }
/// title();
/// ```
pub fn title(){
    println!("第4章：{}", "内存管理");
}

pub mod general_concepts;
pub mod resource_management;
pub mod raii;
