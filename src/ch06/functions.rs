/// # 函数：参数传递
///
/// Base usage: 可变参数按值传递
///
/// ```rust
/// fn modify(mut v: Vec<u32>) -> Vec<u32> {
///     v.push(42);
///     v
/// }
/// fn main(){
///     let v = vec![1,2,3];
///     let v = modify(v);
///     println!("{:?}", v);
/// }
/// ```
///
/// Base usage: 可变参数传递按引用传递
///
/// ```rust
/// fn modify(v: &mut [u32])  {
///     v.reverse();
/// }
/// fn main(){
///     let mut v = vec![1,2,3];
///     modify(&mut v);
///     println!("{:?}", v); // [3, 2, 1]
/// }
/// ```
pub fn mut_params(){
    fn modify(mut v: Vec<u32>) -> Vec<u32> {
        v.push(42);
        v
    }
    fn main(){
        let v = vec![1,2,3];
        let v = modify(v);
        println!("{:?}", v);
    }

}

/// # 函数：函数定义
///
/// Base usage: 函数屏蔽
///
/// ```rust
/// fn f() { print!("1"); }
/// fn main() {
///     f(); // 2
///     {
///         f(); // 3
///         fn f() { print!("3"); } // 只在当前块作用域有效
///     }
///     f(); // 2
///     fn f() { print!("2"); }
/// }
/// ```
///
/// Base usage: Rust 2018 利用原始标识（Raw identifiers）来将保留字作为函数名
///
/// ```rust
/// fn r#match(needle: &str, haystack: &str) -> bool {
///     haystack.contains(needle)
/// }
/// 
/// fn main() {
///     assert!(r#match("foo", "foobar"));
/// }
/// ```
pub fn function_shadow(){
    fn f() { print!("1"); }
    fn main() {
        f(); // 2
        {
            f(); // 3
            fn f() { print!("3"); }
        }
        f(); // 2
        fn f() { print!("2"); }
    }

}

/// # 函数：函数参数模式匹配
///
/// Base usage:
///
/// ```rust
/// #[derive(Debug)]
/// struct S { i: i32 }
/// fn f(ref _s: S) {
///     println!("{:p}", _s); //0x7ffdd1364b80
/// }
/// fn main() {
///     let s = S { i: 42 };
///     f(s);
///     // println!("{:?}", s);
/// }
/// ```
///
/// Base usage: 使用通配符忽略参数
///
/// ```rust
/// fn foo(_: i32) {
///     // do something
/// }
/// fn main() {
///     foo(3);
/// }
/// ```
///
/// Base usage: 模式匹配解构元组
///
/// ```rust
/// fn swap((x, y): (&str, i32)) -> (i32, &str){
///     (y, x)
/// }
/// fn main() {
///     let t = ("Alex", 18);
///     let t = swap(t);
///     assert_eq!(t, (18, "Alex"));
/// }
/// ```
pub fn function_pattern_match(){
    #[derive(Debug)]
    struct S { i: i32 }
    fn f(ref _s: S) {
        println!("{:p}", _s); //0x7ffdd1364b80
    }
    fn main() {
        let s = S { i: 42 };
        f(s);
        // println!("{:?}", s);
    }

}

/// # 函数：函数返回值
///
/// Base usage:
///
/// ```rust
/// fn addsub(x: isize, y: isize) -> (isize, isize) {
///     (x + y, x - y)
/// }
/// fn main(){
///     let (a, b) = addsub(5, 8);
///     println!("a: {:?}, b: {:?}", a, b);
/// }
/// ```
///
/// Base usage: 提前返回
///
/// 使用欧几里德算法（辗转相除法）求两数中的最大公约数
///
/// ```rust
/// fn gcd(a: u32, b: u32) -> u32 {
///     if b == 0 { return a; }
///     return gcd(b, a % b);
/// }
/// fn main(){
///     let g = gcd(60, 40);
///     assert_eq!(20, g);
/// }
/// ```
pub fn function_return(){
    fn addsub(x: isize, y: isize) -> (isize, isize) {
        (x + y, x - y)
    }
    fn main(){
        let (a, b) = addsub(5, 8);
        println!("a: {:?}, b: {:?}", a, b);
    }

}

/// # 函数：泛型函数
///
/// Base usage: 泛型函数可推断类型
///
/// ```rust
/// use std::ops::Mul;
/// fn square<T: Mul<T, Output=T>>(x: T, y: T) -> T {
///     x * y
/// }
/// fn main() {
///     let a: i32 = square(37, 41);
///     let b: f64 = square(37.2, 41.1);
///     assert_eq!(a, 1517);
///     assert_eq!(b, 1528.92); // 浮点数可能执行结果有所差别
/// }
/// ```
///
/// Base usage: 使用turbofish操作符
///
/// ```rust
/// use std::ops::Mul;
/// fn square<T: Mul<T, Output = T>>(x: T, y: T) -> T {
///     x * y
/// }
/// fn main() {
///     let a = square::<u32>(37, 41);
///     let b = square::<f32>(37.2, 41.1);
///     assert_eq!(a, 1517);
///     assert_eq!(b, 1528.9199);
/// }
/// ```
pub fn generic_function(){
    use std::ops::Mul;
    fn square<T: Mul<T, Output=T>>(x: T, y: T) -> T {
        x * y
    }
    fn main() {
        let a: i32 = square(37, 41);
        let b: f64 = square(37.2, 41.1);
        assert_eq!(a, 1517);
        assert_eq!(b, 1528.9199);
    }

}

