/// # NonNull指针
///
/// `NonNull<T>`旨在成为`Unsafe Rust`默认的原生指针
/// 
/// Basic usage: NonNull内置方法展示
/// 
/// 允许开发者更安全地使用原生指针
///
/// ```rust
/// use std::ptr::{null, NonNull};
/// fn main(){
///     let ptr : NonNull<i32> = NonNull::dangling();
///     println!("{:p}", ptr); // 0x4
///     let mut v = 42;
///     let ptr : Option<NonNull<i32>> = NonNull::new(&mut v);
///     println!("{:?}", ptr); // Some(0x7fff73406a78)
///     println!("{:?}", ptr.unwrap().as_ptr()); // 0x7fff73406a78
///     println!("{}", unsafe{ptr.unwrap().as_mut()});  // 42
///    let mut v = 42;
///    let ptr  = NonNull::from(&mut v);
///    println!("{:?}", ptr);  // 0x7fff73406a7c
///    let null_p: *const i32 = null();
///    let ptr  = NonNull::new(null_p as *mut i32);
///    println!("{:?}",ptr);  // None
/// }
/// ```
/// 
/// 
/// ```rust
/// use std::mem;
/// use std::ptr::NonNull;
/// struct Foo {
///     a: *mut u64,
///     b: *mut u64,
/// }
/// struct FooUsingNonNull {
///     a: *mut u64,
///     b: NonNull<*mut u64>,
/// }
/// fn main() {
///    println!("*mut u64: {} bytes", mem::size_of::<*mut u64>());
///    println!("NonNull<*mut u64>: {} bytes", 
///        mem::size_of::<NonNull<*mut u64>>());
///    println!("Option<*mut u64>: {} bytes",
///        mem::size_of::<Option<*mut u64>>());
///    println!("Option<NonNull<*mut u64>>: {} bytes", 
///        mem::size_of::<Option<NonNull<*mut u64>>>());
///    println!("Option<Foo>: {} bytes",
///        mem::size_of::<Option<Foo>>());
///    println!("Option<FooUsingNonNull>: {} bytes",
///        mem::size_of::<Option<FooUsingNonNull>>());
/// }
/// ```
pub fn non_null_intro(){
    unimplemented!();
}