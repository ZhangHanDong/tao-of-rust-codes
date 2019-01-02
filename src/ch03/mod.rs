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
pub mod type_infer;
pub mod generics;
pub mod traits;
pub mod abstract_type;
pub mod tag_trait;
pub mod type_cast;
pub mod trait_limit;
