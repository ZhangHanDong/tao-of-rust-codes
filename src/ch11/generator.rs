/// # 协程
/// 
/// Basic usage: 生成器示例1
///
/// ```rust
/// #![feature(generators, generator_trait)]
/// 
/// use std::ops::Generator;
/// 
/// fn main(){
///     
///     // let mut gen : Box<Generator<Yield=i32, Return=i32>> = Box::new(||{
///     let mut gen  = ||{
///         yield 1;
///         yield 2;
///         yield 3;
///         return 4;
///     };
///     unsafe {
///       for _ in 0..4{
///          let c = gen.resume();   
///          println!("{:?}", c);
///       }
///     }
/// }
/// ```
/// 
/// Basic usage: 生成器示例2
///
/// ```rust
/// #![feature(generators, generator_trait)]
/// 
/// use std::ops::Generator;
/// 
/// pub fn up_to(limit: u64) -> impl Generator<Yield = u64, Return = u64> {
///     move || {
/// 	for x in 0..limit {
/// 	     yield x;
/// 	}
/// 	return limit;
///     }
/// }
/// fn main(){
///     let a = 10;
///     let mut b = up_to(a);
///     unsafe {
///       for _ in 0..=10{
///          let c = b.resume();   
///          println!("{:?}", c);
///       }
///     }
/// }
/// ```
/// 
///
/// Basic usage: 生成器变为迭代器
/// 
/// yields T and returns ()
///
/// ```rust
/// #![feature(generators, generator_trait)]
/// 
/// use std::ops::Generator;
/// 
/// pub fn up_to(limit: u64) -> impl Generator<Yield = u64, Return = ()> {
///     move || {
/// 	for x in 0..limit {
/// 	     yield x;
/// 	}
/// 	return ();
///     }
/// }
/// fn main(){
///     let a = 10;
///     let mut b = up_to(a);
///     unsafe {
///       for _ in 0..=10{
///          let c = b.resume();   
///          println!("{:?}", c);
///       }
///     }
/// }
/// ```
/// 
/// 
///
/// Basic usage: 生成器变身Futures
/// 
/// yields () and returns Result<T, E> 等价于 Future of T and E
///
/// ```rust
/// #![feature(generators, generator_trait)]
/// 
/// use std::ops::Generator;
/// 
/// pub fn up_to(limit: u64) -> impl Generator<Yield = (), Return = Result<u64, ()>> {
///     move || {
/// 	for x in 0..limit {
/// 	     yield ();
/// 	}
/// 	return Ok(limit);
///     }
/// }
/// fn main(){
///     let a = 10;
///     let mut b = up_to(a);
///     unsafe {
///       for _ in 0..=10{
///          let c = b.resume();   
///          println!("{:?}", c);
///       }
///     }
/// }
/// ```
///
/// Basic usage: 跨yield借用会报错
///
/// ```rust
/// #![feature(generators, generator_trait)]
/// 
/// use std::ops::Generator;
/// 
/// pub fn up_to(limit: u64) -> impl Generator<Yield = u64, Return = u64> {
///     move || {
///         let a = 5;
///         let ref_a = &a;
/// 	for x in 0..limit {
/// 	     yield x;
/// 	     if x == 5{
/// 	         yield *ref_a;
/// 	     }
/// 	}
/// 	return limit;
///     }
/// }
/// fn main(){
///     let a = 10;
///     let mut b = up_to(a);
///     unsafe {
///       for _ in 0..=10{
///          let c = b.resume();   
///          println!("{:?}", c);
///       }
///     }
/// }
/// ```
pub fn generaotr(){
    unimplemented!();
}