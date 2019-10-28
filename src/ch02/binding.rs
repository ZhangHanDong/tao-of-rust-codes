
/// # 临时值
///
/// Basic usage:
///
/// ```
/// pub fn temp() -> i32 {
///     return 1;
/// }
/// let x = &temp();
/// temp() = *x;   // error[E0070]: invalid left-hand side expression
/// ```
pub fn temp() -> i32 {
    return 1;
}

/// # 不变与可变
///
/// Basic usage:
///
/// ```
/// pub fn immutable_and_mutable() {
///     let a = 1;  // 默认不可变
///     // a = 2; // immutable and error: cannot assign twice to immutable variable
///     let mut b = 2;  // 使用mut关键字声明可变绑定
///     b = 3; // mutable
/// }
/// immutable_and_mutable();
/// ```
pub fn immutable_and_mutable() {
    let a = 1;
    // a = 2; // immutable and error
    let mut b = 2;
    b = 3; // mutable
}

/// # 所有权
///
/// Basic usage:
///
/// ```
/// pub fn ownership(){
///     let place1 = "hello";
///     //  ^^ 位置表达式 ^^  值表达式
///     //   ^ 位置上下文  ^  值上下文
///     let place2 = "hello".to_string();
///     let other = place1;    // Copy
///                  // ^^ 位置表达式出现在了值上下文中
///     println!("{:?}", place1);  // place1还可以继续使用
///     let other = place2;    // Move
///                  // ^^ 位置表达式出现在了值上下文中
///     println!("{:?}", place2); // place2不能再被使用，编译出错
/// }
/// ownership();
/// ```
pub fn ownership() {
    let place1 = "hello";
    let place2 = "hello".to_string();
    let other = place1;
    println!("{:?}", other);
    let other = place2;
    println!("{:?}", place2); // place2 value used here after move
}

/// # 引用
///
/// Basic usage:
///
/// ```
/// pub fn reference() {
///     let a = [1,2,3];
///     let b = &a;
///     println!("{:p}", b);  // 0x7ffcbc067704
///     let mut c = vec![1,2,3];
///     let d = &mut c;
///     d.push(4);
///     println!("{:?}", d);
///     let e = &42;
///     assert_eq!(42, *e);
/// }
/// reference();
/// ```
pub fn reference() {
    let a = [1,2,3];
    let b = &a;
    println!("{:p}", b); // 0x7ffcbc067704
    let mut c = vec![1,2,3];
    let d = &mut c;
    d.push(4);
    println!("{:?}", d); // [1, 2, 3, 4]
    let e = &42;
    assert_eq!(42, *e);
}
