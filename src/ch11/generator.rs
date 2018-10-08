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
/// Basic usage: 生成器示例1中的gen生成器实例由编译器自动生成下面等价的代码
///
/// ```rust
/// #![feature(generators, generator_trait)]
/// use std::ops::{Generator, GeneratorState};
/// 
/// let mut gen = {
///     enum __Gen {
///         // (0) 初始状态
///         State0,
///         // (1) resume方法执行以后
///        State1(State1),
///        // (2) resume方法执行以后
///        State2(State2),
///        // (3) resume方法执行以后
///        State3(State3),
///        // (4) resume方法执行以后，正好完成
///        State4
///     }
/// 
///     struct State1 { x: u64 }
///     struct State2 { x: u64 }
///     struct State3 { x: u64 }
///     struct State4 { x: u64 }
/// 
///     impl Generator for __Gen {
///         type Yield = u64;
///         type Return = u64;
/// 
///         unsafe fn resume(&mut self) -> GeneratorState<u64, u64> {
///             use std::mem;
///             match mem::replace(self, __Gen::State4) {
///                 __Gen::State0 => {
///                     *self = __Gen::State1(State1{x: 1});
///                     GeneratorState::Yielded(1)
///                 }
///                 __Gen::State1(State1{x: 1}) => {
///                     *self = __Gen::State2(State2{x: 2});
///                     GeneratorState::Yielded(2)
///             }
///                 __Gen::State2(State2{x: 2}) => {
///                     *self = __Gen::State3(State3{x: 3});
///                     GeneratorState::Yielded(3)
///                 }
///                 __Gen::State3(State3{x: 3}) => {
///                     *self = __Gen::State4;
///                     GeneratorState::Complete(4)
///                 }
///                 _ => {
///                     panic!("generator resumed after completion")
///                 }
///             }
///         }
///     }
///     __Gen::State0
/// };
/// 
/// for _ in 0..4 {
///     println!("{:?}", unsafe{ gen.resume()});
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
/// 
/// 
/// 
pub fn generaotr(){
    unimplemented!();
}