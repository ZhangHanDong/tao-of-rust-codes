/// # 用作标签的trait：Copy
///
/// 实现Copy必须也实现Clone
///
/// Base usage:
///
/// ```
/// struct MyStruct;
/// impl Copy for MyStruct { }
/// impl Clone for MyStruct {
///     fn clone(&self) -> MyStruct {
///         *self
///     }
/// }
/// ```
///
/// Base usage: 可以用derive属性自动生成Copy
///
/// ```
/// #[derive(Copy, Clone)]
/// struct MyStruct;
/// ```
pub fn impl_clone(){
    struct MyStruct;
    impl Copy for MyStruct { }
    impl Clone for MyStruct {
        fn clone(&self) -> MyStruct {
            *self
        }
    }
}

/// # 用作标签的trait：Copy
///
/// 在Rust中一共有五种主要的标签trait：
///
/// -  Sized。用来标识编译期可确定大小的类型。
/// -  Unsize。目前该Trait为实验特性，用于标识动态大小类型（DST）。
/// -  Copy。用来标识可以按位复制其值的类型。
/// -  Send。用来标识可以跨线程安全通信的类型。
/// -  Sync。用来标识可以在线程间安全共享引用的类型。
///
/// Base usage: 检测是否实现Copy trait的函数
///
/// ```
/// fn test_copy<T: Copy>(i: T) {
///     println!("hhh");
/// }
/// let a = "String".to_string();
/// test_copy(a);
/// ```
pub fn test_copy_trait(){
    fn test_copy<T: Copy>(i: T) {
        println!("hhh");
    }
    let a = "String";
    test_copy(a);
}

/// # 用作标签的trait：Sync / Send
///
/// Base usage: error
///    利用所有权机制化解了一次并发危机
/// ```
/// use std::thread;
/// let mut x = vec![1, 2, 3, 4];
/// thread::spawn(|| {  // Error: may outlive borrowed value `x`
///     x.push(1); // 子线程修改x
/// });
/// x.push(2); // 父线程修改x
/// ```
///
/// Base usage: right
///    使用move关键字，
/// ```
/// use std::thread;
/// let mut x = vec![1, 2, 3, 4];
/// thread::spawn(move || { // x实现了Sync和Send
///     x.push(1);
/// });
/// //x.push(2); // 父线程不允许修改x
/// ```
///
/// Base usage: 未实现Sync和Send的类型示范
/// ```
/// use std::thread;
/// use std::rc::Rc;
///
/// let x = Rc::new(vec![1, 2, 3, 4]);
/// thread::spawn( move || { // Error: std::marker::Send is not satisfied
///     x[1];
/// });
/// ```
pub fn sync_send_trait(){
    use std::thread;
    let mut x = vec![1, 2, 3, 4];
    thread::spawn(move || x.push(1));
}
