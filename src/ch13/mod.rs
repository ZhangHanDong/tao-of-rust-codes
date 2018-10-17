//! 第十三章：超越安全边界
//!

/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第13章：{}", "超越安全边界");
/// }
/// title();
/// ```
pub fn title(){
    println!("第13章：{}", "超越安全边界");
}

pub mod raw_pointer;
pub mod unsafe_intro;
pub mod security_abstract;
pub mod non_null_pointer;
pub mod panic_safety;
pub mod global_alloc;
pub mod ffi;
pub mod wasm;