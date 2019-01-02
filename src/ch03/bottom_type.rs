/// # 底类型：应用
///
/// Base usage
///
/// ```
/// #![feature(never_type)]
/// fn foo() -> ! {
///     // do something
///     loop { println!("jh"); }
/// }
/// let i = if false {
///     foo();
/// } else {
///     100
/// };
/// assert_eq!(i, 100);
/// ```
pub fn bottom_type() {
    let i = if false {
       foo();
    } else {
       100
    };
    assert_eq!(i, 100);
}
fn foo() -> ! {
  // do something
  loop { println!("jh"); }
}

/// # 底类型：空Enum
///
/// Base usage：仅仅为了演示，编译并不能通过
///
/// ```
/// // 使用enum Void{}就可以避免处理Err的情况，但当前Rust并不支持该用法
/// fn void_enum() {
///     enum Void {}
///     let res: Result<u32, Void> = Ok(0);
///     let Ok(num) = res;
/// }
/// ```
pub fn void_enum() {
    // Just for show

    // enum Void {}
    // let res: Result<u32, Void> = Ok(0);
    // let Ok(num) = res;
}
