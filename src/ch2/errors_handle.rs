/// # std::result::Result 定义
///
/// Basic usage:
///
/// ```
/// let x: Result<i32, &str> = Ok(-3);
/// assert_eq!(x.is_ok(), true);
///
/// let x: Result<i32, &str> = Err("Some error message");
/// assert_eq!(x.is_ok(), false);
/// ```
///
/// ### Rust 2018 main函数可以返回Result<T, E>
///
/// ```
/// // use std::fs::File;
/// // fn main() -> Result<(), std::io::Error> {
/// //    let f = File::open("bar.txt")?;
/// //    Ok(())
/// // }
/// ```
pub enum Result<T, E> {
    Ok(T),
    Err(E),
}
