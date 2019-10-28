/// # 函数定义
///
/// Basic usage:
///
/// ```
/// pub fn fizz_buzz(num: i32) -> String {
///     if num % 15 == 0 {
///         return "fizzbuzz".to_string();
///     } else if num % 3 == 0 {
///         return "fizz".to_string();
///     } else if num % 5 == 0 {
///         return "buzz".to_string();
///     } else {
///         return num.to_string();
///     }
/// }
/// assert_eq!(fizz_buzz(15), "fizzbuzz".to_string());
/// assert_eq!(fizz_buzz(3), "fizz".to_string());
/// assert_eq!(fizz_buzz(5), "buzz".to_string());
/// assert_eq!(fizz_buzz(13), "13".to_string());
/// ```
pub fn fizz_buzz(num: i32) -> String {
    if num % 15 == 0 {
        return "fizzbuzz".to_string();
    } else if num % 3 == 0 {
        return "fizz".to_string();
    } else if num % 5 == 0 {
        return "buzz".to_string();
    } else {
        return num.to_string();
    }
}


/// # 词法作用域
///
/// Basic usage:
///
/// ```
/// pub fn lexical_scope(){
///     let v = "hello world!";
///     assert_eq!(v, "hello world!");
///     let v = "hello Rust!";
///     assert_eq!(v, "hello Rust!");
///     {
///         let v = "hello World!";
///         assert_eq!(v, "hello World!");
///     }
///     assert_eq!(v, "hello Rust!");
/// }
/// lexical_scope();
/// ```
pub fn lexical_scope(){
    let v = "hello world!";
    assert_eq!(v, "hello world!");
    let v = "hello Rust!";
    assert_eq!(v, "hello Rust!");
    {
        let v = "hello World!";
        assert_eq!(v, "hello World!");
    }
    assert_eq!(v, "hello Rust!");
}


/// # 函数指针： 函数作为参数
///
/// Basic usage:
///
/// ```
/// pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32{
///     op(a, b)
/// }
/// fn sum(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// fn product(a: i32, b: i32) -> i32 {
///     a * b
/// }
///
/// let a = 2;
/// let b = 3;
/// assert_eq!(math(sum, a, b), 5);
/// assert_eq!(math(product, a, b), 6);
/// ```
pub fn math(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32{
    op(a, b)
}
fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn product(a: i32, b: i32) -> i32 {
    a * b
}

fn is_true() -> bool { true }
/// # 函数指针 : 函数作为返回值
///
/// Basic usage:
///
/// ```
/// fn is_true() -> bool { true }
/// fn true_maker() -> fn() -> bool { is_true }
/// assert_eq!(true_maker()(), true);
/// ```
pub fn true_maker() -> fn() -> bool { is_true }


/// # CTFE: const fn
///
/// Basic usage:
///
/// ```
/// // #![feature(const_fn)]
/// const fn init_len() -> usize {
///     return 5;
/// }
/// [0; init_len()];
/// ```
pub const fn init_len() -> usize {
    return 5;
}

/// # 闭包
///
/// Basic usage:
///
/// ```
/// pub fn closure(){
///     let out = 42;
///     // 普通函数
///     // fn  add(i: i32, j: i32) -> i32 { i + j + out}
///     fn  add(i: i32, j: i32) -> i32 { i + j }
///     // 定义类型标注的闭包
///     let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out};
///     // 如果没有类型标注则由编译器推断类型
///     let closure_inferred  = |i, j| i + j + out;
///     let i = 1;
///     let j = 2;
///     assert_eq!(3, add(i, j));
///     assert_eq!(45, closure_annotated(i, j));
///     assert_eq!(45, closure_inferred(i, j));
/// }
/// closure();
/// ```
pub fn closure(){
    let out = 42;
    // fn  add(i: i32, j: i32) -> i32 { i + j + out}
    fn  add(i: i32, j: i32) -> i32 { i + j }
    let closure_annotated = |i: i32, j: i32| -> i32 { i + j + out};
    let closure_inferred  = |i, j| i + j + out;
    let i = 1;
    let j = 2;
    assert_eq!(3, add(i, j));
    assert_eq!(45, closure_annotated(i, j));
    assert_eq!(45, closure_inferred(i, j));
}

/// # 闭包: 作为参数
///
/// Basic usage:
///
/// ```
/// pub fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
///     op()
/// }
/// let a = 2;
/// let b = 3;
/// assert_eq!(closure_math(|| a + b), 5);
/// assert_eq!(closure_math(|| a * b), 6);
/// ```
pub fn closure_math<F: Fn() -> i32>(op: F) -> i32 {
    op()
}

/// # 闭包: 作为返回值（动态分发）
///
/// Basic usage:
///
/// ```
/// fn two_times() -> Box<Fn(i32) -> i32> {
///     let i = 2;
///     Box::new(move |j| j * i)
/// }
/// let result = two_times();
/// assert_eq!(result(2), 4);
/// ```
pub fn two_times() -> Box<Fn(i32) -> i32> {
    let i = 2;
    Box::new(move |j| j * i)
}

/// # 闭包: 作为返回值（动态分发）Rust 2018
///
/// Basic usage:
///
/// ```
/// fn two_times_dyn() -> Box<dyn Fn(i32) -> i32> {
///     let i = 2;
///     Box::new(move |j| j * i)
/// }
/// let result = two_times_dyn();
/// assert_eq!(result(2), 4);
/// ```
pub fn two_times_dyn() -> Box<dyn Fn(i32) -> i32> {
    let i = 2;
    Box::new(move |j| j * i)
}



/// # 闭包: 作为返回值（静态分发）
///
/// Basic usage:
///
/// ```
/// fn two_times_impl() -> impl Fn(i32) -> i32{
///     let i = 2;
///     move |j| j * i
/// }
/// let result = two_times_impl();
/// assert_eq!(result(2), 4);
/// ```
pub fn two_times_impl() -> impl Fn(i32) -> i32 {
    let i = 2;
    move |j| j * i
}
