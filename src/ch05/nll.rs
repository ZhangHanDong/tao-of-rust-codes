/// # NLL： 非词法作用域生命周期
///
/// 以下代码目前在 Rust 2015版本中 会出错，但是选择Rust 2018 edition将正常编译
///   可以去play.rust-lang.org选择2018 edtion版本尝试
///
/// Base usage: 案例1
/// ```rust
/// struct Foo;
///
/// impl Foo {
///     fn mutate_and_share(&mut self) -> &Self { &*self }
///     fn share(&self) {}
/// }
/// let mut foo = Foo;
/// let loan = foo.mutate_and_share();
/// foo.share();
/// ```
///
/// Base usage: 案例2
/// ```rust
/// fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
///     if x.len() % 2 == 0 {
///         x
///     } else {
///         y
///     }
/// }
/// fn main(){
///     let x = String::from("hello");
///     let z;
///     let y = String::from("world");
///     z = foo(&x, &y);
///     println!("{:?}", z);
/// }
/// ```
///
/// Base usage: 案例3
/// ```rust
/// fn capitalize(data: &mut [char]) {
///     // do something
/// }
/// fn bar() {
///     let mut data = vec!['a', 'b', 'c'];
///     let slice = &mut data[..]; // <-+ 'lifetime
///     capitalize(slice);         //   |
///     data.push('d'); // ERROR!  //   |
///     data.push('e'); // ERROR!  //   |
///     data.push('f'); // ERROR!  //   |
/// } // <------------------------------+
/// ```
///
/// Base usage: 案例4，目前NLL解决不了的问题
/// ```rust
/// fn get_default<'r,K:Hash+Eq+Copy,V:Default>(map: &'r mut HashMap<K,V>,
///                                             key: K)
///                                             -> &'r mut V {
///     match map.get_mut(&key) { // -------------+ 'r
///         Some(value) => value,              // |
///         None => {                          // |
///             map.insert(key, V::default()); // |
///             //  ^~~~~~ ERROR               // |
///             map.get_mut(&key).unwrap()     // |
///         }                                  // |
///     }                                      // |
/// }                                          // v
/// ```
///
/// Base usage: 案例4修正
/// ```rust
/// fn get_default2<'r,K:Hash+Eq+Copy,V:Default>(map: &'r mut HashMap<K,V>,
///                                              key: K)
///                                              -> &'r mut V {
///     if map.contains_key(&key) {
///     // ^~~~~~~~~~~~~~~~~~ 'n
///         return match map.get_mut(&key) { // + 'r
///             Some(value) => value,        // |
///             None => unreachable!()       // |
///         };                               // v
///     }
///
///     // At this point, `map.get_mut` was never
///     // called! (As opposed to having been called,
///     // but its result no longer being in use.)
///     map.insert(key, V::default()); // OK now.
///     map.get_mut(&key).unwrap()
/// }
/// ```
///
/// Base usage: 案例5 无限循环
/// ```rust
/// struct List<T> {
///     value: T,
///     next: Option<Box<List<T>>>,
/// }
///
/// fn to_refs<T>(mut list: &mut List<T>) -> Vec<&mut T> {
///     let mut result = vec![];
///     loop {
///         result.push(&mut list.value);
///         if let Some(n) = list.next.as_mut() {
///             list = n;
///         } else {
///             return result;
///         }
///     }
/// }
/// ```
///
/// Base usage: 案例6， NLL目前还未解决
/// ```rust
/// fn main() {
///     let mut x = vec![1];
///     x.push(x.pop().unwrap());
/// }
/// ```
pub fn borrow_ck_problem(){
    fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() % 2 == 0 {
            x
        } else {
            y
        }
    }
    fn main(){
        let x = String::from("hello");
        let y = String::from("world");
        let z;
        z = foo(&x, &y);
        println!("{:?}", z);
    }

}

/// # MIR 介绍
///
/// Base usage: 用于生成MIR
/// ```rust
/// fn main(){
///     let a = "hello".to_string();
///     let s = " world";
///     a.to_string() + s;
/// }
/// ```
///
/// Base usage: 生成的MIR
/// ```rust
/// // MIR 解释
/// fn main() -> (){
///     let mut _0: ();         // 返回值
///     scope 1 {              // 第一个变量产生的顶级作用域，会包裹其他变量
///     }
///     scope 2 {             // a自己的作用域
///         let _1: i32;
///     }
///     let mut _2: i32;        // 临时值
///     let mut _3: i32;
///     let mut _4: (i32, bool);
///
///     bb0: {                 // 基础块
///         StorageLive(_1);   // 语句，代表活跃，给LLVM用来分配栈空间
///         _1 = const 1i32;  // 赋值
///
///         StorageLive(_3);
///         _3 = _1;        // 使用临时变量
///         // 执行Add操作，具有防溢出检查，
///         // 其中move代表move语义，编译器会自己判断是不是Copy
///         _4 = CheckedAdd(move _3, const 2i32);
///         // 断言，溢出时抛出错误，并且流向bb1块，此为终止句
///         assert(!move (_4.1: bool), "attempt to add with overflow") -> bb1;
///     }
///
///     bb1: {            // 基础块
///         _2 = move (_4.0: i32);     // 赋值，move为右值默认语义
///         StorageDead(_3);    // 语句，代表不活跃，给LLVM用来分配栈空间
///         StorageDead(_1);
///         return;             // 返回
///     }
/// }
/// ```
pub fn mir_explain(){
    fn main(){
        let a = "hello".to_string();
        let s = " world";
        a.to_string() + s;
    }
}
