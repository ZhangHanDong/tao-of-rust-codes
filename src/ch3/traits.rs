/// # trait: 关联类型
///
/// Base usage:
///
/// ```
/// pub fn associated_type(){
///     #[derive(Debug, PartialEq)]
///     struct Foo(i32);
///     #[derive(Debug, PartialEq)]
///     struct Bar(i32, i32);
///     trait Inst {
///         type F;
///         type B;
///         fn new_foo(i: i32) -> Self::F;
///         fn new_bar(i: i32) -> Self::B;
///     }
///     struct FooBar;
///     impl Inst for FooBar {
///         type F = Foo;
///         type B = Bar;
///         fn new_foo(i: i32) -> Foo {
///             Foo(i)
///         }
///         fn new_bar(i: i32) -> Bar {
///             Bar(i, i + 10)
///         }
///     }
///     let f: Foo = FooBar::new_foo(10);
///     assert_eq!(f, Foo(10));
///     let b: Bar = FooBar::new_bar(20);
///     assert_eq!(b, Bar(20, 30));
/// }
/// infer_generics();
/// ```
pub fn associated_type(){
    #[derive(Debug, PartialEq)]
    struct Foo(i32);
    #[derive(Debug, PartialEq)]
    struct Bar(i32, i32);
    trait Inst {
        type F;
        type B;
        fn new_foo(i: i32) -> Self::F;
        fn new_bar(i: i32) -> Self::B;
    }
    struct FooBar;
    impl Inst for FooBar {
        type F = Foo;
        type B = Bar;
        fn new_foo(i: i32) -> Foo {
            Foo(i)
        }
        fn new_bar(i: i32) -> Bar {
            Bar(i, i + 10)
        }
    }
    let f: Foo = FooBar::new_foo(10);
    assert_eq!(f, Foo(10));
    let b: Bar = FooBar::new_bar(20);
    assert_eq!(b, Bar(20, 30));
}


/// # trait: 泛型trait
///
/// Base usage:
///
/// ```
/// fn generics_trait(){
///     trait Add<RHS, Output> {
///         fn my_add(self, rhs: RHS) -> Output;
///     }
///     impl Add<i32, i32> for  i32 {
///         fn my_add(self, rhs: i32) -> i32 {
///             self + rhs
///         }
///     }
///     impl Add<u32, i32> for  u32 {
///         fn my_add(self, rhs: u32) -> i32 {
///             (self + rhs ) as i32
///         }
///     }
///     let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
///     let x: i32 = a.my_add(b);
///     let y: i32 = c.my_add(d);
///     assert_eq!(x, 3i32);
///     assert_eq!(y, 7i32);
/// }
/// generics_trait();
/// ```
pub fn generics_trait(){
    trait Add<RHS, Output> {
        fn my_add(self, rhs: RHS) -> Output;
    }
    impl Add<i32, i32> for  i32 {
        fn my_add(self, rhs: i32) -> i32 {
            self + rhs
        }
    }
    impl Add<u32, i32> for  u32 {
        fn my_add(self, rhs: u32) -> i32 {
            (self + rhs ) as i32
        }
    }
    let (a, b, c, d) = (1i32, 2i32, 3u32, 4u32);
    let x: i32 = a.my_add(b);
    let y: i32 = c.my_add(d);
    assert_eq!(x, 3i32);
    assert_eq!(y, 7i32);
}

/// # trait: 关联类型String用法
///
/// 说明：
/// 在Rust标准库中，`String`类型实现`Add trait`，使用了关联类型。
/// 如下所示：
/// ```
/// impl Add<&str> for String {
///     type Output = String;
///     fn add(mut self, other: &str) -> String {
///         self.push_str(other);
///         self
///     }
/// }
/// ```
/// Base usage:
///
/// ```
/// let a = "hello";
/// let b = " world";
/// let c = a.to_string() + b;
/// println!("{:?}", c); // "hello world"
/// ```
pub fn string_add(){
    let a = "hello";
    let b = " world";
    let c = a.to_string() + b;
    println!("{:?}", c); // "hello world"
}


