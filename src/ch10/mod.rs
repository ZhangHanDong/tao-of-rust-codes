//! 第十章：模块化开发
//!

extern crate csv_challenge;

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第10章：{}", "模块化开发");
/// }
/// title();
/// ```
pub fn title(){
    println!("第10章：{}", "模块化开发");
}

pub mod visibility;