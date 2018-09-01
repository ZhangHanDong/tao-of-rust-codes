/// # 智能指针：
///
/// Base usage: 智能指针示意
/// ```
/// fn main() {
///     let s = String::from("hello");
///     // let deref_s : str = *s;
///     let v = vec![1,2,3];
///     // let deref_v: [u32] = *v;
/// }
/// ```
pub fn smart_pointer(){
    fn main() {
        let s = String::from("hello");
        // let deref_s : str = *s;
        let v = vec![1,2,3];
        // let deref_v: [u32] = *v;
    }

}

/// # RAII： 确定性析构
///
/// Base usage: 实现Drop
/// ```
/// use std::ops::Drop;
/// #[derive(Debug)]
/// struct S(i32);
/// impl Drop for S {
///     fn drop(&mut self) {
///         println!("drop {}", self.0);
///     }
/// }
/// fn main() {
///     let x = S(1);
///     println!("crate x: {:?}", x);
///     {
///         let y = S(2);
///         println!("crate y: {:?}", y);
///         println!("exit inner scope");
///     }
///     println!("exit main");
/// }
/// ```
///
/// Base usage: 配合Valgrind工具来检查是否会内存泄漏
/// 看看Box<T>是否会自动释放
/// ```
/// fn create_box() {
///     let box3 = Box::new(3);
/// }
/// fn main() {
///     let box1 = Box::new(1);
///     {
///         let box2 = Box::new(2);
///     }
///     for _ in 0..1_000 {
///        create_box();
///    }
/// }
/// ```
///
/// Base usage: 使用花括号块主动析构
/// ```
/// fn main() {
///     let mut v = vec![1, 2, 3];
///     {
///         v
///     };
///     //   v.push(4);
/// }
/// ```
///
/// Base usage: 变量遮蔽不等于drop
/// ```
/// use std::ops::Drop;
/// #[derive(Debug)]
/// struct S(i32);
/// impl Drop for S {
///     fn drop(&mut self) {
///         println!("drop for {}", self.0);
///     }
/// }
/// fn main() {
///    let x = S(1);
///    println!("create x: {:?}", x);
///    let x = S(2);
///    println!("create shadowing x: {:?}", x);
/// }
/// ```
pub fn drop_demo(){
    use std::ops::Drop;
    #[derive(Debug)]
    struct S(i32);
    impl Drop for S {
        fn drop(&mut self) {
            println!("drop {}", self.0);
        }
    }
    fn main() {
        let x = S(1);
        println!("crate x: {:?}", x);
        {
            let y = S(2);
            println!("crate y: {:?}", y);
            println!("exit inner scope");
        }
        println!("exit main");
    }
}
