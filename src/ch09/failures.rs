/// # 消除失败
///
/// 1. 利用类型系统
/// 2. 利用断言
///
/// Base usage: 利用类型系统消除程序中的失败
///
/// ```rust
/// fn sum(a: i32, b: i32) -> i32 {
///     a + b
/// }
/// fn main() {
///     sum(1u32, 2u32); // 违反契约，报错
/// }
/// ```
///
/// Base usage: 利用断言消除运行时failure
///
/// ```rust
/// fn main() {
///     let mut vec = vec![1, 2, 3];
///     vec.insert(1, 4);
///     assert_eq!(vec, [1, 4, 2, 3]);
///     vec.insert(4, 5);
///     assert_eq!(vec, [1, 4, 2, 3, 5]);
///     // vec.insert(8, 8);
/// }
/// ```
pub fn eliminate_failure(){
    unimplemented!();
}
