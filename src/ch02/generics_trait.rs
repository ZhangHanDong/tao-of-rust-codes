/// # std::option::Option 定义
///
/// Basic usage:
///
/// ```
/// use std::fmt::Debug;
/// fn match_option<T: Debug>(o: Option<T>) {
///     match o {
///         Some(i) => println!("{:?}", i),
///         None => println!("nothing"),
///     }
/// }
/// let a: Option<i32> = Some(3);
/// let b: Option<&str> = Some("hello");
/// let c: Option<char> = Some('A');
/// let d: Option<u32>  = None;
///
/// match_option(a);  // 3
/// match_option(b);  // "hello"
/// match_option(c);  // 'A'
/// match_option(d);  // nothing
/// ```
pub enum Option<T>{
    Some(T),
    None,
}

/// # trait示例
///
/// Basic usage:
///
/// ```
/// pub trait Fly {
///     fn fly(&self) -> bool;
/// }
/// struct Duck;
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
/// fn fly_static<T: Fly>(s: T) -> bool {
///     s.fly()
/// }
/// fn fly_dyn(s: &Fly) -> bool {
///     s.fly()
/// }
/// let pig = Pig;
/// assert_eq!(fly_static::<Pig>(pig), false);
/// let duck = Duck;
/// assert_eq!(fly_static::<Duck>(duck), true);
/// assert_eq!(fly_dyn(&Pig), false);
/// assert_eq!(fly_dyn(&Duck), true);
/// ```
pub trait Fly {
    fn fly(&self) -> bool;
}

/// # impl Debug trait示例
///
/// Basic usage:
///
/// ```
/// use std::fmt::*;
/// struct Point {
///     x: i32,
///     y: i32,
/// }
/// // Debug trait中定义了fmt方法
/// impl Debug for Point {
///     fn fmt(&self, f: &mut Formatter) -> Result {
///         // 使用编译器内置的write!宏来实现
///         write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
///     }
/// }
/// let origin = Point { x: 0, y: 0 };
/// println!("The origin is: {:?}", origin);
/// ```
pub fn impl_debug_trait(){
    use std::fmt::*;
    struct Point {
        x: i32,
        y: i32,
    }
    impl Debug for Point {
        fn fmt(&self, f: &mut Formatter) -> Result {
            write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
        }
    }
    let origin = Point { x: 0, y: 0 };
    println!("The origin is: {:?}", origin);
}
