/// # CSP并发模型
/// 
/// 查看随书源码中的pow库，可以查看channel实现工作量证明的示例
///
/// Basic usage: 流通道channel示例
/// 
/// 异步channel
///
/// ```rust
/// use std::thread;
/// use std::sync::mpsc::channel;
/// fn main() {
///     let (tx, rx) = channel();
///     thread::spawn(move|| {
///         tx.send(10).unwrap();
///     });
///     assert_eq!(rx.recv().unwrap(), 10);
/// }
/// ```
///
/// Basic usage: 多生产者（共享通道）
/// 
/// 异步channel
///
/// ```rust
/// use std::thread;
/// use std::sync::mpsc::channel;
/// fn main() {
///     let (tx, rx) = channel();
///     for i in 0..10 {
///         let tx = tx.clone();
///         thread::spawn(move|| {
///             tx.send(i).unwrap();
///         });
///    }
///    for _ in 0..10 {
///        let j = rx.recv().unwrap();
///        assert!(0 <= j && j < 10);
///    }
/// }
/// ```
/// 
/// Basic usage: 同步通道
/// 
///
/// ```rust
/// use std::sync::mpsc::sync_channel;
/// use std::thread;
/// fn main() {
///     let (tx, rx) = sync_channel(1);
///     tx.send(1).unwrap();
///     thread::spawn(move|| {
///         tx.send(2).unwrap();
///     });
///     assert_eq!(rx.recv().unwrap(), 1);
///    assert_eq!(rx.recv().unwrap(), 2);
/// }
/// ```
/// 
/// Basic usage: Channel死锁
/// 
/// 共享通道
///
/// ```rust
/// use std::thread;
/// use std::sync::mpsc::channel;
/// fn main() {
///     let (tx, rx) = channel();
///     for i in 0..5 {
///         let tx = tx.clone();
///         thread::spawn(move || {
///             tx.send(i).unwrap();
///         });
///    }
///    // drop(tx); // 使用drop解决死锁
///    for j in rx.iter() {
///        println!("{:?}", j);
///    }
/// }
/// ```
/// 
/// Basic usage: Channel没有死锁
/// 
/// 流通道
///
/// ```rust
/// use std::sync::mpsc::channel;
/// use std::thread;
/// fn main() {
///     let (tx, rx) = channel();
///     thread::spawn(move || {
///         tx.send(1u8).unwrap();
///         tx.send(2u8).unwrap();
///         tx.send(3u8).unwrap();
///     });
///    for x in rx.iter() {
///        println!("receive: {}", x);
///    }
/// }
/// ```
pub fn channel_demo(){
    unimplemented!();
}
