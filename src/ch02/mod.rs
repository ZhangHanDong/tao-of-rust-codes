//! 第二章：语言精要
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第2章：{}", "语言精要");
/// }
/// title();
/// ```
pub fn title(){
    println!("第2章：{}", "语言精要");
}

/// # answer
///
/// Basic usage:
///
/// ```
/// // extern crate std;      // 声明语句
/// // use std::prelude::v1::*;  // 声明语句
///
/// pub fn answer() -> (){
///     let a = 40;  // 声明语句
///     let b = 2;   // 声明语句
///     assert_eq!(sum(a, b), 42); // 宏语句
/// }
///
/// pub fn sum(a: i32, b: i32) -> i32 {
///     a + b  // 表达式
/// }
///
/// answer(); // 表达式语句
/// ```

pub fn answer() -> (){
    let a = 40;
    let b = 2;
    assert_eq!(sum(a, b), 42);
}
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}

pub mod binding;
pub mod function;
pub mod control_flow;
pub mod collections;
pub mod primitives;
pub mod enums;
pub mod structs;
pub mod smart_pointer;
pub mod generics_trait;
pub mod errors_handle;
pub mod annotation;