/// # trait 一致性
///
/// Base usage: 错误的方式，违反孤儿规则
///
/// ```
/// use std::ops::Add;
/// impl Add<u64> for u32{
///     type Output = u64;
///     fn add(self, other: u64) -> Self::Output {
///         (self as u64) + other
///     }
/// }
/// let a = 1u32;
/// let b = 2u64;
/// assert_eq!(a+b, 3);
/// ```
///
/// Base usage: 正确的方式之重新定义trait
///
/// ```
/// trait Add<RHS=Self> {
/// type Output;
///     fn add(self, rhs: RHS) -> Self::Output;
/// }
/// impl Add<u64> for u32{
///     type Output = u64;
///     fn add(self, other: u64) -> Self::Output {
///         (self as u64) + other
///     }
/// }
/// let a = 1u32;
/// let b = 2u64;
/// assert_eq!(a.add(b), 3);
/// ```
///
/// Base usage: 正确的方式之重新定义类型
///
/// ```
/// use std::ops::Add;
/// #[derive(Debug)]
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// impl Add for Point {
///     type Output = Point;
///     fn add(self, other: Point) -> Point {
///         Point {
///             x: self.x + other.x,
///             y: self.y + other.y,
///         }
///    }
/// }
///
/// // Point { x: 3, y: 3 }
/// println!("{:?}", Point { x: 1, y: 0 } + Point { x: 2, y: 3 });
/// ```
pub fn override_op(){
    trait Add<RHS=Self> {
    type Output;
        fn add(self, rhs: RHS) -> Self::Output;
    }
    impl Add<u64> for u32{
        type Output = u64;
        fn add(self, other: u64) -> Self::Output {
            (self as u64) + other
        }
    }
    let a = 1u32;
    let b = 2u64;
    assert_eq!(a.add(b), 3);
}

/// # trait继承
///
/// Base usage:
///
/// ```
/// trait Page{
///     fn set_page(&self, p: i32){
///         println!("Page Default: 1");
///     }
/// }
/// trait PerPage{
///     fn set_perpage(&self, num: i32){
///         println!("Per Page Default: 10");
///     }
/// }
/// trait Paginate: Page + PerPage{
///     fn set_skip_page(&self, num: i32){
///         println!("Skip Page : {:?}", num);
///     }
/// }
/// impl <T: Page + PerPage>Paginate for T{}
/// struct MyPaginate{ page: i32 }
/// impl Page for MyPaginate{}
/// impl PerPage for MyPaginate{}
/// let my_paginate = MyPaginate{page: 1};
/// my_paginate.set_page(2);
/// my_paginate.set_perpage(100);
/// my_paginate.set_skip_page(12);
/// ```
pub fn trait_inherit(){
    trait Page{
        fn set_page(&self, p: i32){
            println!("Page Default: 1");
        }
    }
    trait PerPage{
        fn set_perpage(&self, num: i32){
            println!("Per Page Default: 10");
        }
    }
    trait Paginate: Page + PerPage{
        fn set_skip_page(&self, num: i32){
            println!("Skip Page : {:?}", num);
        }
    }
    impl <T: Page + PerPage>Paginate for T{}

    struct MyPaginate{ page: i32 }
    impl Page for MyPaginate{}
    impl PerPage for MyPaginate{}
    let my_paginate = MyPaginate{page: 1};
    my_paginate.set_page(2);
    my_paginate.set_perpage(100);
    my_paginate.set_skip_page(12);
}

/// # trait bound 泛型约束
///
/// Base usage:
///
/// ```
/// use std::ops::Add;
/// fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T{
///     a + b
/// }
///
/// assert_eq!(sum(1u32, 2u32), 3);
/// assert_eq!(sum(1u64, 2u64), 3);
/// ```
pub fn trait_bound(){
    use std::ops::Add;
    fn sum<T: Add<T, Output=T>>(a: T, b: T) -> T{
        a + b
    }
    assert_eq!(sum(1u32, 2u32), 3);
    assert_eq!(sum(1u64, 2u64), 3);
}
