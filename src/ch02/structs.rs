/// # 具名结构体定义
///
/// Basic usage:
///
/// ```
/// #[derive(Debug, PartialEq)]
/// struct People {
///     name:  &'static str,
///     gender: u32,
/// }
/// impl People {
///     fn new(name: &'static str, gender: u32) -> Self{
///         return People{name: name, gender: gender};
///     }
///
///     fn name(&self) {
///         println!("name: {:?}", self.name);
///     }
///
///     fn set_name(&mut self, name: &'static str)  {
///         self.name = name;
///     }
///
///     fn gender(&self){
///       let gender = if (self.gender == 1) {"boy"} else {"girl"};
///       println!("gender: {:?}", gender);
///     }
/// }
/// let alex = People::new( "Alex", 1);
/// alex.name();
/// alex.gender();
/// assert_eq!(alex, People { name: "Alex", gender: 1 });
///
/// let mut alice = People::new("Alice", 0);
/// alice.name();
/// alice.gender();
/// assert_eq!(alice, People { name: "Alice", gender: 0 });
/// alice.set_name("Rose");
/// alice.name();
/// assert_eq!(alice, People { name: "Rose", gender: 0 });
/// ```
#[derive(Debug, PartialEq)]
pub struct People {
    name:  &'static str,
    gender: u32,
}
impl People {
    fn new(name: &'static str, gender: u32) -> Self{
        return People{name: name, gender: gender};
    }
    fn name(&self) {
        println!("name: {:?}", self.name);
    }
    fn set_name(&mut self, name: &'static str)  {
        self.name = name;
    }
    fn gender(&self){
      let gender = if (self.gender == 1) {"boy"} else {"girl"};
      println!("gender: {:?}", gender);
    }
}


/// # 元组结构体定义
///
/// Basic usage:
///
/// ```
/// struct Color(i32, i32, i32);
/// let color = Color(0, 1, 2);
/// assert_eq!(color.0, 0);
/// assert_eq!(color.1, 1);
/// assert_eq!(color.2, 2);
/// ```
pub struct Color(i32, i32, i32);


/// # 元组结构体：NewType模式
///
/// Basic usage:
///
/// ```
/// struct Integer(u32);
/// type Int = i32;  // 为i32类型创建别名Int
/// let int = Integer(10);
/// assert_eq!(int.0, 10);
/// let int: Int = 10;
/// assert_eq!(int, 10);
/// ```
pub struct Integer(u32);


/// # 单元结构体
///
/// Basic usage:
///
/// ```
/// struct Empty; // 等价于  struct Empty {}
/// struct RangeFull;  // 标准库源码中RangeFull就是一个单元结构体
/// assert_eq!((..), std::ops::RangeFull); //  RangeFull就是(..)，表示全范围
/// let x = Empty;
/// println!("{:p}", &x as *const _);
/// let y = x;
/// println!("{:p}", &y as *const _);
/// let z = Empty;
/// println!("{:p}", &z as *const _);
/// ```
pub struct Empty;
