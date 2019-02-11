/// # Crossbeam使用
///
/// Basic usage: 使用标准库，出错
///
/// ```rust
/// let array = [1, 2, 3];
/// let mut guards = vec![];
///
/// // 这样也可以 for &i in &array {
/// for i in &array {
///     let guard = std::thread::spawn(move || {
///         println!("element: {}", i);
///     });
///
///     guards.push(guard);
/// }
///
/// for guard in guards {
///     guard.join().unwrap();
/// }
/// ```
///
/// Basic usage: 使用Arc修正
///
/// ```rust
/// use std::sync::Arc;
///
/// let array = Arc::new([1, 2, 3]);
/// let mut guards = vec![];
///
/// for i in 0..array.len() {
///     let a = array.clone();
///
///     let guard = std::thread::spawn(move || {
///         println!("element: {}", a[i]);
///     });
///
///     guards.push(guard);
/// }
///
/// for guard in guards {
///     guard.join().unwrap();
/// }
/// ```
///
/// Basic usage: 使用crossbeam的scoped thread
///
/// 请用 Rust 2018 Edtion
///
/// ```rust
/// use crossbeam::thread::scope;
///
/// let array = [1, 2, 3];
///
/// scope(|scope| {
///     for i in &array {
///         scope.spawn(move || {
///             println!("element: {}", i);
///         });
///     }
/// });
/// ```
/// Basic usage: 使用crossbeam channel 「0.7.1」
///
/// select!宏语法变更
///
/// 请用 Rust 2018 Edtion
///
/// ```rust
/// use crossbeam_channel::select;
/// use crossbeam_channel as channel;
///
/// use std::thread;
///
/// fn fibonacci(fib: channel::Sender<u64>, quit: channel::Receiver<()>) {
///     let (mut x, mut y) = (0, 1);
///     loop {
///         select! {
///             send(fib, x) -> _ => {
///                 let tmp = x;
///                 x = y;
///                 y = tmp + y;
///             }
///             recv(quit) -> _ => {
///                 println!("quit");
///                 return;
///             }
///         }
///     }
/// }
///
/// fn main() {
///     let (fib_s, fib_r) = channel::bounded(0);
///     let (quit_s, quit_r) = channel::bounded(0);
///
///     thread::spawn(move || {
///         for _ in 0..10 {
///             println!("{}", fib_r.recv().unwrap());
///         }
///         quit_s.send(());
///     });
///
///     fibonacci(fib_s, quit_r);
/// }
/// ```
///
///
/// Basic usage: 使用crossbeam channel 「已过期」
///
/// 请用 Rust 2018 Edtion
///
/// ```rust
/// use crossbeam_channel::select;
/// use crossbeam_channel as channel;
///
/// use std::thread;
///
/// fn fibonacci(fib: channel::Sender<u64>, quit: channel::Receiver<()>) {
///     let (mut x, mut y) = (0, 1);
///     loop {
///         select! {
///             send(fib, x) => {
///                 let tmp = x;
///                 x = y;
///                 y = tmp + y;
///             }
///             recv(quit) => {
///                 println!("quit");
///                 return;
///             }
///         }
///     }
/// }
///
/// fn main() {
///     let (fib_s, fib_r) = channel::bounded(0);
///     let (quit_s, quit_r) = channel::bounded(0);
///
///     thread::spawn(move || {
///         for _ in 0..10 {
///             println!("{}", fib_r.recv().unwrap());
///         }
///         quit_s.send(());
///     });
///
///     fibonacci(fib_s, quit_r);
/// }
/// ```
///
/// Basic usage: 使用crossbeam channel，通过引用共享
///
/// 请用 Rust 2018 Edtion
///
/// ```rust
/// use crossbeam::channel as channel;
/// fn main(){
/// let (s, r) = channel::unbounded();
///
/// crossbeam::scope(|scope| {
///     // Spawn a thread that sends one message and then receives one.
///     scope.spawn(|| {
///         s.send(1);
///         r.recv().unwrap();
///     });
///
///     // Spawn another thread that does the same thing.
///     scope.spawn(|| {
///         s.send(2);
///         r.recv().unwrap();
///     });
/// });
/// }
/// ```
///
/// Basic usage: 使用crossbeam channel，通过clone共享
///
/// 请用 Rust 2018 Edtion
///
/// ```rust
/// use std::thread;
/// use crossbeam::channel as channel;
///
/// fn main(){
/// let (s1, r1) = channel::unbounded();
/// let (s2, r2) = (s1.clone(), r1.clone());
///
/// // Spawn a thread that sends one message and then receives one.
/// thread::spawn(move || {
///     s1.send(1);
///     r1.recv().unwrap();
/// });
///
/// // Spawn another thread that receives a message and then sends one.
/// thread::spawn(move || {
///     r2.recv().unwrap();
///     s2.send(2);
/// });
/// }
/// ```
pub fn crossbeam_demo() {
    unimplemented!();
}
