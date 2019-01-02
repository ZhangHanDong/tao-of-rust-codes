/// # 类型转换：自动解引用（For SmartPointer）
///
/// Base usage: `Vec<T>`实现了`Deref<Target=[T]>`
///
/// ```
/// fn foo(s: &[i32]){
///     println!("{:?}", s[0]);
/// }
///
/// let v = vec![1,2,3];
/// foo(&v)
/// ```
///
/// Base usage: `String`实现了`Deref<Target=str>`
///
/// ```
/// let a = "hello".to_string();
/// let b = " world".to_string();
/// let c = a + &b;
/// println!("{:?}", c); // "hello world"
/// ```
//
/// Base usage: `Rc<T>`实现了`Deref<Target<T>>`
///
/// ```
/// use std::rc::Rc;
/// let x = Rc::new("hello");
/// println!("{:?}", x.chars());
/// ```
pub fn auto_deref(){
    fn foo(s: &[i32]){
        println!("{:?}", s[0]);
    }

    let v = vec![1,2,3];
    foo(&v)
}

/// # 类型转换：手动解引用（For SmartPointer）
///
/// Base usage: `Rc` 和 `&str` 都实现了clone
///
/// ```
/// use std::rc::Rc;
///
/// let x = Rc::new("hello");
/// let y = x.clone();  // Rc<&str>
/// let z = (*x).clone();   // &str
/// ```
///
/// Base usage: match匹配里需要手动解引用
///
/// 将match &x改为以下5种形式任意一种即可:
/// - match x.deref()，直接调用deref方法，需要use std::ops::Deref。
/// - match x.as_ref()，String类型提供了as_ref方法来返回一个&str类型，该方法定义于AsRef trait中。
/// - match x.borrow()，方法borrow定义于Borrow trait中，行为和AsRef类型。需要use std::borrow::Borrow。
/// - match &*x，使用“解引用”操作符，将String转换为str，然后再用“引用”操作符转为&str。
/// - match &x[..]，这是因为String类型的index操作可以返回&str类型。
///
/// ```
///     let x = "hello".to_string();
///     match &x {
///         "hello" => {println!("hello")},
///         _ => {}
///     }
/// ```
pub fn manual_deref(){
    use std::rc::Rc;

    let x = Rc::new("hello");
    let y = x.clone();  // Rc<&str>
    let z = (*x).clone();   // &str
}

/// # 类型转换：as 操作符
///
/// Base usage: 无歧义完全限定语法（Fully Qualified Syntax for Disambiguation）
///  曾用名： 通用函数调用语法（UFCS）
/// ```
/// struct S(i32);
/// trait A {
///     fn test(&self, i: i32);
/// }
/// trait B {
///     fn test(&self, i: i32);
/// }
/// impl A for S {
///     fn test(&self, i: i32) {
///         println!("From A: {:?}", i);
///     }
/// }
/// impl B for S {
///     fn test(&self, i: i32) {
///         println!("From B: {:?}", i+1);
///     }
/// }
///
/// let s = S(1);
/// A::test(&s, 1);
/// B::test(&s, 1);
/// <S as A>::test(&s, 1);
/// <S as B>::test(&s, 1);
/// ```
///
/// Base usage: 父类型子类型相互转换
/// ```
/// let  a: &'static str  = "hello";  // &'static str
/// let  b: &str = a as &str; // &str
/// let  c: &'static str = b as &'static str; // &'static str
/// ```
pub fn fqsfd(){
    struct S(i32);
    trait A {
        fn test(&self, i: i32);
    }
    trait B {
        fn test(&self, i: i32);
    }
    impl A for S {
        fn test(&self, i: i32) {
            println!("From A: {:?}", i);
        }
    }
    impl B for S {
        fn test(&self, i: i32) {
            println!("From B: {:?}", i+1);
        }
    }

    let s = S(1);
    A::test(&s, 1);
    B::test(&s, 1);
    <S as A>::test(&s, 1);
    <S as B>::test(&s, 1);
}

/// # 类型转换：From和Into
///
/// Base usage: String实现了From
///
/// ```
/// let string = "hello".to_string();
/// let other_string = String::from("hello");
/// assert_eq!(string, other_string);
/// ```
///
/// Base usage: 使用into简化代码
///
/// ```
/// #[derive(Debug)]
/// struct Person{ name: String }
/// impl Person {
///     fn new<T: Into<String>>(name: T) -> Person {
///         Person {name: name.into()}
///     }
/// }
/// let person = Person::new("Alex");
/// let person = Person::new("Alex".to_string());
/// println!("{:?}", person);
/// ```
pub fn from_into(){
    let string = "hello".to_string();
    let other_string = String::from("hello");
    assert_eq!(string, other_string);
}
