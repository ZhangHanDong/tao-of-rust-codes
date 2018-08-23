/// # 零大小类型：
///
/// Base usage:
///
/// ```
/// fn zero_size(){
///     enum Void {}
///     struct Foo;
///     struct Baz {
///         foo: Foo,
///         qux: (),
///         baz: [u8; 0],
///     }
///     assert_eq!(std::mem::size_of::<()>(), 0);
///     assert_eq!(std::mem::size_of::<Foo>(), 0);
///     assert_eq!(std::mem::size_of::<Baz>(), 0);
///     assert_eq!(std::mem::size_of::<Void>(), 0);
///     assert_eq!(std::mem::size_of::<[(); 10]>(), 0);
/// }
/// zero_size();
/// ```
pub fn zero_size(){
    use std::mem::size_of;
    enum Void {}
    struct Foo;
    struct Baz {
        foo: Foo,
        qux: (),
        baz: [u8; 0],
    }
    assert_eq!(size_of::<()>(), 0);
    assert_eq!(size_of::<Foo>(), 0);
    assert_eq!(size_of::<Baz>(), 0);
    assert_eq!(size_of::<Void>(), 0);
    assert_eq!(size_of::<[(); 10]>(), 0);
}

/// # 零大小类型：应用
///
/// Base usage: 利用单元类型查看值的类型
///
/// ```
/// let v: () = vec![(); 10]; // Error: expected (), found struct `std::vec::Vec`
/// ```
///
/// Base usage: 零大小循环
///
/// ```
/// fn zero_size_loop(){
///     let v: Vec<()> = vec![(); 10];
///     for i in v {
///         println!("{:?}", i);
///     }
/// }
/// zero_size_loop();
/// ```
pub fn zero_size_loop(){
    let v: Vec<()> = vec![(); 10];
    for i in v {
        println!("{:?}", i);
    }
}
