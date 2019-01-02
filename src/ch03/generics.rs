/// # 泛型
///
/// Base usage: 自定义泛型函数
///
/// ```
/// fn foo<T>(x: T) -> T {
///    return x;
/// }
///
/// assert_eq!(foo(1), 1);
/// assert_eq!(foo("hello"), "hello");
/// ```
///
/// Base usage: 自定义泛型结构体
///
/// ```
/// struct Point<T> {  x: T, y: T }
/// ```
///
/// Base usage: `foo<T>`静态分发等价于下面代码
///
/// ```
/// fn foo_1(x: i32) -> i32 {
///     return x;
/// }
/// fn foo_2(x: &'static str) -> &'static str {
///     return x;
/// }
/// foo_1(1);
/// foo_2("2");
/// ```
pub fn foo<T>(x: T) -> T {
   return x;
}

/// # 泛型: 为泛型结构体实现方法
///
/// Base usage:
///
/// ```
/// fn impl_method(){
///     #[derive(Debug, PartialEq)]
///     struct Point<T> {x: T, y: T}
///     impl<T> Point<T> {
///         fn new(x: T, y: T) -> Self{
///             Point{x: x, y: y}
///         }
///     }
///     let point1 = Point::new(1, 2);
///     let point2 = Point::new("1", "2");
///     assert_eq!(point1, Point{x: 1, y: 2});
///     assert_eq!(point2, Point{x: "1", y: "2"});
/// }
/// impl_method();
/// ```
pub fn impl_method(){
    #[derive(Debug, PartialEq)]
    struct Point<T> {x: T, y: T}
    impl<T> Point<T> {
        fn new(x: T, y: T) -> Self{
            Point{x: x, y: y}
        }
    }
    let point1 = Point::new(1, 2);
    let point2 = Point::new("1", "2");
    assert_eq!(point1, Point{x: 1, y: 2});
    assert_eq!(point2, Point{x: "1", y: "2"});
}

/// # 泛型: 返回值自动推导
///
/// Base usage:
///
/// ```
/// fn  infer_generics(){
///     #[derive(Debug, PartialEq)]
///     struct Foo(i32);
///     #[derive(Debug, PartialEq)]
///     struct Bar(i32, i32);
///     trait Inst {
///         fn new(i: i32) -> Self;
///     }
///     impl Inst for Foo {
///         fn new(i: i32) -> Foo {
///             Foo(i)
///       }
///     }
///     impl Inst for Bar {
///        fn new(i: i32) -> Bar {
///            Bar(i, i + 10)
///        }
///     }
///     fn foobar<T: Inst>(i: i32) -> T {
///       T::new(i)
///     }
///     let f: Foo = foobar(10);
///     assert_eq!(f, Foo(10));
///     let b: Bar = foobar(20);
///     assert_eq!(b, Bar(20, 30));
/// }
/// infer_generics();
/// ```
pub fn infer_generics(){
    #[derive(Debug, PartialEq)]
    struct Foo(i32);
    #[derive(Debug, PartialEq)]
    struct Bar(i32, i32);
    trait Inst {
        fn new(i: i32) -> Self;
    }
    impl Inst for Foo {
        fn new(i: i32) -> Foo {
            Foo(i)
      }
    }
    impl Inst for Bar {
       fn new(i: i32) -> Bar {
           Bar(i, i + 10)
       }
    }
    fn foobar<T: Inst>(i: i32) -> T {
      T::new(i)
    }
    let f: Foo = foobar(10);
    assert_eq!(f, Foo(10));
    let b: Bar = foobar(20);
    assert_eq!(b, Bar(20, 30));

}