/// # 方法与函数
///
/// Base usage: 泛型函数可推断类型
///
/// ```rust
/// #[derive(Debug)]
/// struct User {
///     name: &'static str,
///     avatar_url: &'static str,
/// }
/// impl User {
///     fn show(&self)  {
///         println!("name: {:?} ", self.name);
///         println!("avatar: {:?} ", self.avatar_url);
///     }
/// }
/// fn main() {
///     let user = User {
///         name: "Alex",
///         avatar_url: "https://avatar.com/alex"
///     };
///     // User::show(&user)
///     user.show();
/// }
/// ```
pub fn method_and_function(){
    #[derive(Debug)]
    struct User {
        name: &'static str,
        avatar_url: &'static str,
    }
    impl User {
        fn show(&self)  {
            println!("name: {:?} ", self.name);
            println!("avatar: {:?} ", self.avatar_url);
        }
    }
    fn main() {
        let user = User {
            name: "Alex",
            avatar_url: "https://avatar.com/alex"
        };
        // User::show(&user)
        user.show();
    }

}

/// # 函数：高阶函数
///
/// Base usage: 函数作为参数
///
/// ```rust
/// type MathOp = fn(i32, i32) -> i32;
/// fn math(op: MathOp, a: i32, b: i32) -> i32{
///     println!("{:p}", op);
///     op(a, b)
/// }
///
/// fn sum(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// fn product(a: i32, b: i32) -> i32 {
///     a * b
/// }
/// fn main() {
///     let (a, b) = (2, 3);
///     assert_eq!(math(sum, a, b), 5);  // 传递函数指针
///     assert_eq!(math(product, a, b), 6);
/// }
/// ```
///
/// Base usage: 函数指针
///
/// ```rust
/// fn hello(){
///     println!("hello function pointer");
/// }
/// fn main(){
///     let fn_ptr: fn() = hello;
///     println!("{:p}", fn_ptr); // 0x562bacfb9f80
///     let other_fn = hello;
///     // println!("{:p}", other_fn);  // not function pointer
///     hello();
///    other_fn();
///    fn_ptr();
///    (fn_ptr)();
/// }
/// ```
///
/// Base usage: 函数作为返回值
///
/// ```rust
/// type MathOp = fn(i32, i32) -> i32;
/// fn math(op: &str) -> MathOp {
///     fn sum(a: i32, b: i32) -> i32 {
///         a + b
///     }
///     fn product(a: i32, b: i32) -> i32 {
///         a * b
///     }
///     match op {
///         "sum" => sum,
///         "product" => product,
///         _ => {
///             println!(
///                 "Warning: Not Implemented {:?} oprator, Replace with sum",
///                 op
///             );
///             sum
///         }
///     }
/// }
/// fn main() {
///    let (a, b) = (2, 3);
///    let sum = math("sum");
///    let product = math("product");
///    let div = math("div");
///    assert_eq!(sum(a, b), 5);
///    assert_eq!(product(a, b), 6);
///    assert_eq!(div(a, b), 5);
/// }
/// ```
///
/// Base usage:  返回的函数和参与计算的参数直接绑定
///  会报错
/// ```rust
/// fn sum(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// fn product(a: i32, b: i32) -> i32 {
///     a * b
/// }
/// type MathOp = fn(i32, i32) -> i32;
/// fn math(op: &str, a: i32, b: i32) -> MathOp {
///     match op {
///        "sum" => sum(a, b),
///        _ => product(a, b)
///    }
/// }
/// fn main() {
///    let (a, b) = (2, 3);
///    let sum = math("sum", a, b);
/// }
/// ```
///
/// Base usage:  计数函数，默认加一
///
/// ```rust
/// fn counter() -> fn(i32) -> i32 {
///     fn inc(n: i32) -> i32 {
///         n + 1
///     }
///     inc
/// }
/// fn main() {
///     let f = counter();
///     assert_eq!(2, f(1));
/// }
/// ```
///
/// Base usage:  计数函数，指定默认增长值i
///
///  会出错，因为函数不能捕捉动态环境中的变量i，需要闭包才可以
/// ```rust
/// fn counter(i: i32) -> fn(i32) -> i32 {
///     fn inc(n: i32) -> i32 {
///         n + i  // error[E0434]: can't capture dynamic environment in a fn item
///     }
///     inc
/// }
/// fn main() {
///     let f = counter(2);
///     assert_eq!(3, f(1));
/// }
/// ```
pub fn higher_kind_function(){
    fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32{
        op(a, b)
    }
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }
    fn product(a: i32, b: i32) -> i32 {
        a * b
    }
    fn main() {
        let (a, b) = (2, 3);
        assert_eq!(math(sum, a, b), 5);
        assert_eq!(math(product, a, b), 6);
    }

}
