/// # 与其他语言交互
///
/// # 与C语言交互
///
/// Basic usage: Rust中调用C标准库函数
///
/// ```rust
/// extern "C" {
///     fn isalnum(input: i32) -> i32;
/// }
/// fn main() {
///     unsafe {
///         println!("Is 3 a number ?  the answer is : {}", isalnum(3));
///         // println!("Is 'a' a number ? ", isalnum('a'));
///     }
/// }
/// ```
/// 
/// 
pub fn hello_c(){
    unimplemented!();
}

/// # 与CPP语言交互
///
/// Basic usage: 请看随书源码crate
/// 
/// - rustcallcpp， Rust中调用CPP
pub fn hello_cpp(){
    unimplemented!();
}

/// # C/Ruby/Python/Node.js中调用Rust函数
///
/// Basic usage: 请看随书源码crate
/// 
/// - callrust
pub fn hello_rust(){
    unimplemented!();
}