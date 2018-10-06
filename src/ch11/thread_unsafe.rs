/// # 竞态条件与临界区
///
/// Basic usage: 一段线程不安全的代码
///
/// 但实际执行多次会看到不同的输出结果，基本会出现以下两种情况：
/// 
/// 1.	main主线程输出的结果中会莫名其妙少一位，并不是从0到10的连续值。
/// 2.	child子线程输出的结果和main主线程输出的结果有重复。
/// 
/// ```rust
/// use std::thread;
/// static mut V: i32 = 0;
/// fn unsafe_seq() -> i32 {
///     unsafe {
///         V += 1;
///         V
///     }
/// }
/// fn main() {
///    let child = thread::spawn(move || {
///        for _ in 0..10 {
///            unsafe_seq();
///            unsafe{println!("child : {}", V);}
///        }
///    });
///    for _ in 0..10 {
///        unsafe_seq();
///        unsafe{println!("main : {}", V);}
///    }
///    child.join().unwrap();
/// }
/// ```
pub fn unsafe_seq(){
    unimplemented!();
}
