//! 第九章：构建健壮的程序
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第9章：{}", "构建健壮的程序");
/// }
/// title();
/// ```
extern crate failures_crate;

pub fn title(){
    println!("第9章：{}", "构建健壮的程序");
}

pub mod failures;
pub mod errors;
pub mod panics;
