/// # Unsafe Rust介绍
///
/// - unsafe关键字和unsafe块
/// - 什么情况下需要用unsafe？ 
///     - 什么情况下函数需要标记unsafe ?
///     - 什么情况下trait需要标记unsafe ？
///     - 什么情况下需要使用unsafe块 ？
/// 
/// Basic usage: Unsafe中依旧会进行借用检查
///
///
/// `error[E0502]`: cannot borrow `a` as mutable because it is also borrowed as immutable
/// 
/// ```
/// fn main(){
///     unsafe {
///         let mut a = "hello";
///         let b = &a;
///         let c = &mut a;
///     }
/// }
/// ```
///
/// Basic usage: Unsafe块示意
/// 
/// ```
/// fn main() {
///     let hello = vec![104, 101, 108, 108, 111];
///     let hello = unsafe {
///         // 该函数为unsafe函数
///         String::from_utf8_unchecked(hello)
///     };
///     assert_eq!("hello", hello);
/// }
/// ```
///
/// Basic usage: 访问和修改可变静态变量 
/// 
/// ```rust
/// static mut COUNTER: u32 = 0;
/// fn main() {
///     let inc = 3;
///     unsafe {
///         COUNTER += inc;
///         println!("COUNTER: {}", COUNTER);
///     }
/// }
/// ```
pub fn unsafe_intro(){
    unimplemented!();
}

/// # Union联合体
///
/// 也叫共用体、Untagged Union
/// 
/// Basic usage: 使用Union和Struct来模拟Enum - V1
///
/// Error: 当前Union不支持Non-Copy类型
/// 
/// ```rust
/// // #![feature(untagged_unions)]
/// #[repr(C)]
/// union U {
///     i: i32,
///     f: f32,
/// }
/// #[repr(C)]
/// struct Value{
///     tag: u8,
///     value: U,
/// }
/// #[repr(C)]
/// union MyZero {
///    i: Value,
///    f: Value,
/// }
/// enum MyEnumZero {
///     I(i32),
///     F(f32),
/// }
/// fn main(){
///    let int_0 = MyZero{i: Value{tag: b'0', value: U { i: 0 } } };
///    let float_0 = MyZero{i: Value{tag: b'1', value: U { f: 0.0 } } };
/// }
/// ```
/// 
/// Basic usage: 使用Union和Struct来模拟Enum - V2
///
/// 
/// ```rust
/// #[repr(u32)]
/// enum Tag { I, F }
/// #[repr(C)]
/// union U {
///     i: i32,
///     f: f32,
/// }
/// #[repr(C)]
/// struct Value {
///    tag: Tag,
///    u: U,
/// }
/// fn is_zero(v: Value) -> bool {
///    unsafe {
///        match v {
///            Value { tag: Tag::I, u: U { i: 0 } } => true,
///            Value { tag: Tag::F, u: U { f: 0.0 } } => true,
///            _ => false,
///        }
///    }
/// }
/// fn main() {
///    let int_0 = Value{tag: Tag::I, u: U{i: 0}};
///    let float_0 = Value{tag: Tag::F, u: U{f: 0.0}};
///    assert_eq!(true, is_zero(int_0));
///    assert_eq!(true, is_zero(float_0));
///    assert_eq!(4, std::mem::size_of::<U>());
/// }
/// ```
/// 
/// Basic usage: 访问Union中未初始化的字段
///
/// 虽然未报错，但该用法不安全
/// 
/// ```rust
/// #[repr(C)]
/// union U {
///     i: i32,
///     f: f32,
/// }
/// fn main() {
///     let u = U{i: 1};
///     let i =unsafe{ 
///        u.f
///    };
///    // 0.000000000000000000000000000000000000000000001
///    println!("{}", i);
///    // 对于一个联合体来说，不能同时使用两个字段
///    // 当然也不能同时出借两个字段的可变借用
///    // unsafe{ 
///    //     let i = &mut u.i;
///    //     let f = &mut u.f;
///    // };
/// }
/// ```
pub fn union_demo(){
    unimplemented!();
}