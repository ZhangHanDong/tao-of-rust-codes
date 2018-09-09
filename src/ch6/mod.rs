//!第六章：函数、闭包和迭代器
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第6章：{}", "函数、闭包和迭代器");
/// }
/// title();
/// ```
pub fn title(){
    println!("第6章：{}", "函数、闭包和迭代器");
}

pub mod functions;
pub mod closures;
pub mod iters;
