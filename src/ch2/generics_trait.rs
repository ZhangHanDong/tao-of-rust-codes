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
/// trait Behavior {
///     fn hello(&self) -> ();
///     fn walk(&self) -> ();
/// }
/// struct Animal;
/// struct Duck(Animal);
/// struct Chicken(Animal);
/// struct Swan(Animal);
///
/// impl Behavior for Duck {
///     fn hello(&self) {
///         println!("gua gua ");
///     }
///
///     fn walk(&self) {
///         println!("swing on left and right");
///     }
/// }
///
/// impl Behavior for Chicken {
///     fn hello(&self) {
///         println!("ga ga  ");
///     }
///
///     fn walk(&self) {
///         println!("move on front and back");
///     }
/// }
///
/// impl Behavior for Swan {
///     fn hello(&self) {
///         println!("ke lu ke li ... ");
///     }
///
///     fn walk(&self) {
///         println!("Gentle walk");
///     }
/// }
///
/// let duck = Duck(Animal{});
/// duck.hello();  // gua gua
/// duck.walk();  // swing on left and right
///
/// let chicken = Chicken(Animal{});
/// chicken.hello(); // ga ga
/// chicken.walk(); // move on front and back
///
/// let swan = Swan(Animal{});
/// swan.hello(); // ke lu ke li ...
/// swan.walk(); // Gentle walk
/// ```

pub trait Behavior {
    fn hello(&self) -> ();
    fn walk(&self) -> ();
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
