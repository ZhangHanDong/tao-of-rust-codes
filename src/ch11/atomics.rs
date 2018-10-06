/// # 原子类型
///
/// Basic usage: 使用原子类型实现自旋锁
///
/// ```rust
/// use std::sync::Arc;
/// use std::sync::atomic::{AtomicUsize, Ordering};
/// use std::thread;
/// fn main() {
///     let spinlock = Arc::new(AtomicUsize::new(1));
///     let spinlock_clone = spinlock.clone();
///     let thread = thread::spawn(move|| {
///         spinlock_clone.store(0, Ordering::SeqCst);
///     });
///    while spinlock.load(Ordering::SeqCst) != 0 {}
///    if let Err(panic) = thread.join() {
///        println!("Thread had an error: {:?}", panic);
///    }
/// }
/// ```
pub fn atomic_demo(){
    unimplemented!();
}

