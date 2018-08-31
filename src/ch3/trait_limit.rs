/// # trait局限
///
/// Base usage: 作为父crate在考虑设计trait时，不得不考虑要不要给全体的T或&'a T实现trait
///    才能不影响到子crate
/// ```
/// impl<T:Foo> Bar for T { }
/// impl<'a,T:Bar> Bar for &'a T { }
/// ```
///
/// Base usage:
///
/// ```
/// use std::ops::Add;
/// #[derive(PartialEq)]
/// struct Int(i32);
/// impl Add<i32> for Int {
///     type Output = i32;
///     fn add(self, other: i32) -> Self::Output {
///         (self.0) + other
///     }
/// }
/// // impl Add<i32> for Option<Int> {
/// //    // TODO
/// // }
/// impl Add<i32> for Box<Int> {
///     type Output = i32;
///     fn add(self, other: i32) -> Self::Output {
///         (self.0) + other
///     }
/// }
///
/// assert_eq!(Int(3) + 3, 6);
/// assert_eq!(Box::new(Int(3)) + 3, 6);
/// ```
pub fn trait_limit(){
    use std::ops::Add;
    #[derive(PartialEq)]
    struct Int(i32);
    impl Add<i32> for Int {
        type Output = i32;
        fn add(self, other: i32) -> Self::Output {
            (self.0) + other
        }
    }
    // impl Add<i32> for Option<Int> {
    //    // TODO
    // }
    impl Add<i32> for Box<Int> {
        type Output = i32;
        fn add(self, other: i32) -> Self::Output {
            (self.0) + other
        }
    }
    assert_eq!(Int(3) + 3, 6);
    assert_eq!(Box::new(Int(3)) + 3, 6);

}


/// # trait: 特化（specialization）
///
/// 重叠规则：不能为重叠的类型实现同一个trait
/// 以下代码会编译失败。这种实现方式也被称为覆盖式实现（blanket impl）
/// ```
/// impl<T> AnyTrait for T
/// impl<T> AnyTrait for T where T: Copy
/// impl<T> AnyTrait for String
/// ```
///
/// 第一个问题：性能问题
///
/// 以下代码为所有类型T实现了AddAssign，也就是 += 操作，
/// 这样实现虽然好，但是会带来性能问题，因为强制使用了clone方法，但实际上有的类型并不需要clone。
/// 而因为有重叠规则的限制，并不能为某些不需要clone的具体类型重新实现add_assign方法。
/// 所以，在标准库中，就基本上为每个具体的类型都各自实现了一遍AddAssign。
///
/// 第二个问题：代码重用
///
/// 如果没有重叠规则，则可以默认使用上面对泛型T的实现，
/// 然后对不需要clone的类型重新实现AddAssign，那么就完全没必要为每个具体类型都实现一遍add_assign方法，
/// 可以省掉很多重复代码
///
/// ```
/// impl<R, T: Add<R> + Clone> AddAssign<R> for T {
///     fn add_assign(&mut self, rhs: R) {
///         let tmp = self.clone() + rhs;
///         *self = tmp;
///     }
/// }
/// ```
///
/// Basic usage: 使用特化之trait默认实现
///
/// ```
/// #![feature(specialization)]
/// struct Diver<T> {
///     inner: T,
/// }
/// trait Swimmer {
///     fn swim(&self) {
///         println!("swimming")
///     }
/// }
/// impl<T> Swimmer for Diver<T> {}
///
/// impl Swimmer for Diver<&'static str> {
///     fn swim(&self) {
///         println!("drowning, help!")
///     }
/// }
///
/// let x = Diver::<&'static str> { inner: "Bob" };
/// x.swim();
/// let y = Diver::<String> { inner: String::from("Alice") };
/// y.swim();
/// ```
//
/// Basic usage: 使用特化之default关键字
///
/// ```
/// #![feature(specialization)]
/// struct Diver<T> {
///     inner: T,
/// }
/// trait Swimmer {
///     fn swim(&self);
/// }
/// impl<T> Swimmer for Diver<T> {
///     default fn swim(&self) {
///         println!("swimming")
///     }
/// }
///
/// impl Swimmer for Diver<&'static str> {
///     fn swim(&self) {
///         println!("drowning, help!")
///     }
/// }
///
/// let x = Diver::<&'static str> { inner: "Bob" };
/// x.swim();
/// let y = Diver::<String> { inner: String::from("Alice") };
/// y.swim();
/// ```
pub fn trait_special(){
    struct Diver<T> {
        inner: T,
    }
    trait Swimmer {
        fn swim(&self) {
            println!("swimming")
        }
    }
    impl<T> Swimmer for Diver<T> {}

    impl Swimmer for Diver<&'static str> {
        fn swim(&self) {
            println!("drowning, help!")
        }
    }

    let x = Diver::<&'static str> { inner: "Bob" };
    x.swim()
}
