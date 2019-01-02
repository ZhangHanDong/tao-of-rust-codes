/// # 抽象类型：Box（装箱）抽象类型 之 trait对象
///
/// Base usage:
///
/// ```
/// #[derive(Debug)]
/// struct Foo;
/// trait Bar {
///     fn baz(&self);
/// }
/// impl Bar for Foo {
///     fn baz(&self) { println!("{:?}", self) }
/// }
/// fn static_dispatch<T>(t: &T) where T:Bar {
///     t.baz();
/// }
/// fn dynamic_dispatch(t: &Bar) {
///     t.baz();
/// }
/// let foo = Foo;
/// static_dispatch(&foo);
/// dynamic_dispatch(&foo);
/// ```
pub fn trait_object() {
    #[derive(Debug)]
    struct Foo;
    trait Bar {
        fn baz(&self);
    }
    impl Bar for Foo {
        fn baz(&self) { println!("{:?}", self) }
    }
    fn static_dispatch<T>(t: &T) where T:Bar {
        t.baz();
    }
    fn dynamic_dispatch(t: &Bar) {
        t.baz();
    }
    let foo = Foo;
    static_dispatch(&foo);
    dynamic_dispatch(&foo);
}


/// # 抽象类型： impl Trait unbox存在类型 （Rust 2018）
///
/// Base usage: 重构第二章中trait的示例
///
/// ```
/// use std::fmt::Debug;
/// trait Fly {
///     fn fly(&self) -> bool;
/// }
/// #[derive(Debug)]
/// struct Duck;
/// #[derive(Debug)]
/// struct Pig;
/// impl Fly for Duck {
///     fn fly(&self) -> bool {
///         return true;
///     }
/// }
/// impl Fly for Pig {
///     fn fly(&self) -> bool {
///         return false;
///     }
/// }
/// fn fly_static(s: impl Fly+Debug) -> bool {
///     s.fly()
/// }
/// fn can_fly(s: impl Fly+Debug) -> impl Fly {
///     if s.fly(){
///         println!("{:?} can fly", s);
///     }else{
///         println!("{:?} can't fly", s);
///     }
///     s
/// }
/// fn dyn_can_fly(s: impl Fly+Debug+'static) -> Box<dyn Fly> {
///     if s.fly(){
///         println!("{:?} can fly", s);
///     }else{
///         println!("{:?} can't fly", s);
///     }
///     Box::new(s)
/// }
/// let pig = Pig;
/// assert_eq!(fly_static(pig), false);  // 静态分发
/// let pig = Pig;
/// can_fly(pig);     // 静态分发
/// let duck = Duck;
/// assert_eq!(fly_static(duck), true); // 静态分发
/// let duck = Duck;
/// can_fly(duck);      // 静态分发
/// let duck = Duck;
/// dyn_can_fly(duck);   // 动态分发
/// ```
///
/// Base usage: 错误示范
///
/// ```
/// use std::ops::Add;
/// // 以下多个参数的情况，虽然同时指定了impl Add<T, Output=T>类型，
/// // 但是它们并不是同一个类型，因为这是抽象类型。
/// // 所以编译会出错： mismatched types
/// fn sum<T>(a: impl Add<T, Output=T>, b: impl Add<T, Output=T>) -> T{
///     a + b
/// }
/// ```
///
/// Base usage: 正确
///
/// ```
/// use std::ops::Add;
/// // 只能用于单个参数
/// fn hello<T>(a: impl Add<T, Output=T>) -> impl Add<T, Output=T>{
///     a
/// }
/// ```
pub fn impl_trait(){
    use std::fmt::Debug;
    pub trait Fly {
        fn fly(&self) -> bool;
    }
    #[derive(Debug)]
    struct Duck;
    #[derive(Debug)]
    struct Pig;
    impl Fly for Duck {
        fn fly(&self) -> bool {
            return true;
        }
    }
    impl Fly for Pig {
        fn fly(&self) -> bool {
            return false;
        }
    }
    fn fly_static(s: impl Fly+Debug) -> bool {
        s.fly()
    }
    fn can_fly(s: impl Fly+Debug) -> impl Fly {
        if s.fly(){
            println!("{:?} can fly", s);
        }else{
            println!("{:?} can't fly", s);
        }
        s
    }
    fn dyn_can_fly(s: impl Fly+Debug+'static) -> Box<dyn Fly> {
        if s.fly(){
            println!("{:?} can fly", s);
        }else{
            println!("{:?} can't fly", s);
        }
        Box::new(s)
    }
    let pig = Pig;
    assert_eq!(fly_static(pig), false);
    let duck = Duck;
    assert_eq!(fly_static(duck), true);

    let pig = Pig;
    can_fly(pig);
    let duck = Duck;
    can_fly(duck);

    let duck = Duck;
    dyn_can_fly(duck);
}
