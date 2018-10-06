/// # 线程管理
///
/// Basic usage: 创建线程
/// 
/// ```
/// use std::thread;
/// fn main() {
///     let mut v = vec![];
///     for id in 0..5 {
///         let child = thread::spawn(move || {
///             println!("in child: {}", id);
///         });
///         v.push(child);
///     }
///    println!("in main : join before ");
///    for child in v {
///        child.join();
///    }
///    println!("in main : join after");
/// }
/// ```
/// 
/// Basic usage: 定制线程栈的大小
/// 
/// spawn生成的线程默认栈大小是2MiB
/// 
/// ```rust
/// use std::panic;
/// use std::thread::{Builder, current};
/// fn main() {
///     let mut v = vec![];
///     for id in 0..5 {
///         let thread_name = format!("child-{}", id);
///         let size: usize = 3 * 1024;
///         let builder = Builder::new()
///             .name(thread_name).stack_size(size);
///        let child = builder.spawn(move || {
///            println!("in child: {}", id);
///            if id == 3 {
///                panic::catch_unwind(|| {
///                    panic!("oh no!");
///                });
///                println!("in {} do sm", current().name().unwrap());
///            }
///        }).unwrap();
///        v.push(child);
///    }
///    for child in v {
///       child.join().unwrap();
///    }
/// }
/// ```
/// 
/// 
/// Basic usage: 线程本地存储 TLS
/// 
/// 
/// ```rust
/// use std::cell::RefCell;
/// use std::thread;
/// fn main() {
///     thread_local!(static FOO: RefCell<u32> = RefCell::new(1));
///     FOO.with(|f| {
///         assert_eq!(*f.borrow(), 1);
///         *f.borrow_mut() = 2;
///     });
///     thread::spawn(|| {
///        FOO.with(|f| {
///            assert_eq!(*f.borrow(), 1);
///            *f.borrow_mut() = 3;
///        });
///    });
///    FOO.with(|f| {
///        assert_eq!(*f.borrow(), 2);
///    });
/// }
/// ```
/// 
/// 
/// Basic usage: park和unpark同步原语示例
/// 
/// 
/// ```rust
/// use std::thread;
/// use std::time::Duration;
/// fn main() {
///     let parked_thread = thread::Builder::new()
///         .spawn(|| {
///             println!("Parking thread");
///             thread::park();
///             println!("Thread unparked");
///         }).unwrap();
///    thread::sleep(Duration::from_millis(10));
///    println!("Unpark the thread");
///    parked_thread.thread().unpark();
///    parked_thread.join().unwrap();
/// }
/// ```
pub fn thread_management(){
    unimplemented!();
}