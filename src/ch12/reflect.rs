/// # 使用Any进行反射
///
/// Basic usage: 使用is函数进行类型判断
///
/// ```
/// use std::any::{Any, TypeId};
/// 
/// enum E { H, He, Li}
/// 
/// struct S { x: u8, y: u8, z: u16 }
/// 
/// fn main() {   
///     let v1 = 0xc0ffee_u32;
///     let v2 = E::He;
///     let v3 = S { x: 0xde, y: 0xad, z: 0xbeef };
///     let v4 = "rust";
///     let mut a: &Any; // trait object
///    a = &v1;
///    assert!(a.is::<u32>());
///    // TypeId { t: 12849923012446332737 }
///    println!("{:?}", TypeId::of::<u32>());   
///    a = &v2;
///    assert!(a.is::<E>());
///    // TypeId { t: 15527215023668350898 }
///    println!("{:?}", TypeId::of::<E>());   
///    a = &v3;
///    assert!(a.is::<S>());
///    // TypeId { t: 17868507538031848664 }
///    println!("{:?}", TypeId::of::<S>());   
///    a = &v4;
///    assert!(a.is::<&str>());
///    // TypeId { t: 1229646359891580772 }
///    println!("{:?}", TypeId::of::<&str>());   
/// }
/// ```
///
/// Basic usage: 使用downcase_ref转换到具体类型
///
/// ```
/// use std::any::Any;
/// #[derive(Debug)]
/// enum E { H, He, Li}
/// struct S { x: u8, y: u8, z: u16 }
/// fn print_any(a: &Any) {
///     if let Some(v) = a.downcast_ref::<u32>() {
///         println!("u32 {:x}", v);
///     } else if let Some(v) = a.downcast_ref::<E>() {
///         println!("enum E {:?}", v);
///    } else if let Some(v) = a.downcast_ref::<S>() {
///        println!("struct S {:x} {:x} {:x}", v.x, v.y, v.z);
///    } else {
///        println!("else!");
///    }
/// }
/// fn main() {
///    print_any(& 0xc0ffee_u32);                       
///    print_any(& E::He);                             
///    print_any(& S{ x: 0xde, y: 0xad, z: 0xbeef }); 
///    print_any(& "rust");                           
///    print_any(& "hoge");                           
/// }
/// ```
///
/// Basic usage: 使用Box<Any>
///
/// ```
/// use std::any::Any;
/// fn print_if_string(value: Box<Any>) {
///     if let Ok(string) = value.downcast::<String>() {
///         println!("String (length {}): {}", string.len(), string);
///     }else{
///         println!("Not String")
///     }
/// }
/// fn main() {
///    let my_string = "Hello World".to_string();
///    print_if_string(Box::new(my_string));
///    print_if_string(Box::new(0i8));
/// }
/// ```
///
/// Basic usage: 非静态生命周期的类型未实现Any
///
/// ```
/// use std::any::Any;
/// struct UnStatic<'a> { x: &'a i32 }
/// fn main() {
///     let a = 42;
///     let v = UnStatic { x: &a };
///     let mut any: &Any;
///     //any = &v;  // Compile Error!
/// }
/// ```
///
/// Basic usage: 使用静态生命周期类型的值创建UnStatic实例
///
/// 正常编译
/// 
/// ```
/// use std::any::Any;
/// struct UnStatic<'a> { x: &'a i32 }
/// static ANSWER: i32 = 42;
/// fn main() {
///     let v = UnStatic { x: &ANSWER };
///     let mut a: &Any;
///     a = &v;
///     assert!(a.is::<UnStatic>());
/// }
/// ```
pub fn any_function(){
    unimplemented!();
}