
/// # Rayon并行迭代器
///
/// Basic usage: 并行迭代器
///
/// ```rust
/// extern crate rayon;
/// use rayon::prelude::*;
/// fn sum_of_squares(input: &[i32]) -> i32 {
///     input.par_iter().map(|&i| i * i).sum()
/// }
/// fn increment_all(input: &mut [i32]) {
///     input.par_iter_mut().for_each(|p| *p += 1);
/// }
/// fn main(){
///    let v = [1,2,3,4,5,6,7,8,9,10];
///    let r = sum_of_squares(&v);
///    println!("{}", r);
///    let mut v = [1,2,3,4,5,6,7,8,9,10];
///    increment_all(&mut v);
///    println!("{:?}", v);
/// }
/// ```
///
/// Basic usage:  使用Rayon提供的join
///
/// ```rust
/// extern crate rayon;
/// fn fib(n: u32) -> u32 {
///     if n < 2 { return n; }
///     let (a, b) = rayon::join(
///         || fib(n - 1), || fib(n - 2)
///     );
///     a + b
/// }
/// fn main() {
///     let r = fib(32);
///     assert_eq!(r, 2178309);
/// }
/// ```
pub fn rayon(){
    unimplemented!();
}

/// # 协程
///
/// Basic usage: 生成器
///
/// ```rust
/// #![feature(generators, generator_trait)]
/// use std::ops::{Generator, GeneratorState};
/// fn main() {
///     let mut generator = || {
///         println!("start");
///         yield 1;
///         println!("back");
///         return "foo"
///     };
///    match generator.resume() {
///        GeneratorState::Yielded(1) => {println!("yield 1");}
///        _ => panic!("unexpected value from resume"),
///    }
///    match generator.resume() {
///        GeneratorState::Complete("foo") => {println!("return foo")}
///        _ => panic!("unexpected value from resume"),
///    }
/// }
/// ```
pub fn generator(){
    unimplemented!();
}