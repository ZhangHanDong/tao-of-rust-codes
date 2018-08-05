//! 第一章：新时代的语言
//!


/// # Examples
///
/// Basic usage:
///
/// ```
/// pub fn title(){
///   println!("第1章：{}", "新时代的语言");
/// }
/// title();
/// ```
pub fn title(){
    println!("第1章：{}", "新时代的语言");
}


pub struct Duck;
pub struct Pig;

pub trait Fly {
   fn fly(&self) -> bool;
}

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
/// # 零成本抽象 fly_static
///
/// ```
/// struct Duck;
/// struct Pig;
///
/// trait Fly {
///    fn fly(&self) -> bool;
/// }
///
/// impl Fly for Duck {
///     fn fly(&self) -> bool {
///         return true;
///     }
/// }
/// impl Fly for Pig {
///    fn fly(&self) -> bool {
///        return false;
///    }
/// }
///
/// fn fly_static<T: Fly>(s: T) -> bool {
///    s.fly()
/// }
///
/// fn fly_dyn(s: &Fly) -> bool {
///     s.fly()
/// }
/// let pig = Pig;
/// assert_eq!(fly_static::<Pig>(pig), false);
/// let duck = Duck;
/// assert_eq!(fly_static::<Duck>(duck), true);
/// ```
pub fn fly_static<T: Fly>(s: T) -> bool {
   s.fly()
}

/// # 零成本抽象 fly_dyn
///
/// ```
/// struct Duck;
/// struct Pig;
///
/// trait Fly {
///    fn fly(&self) -> bool;
/// }
///
/// impl Fly for Duck {
///     fn fly(&self) -> bool {
///         return true;
///     }
/// }
/// impl Fly for Pig {
///    fn fly(&self) -> bool {
///        return false;
///    }
/// }
///
/// fn fly_static<T: Fly>(s: T) -> bool {
///    s.fly()
/// }
///
/// fn fly_dyn(s: &Fly) -> bool {
///     s.fly()
/// }
/// assert_eq!(fly_dyn(&Pig), false);
/// assert_eq!(fly_dyn(&Duck), true);
/// ```
pub fn fly_dyn(s: &Fly) -> bool {
   s.fly()
}
