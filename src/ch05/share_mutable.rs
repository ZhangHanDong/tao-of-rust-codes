/// # 可变与不可变
///
/// Base usage: 绑定默认不可变
/// ```
/// fn main() {
///     let x = "hello".to_string();
///     // cannot borrow immutable local variable `x` as mutable
///     // x += " world";
/// }
/// ```
///
/// Base usage: 结构体默认没有实现Copy
/// ```
/// fn main() {
///     let mut x = "hello".to_string();
///     x += " world";
///     assert_eq!("hello world", x);
/// }
/// ```
pub fn immut_and_mut(){
    fn main() {
        let x = "hello".to_string();
        // cannot borrow immutable local variable `x` as mutable
        // x += " world";
    }
}

/// # 词法作用域和生命周期
///
/// Base usage: let 绑定会创造词法作用域
/// ```
/// fn main(){
///     let a = "hello";   // -------scope a-------+
///     let b = "rust";    // -----scope b------+  |
///     let c = "world";   // ---scope c ----+  |  |
///     let d = c;         // - scope d ---  |  |  |
/// } //----------------------------------+--+--+--+
/// ```
///
/// Base usage: 花括号创造词法作用域
/// ```
/// fn main(){
///     let outer_val = 1;
///     let outer_sp = "hello".to_string();
///     {
///         let inner_val = 2;
///         outer_val;
///         outer_sp;
///     }
///     println!("{:?}", outer_val);
///     // error[E0425]: cannot find value `inner_val` in this scope
///     // println!("{:?}", inner_val);
///     // error[E0382]: use of moved value: `outer_sp`
///     // println!("{:?}", outer_sp);
/// }
/// ```
///
/// Base usage: match匹配的花括号创造词法作用域
/// ```
/// fn main(){
///     let a = Some("hello".to_string());
///     match a { // ------------------------------
///         Some(s) => println!("{:?}", s),      // | match scope
///         _ => println!("nothing")              // |
///    } // --------------------------------------
///    // error[E0382]: use of partially moved value: `a`
///    // println!("{:?}", a);
/// }
/// ```
///
/// Base usage: 循环表达式花括号创造词法作用域
/// ```
/// fn main(){
///     let v = vec![1,2,3];
///     for i in v { // ------------------------------------
///         println!("{:?}", i);                             // |
///         // error[E0382]: use of moved value: `v`     // | for scope
///         //  println!("{:?}", v);                         // |
///     }  //-----------------------------------------------
/// }
/// ```
///
/// Base usage: if let表达式花括号创造词法作用域
/// ```
/// fn main(){
///     let a = Some("hello".to_string());
///     if let Some(s) = a { // -------
///         println!("{:?}", s)       //|  if let scope
///     } //---------------------------
/// }
/// ```
///
/// Base usage: while let表达式花括号创造词法作用域
/// ```
/// fn main() {
///     let mut optional = Some(0);
///     while let Some(i) = optional { // ------------------ ----+
///         if i > 9 {                                   //      |
///             println!("Greater than 9, quit!");       //      |
///             optional = None;                         //      |
///         } else {                                     //      |  while scope
///             println!("`i` is `{:?}`. Try again.", i);//      |
///             optional = Some(i + 1);                  //      |
///        }                                             //      |
///     } // ----------------------------------------------------+
/// }
/// ```
///
/// Base usage: 函数花括号创造词法作用域
/// ```
/// fn foo(s: String) -> String { // ----
///     let w = " world".to_string(); // |  function scope
///     s + &w                             // |
/// } // --------------------------------
/// fn main() {
///     let s = "hello".to_string();
///     let ss = foo(s);
///  // println!("{:?}", s) // Error:  use of moved value: `s`
/// }
/// ```
///
/// Base usage: 闭包
/// ```
/// fn main() {
///     let s = "hello".to_string();
///     let join = |i: &str| {s + i}; // moved s into closure scope
///     assert_eq!("hello world", join(" world"));
///     // println!("{:?}", s); // error[E0382]: use of moved value: `s`
/// }
/// ```
pub fn lexical_scope(){
    fn main(){
        let a = "hello";   // -------scope a-------+
        let b = "rust";    // -----scope b------+  |
        let c = "world";   // ---scope c ----+  |  |
        let d = c;         // - scope d ---  |  |  |
    } //----------------------------------+--+--+--+
}
