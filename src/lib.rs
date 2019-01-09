#![feature(const_fn)]
#![feature(never_type)]
#![feature(specialization)]
#![feature(unboxed_closures, fn_traits)]
//! 《Rust编程之道》
//!
//!  作者：张汉东
//!
//!  这里记录本书中涉及的所有示例代码。
//!  
//!  [源码仓库 : ZhangHanDong/tao-of-rust-codes](https://github.com/ZhangHanDong/tao-of-rust-codes)
//! <div>
//! <img src="https://wx3.sinaimg.cn/mw690/71684decly1fymj6metsoj20gf0lm44e.jpg" height="300" width="220" />
//! </div>
//! <hr/>
//!
//! 正文从 [第一章: 新时代的语言][ch1] 开始。
//!
//! Rust安装的所有细节可以在 [附录A][appendix] 中找到。
//!
//! [appendix]: ../../doc/tao_of_rust/appendix/index.html
//! [ch1]: ../../doc/tao_of_rust/ch1/index.html
//! <hr/>
//! <div>
//! <audio id="audio" controls="" preload="none">
//! <source id="mp3" src="../../../audios/感谢.mp4">
//! </audio>
//! </div>

#![doc(
       html_playground_url = "https://play.rust-lang.org/",
       test(no_crate_inject, attr(deny(warnings))),
       test(attr(allow(dead_code, deprecated, unused_variables, unused_mut)))
)]

pub mod ch01;
pub mod ch02;
pub mod ch03;
pub mod ch04;
pub mod ch05;
pub mod ch06;
pub mod ch07;
pub mod ch08;
pub mod ch09;
pub mod ch10;
pub mod ch11;
pub mod ch12;
pub mod ch13;

pub mod appendix;
