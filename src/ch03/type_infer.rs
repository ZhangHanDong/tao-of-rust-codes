/// # 类型推导
///
/// Base usage: 正常推导
///
/// ```
/// fn sum(a: u32, b: i32) -> u32 {
///     a + (b as u32)
/// }
/// fn infer_demo() {
///     let a = 1;
///     let b = 2;
///     assert_eq!(sum(a, b), 3); // a和b的类型会自动推断为i32
///     let elem = 5u8;
///     let mut vec = Vec::new();
///     vec.push(elem); // vec 类型会自动推断为 u8
///     assert_eq!(vec, [5]);
/// }
/// infer_demo();
/// ```
///
/// Base usage: 无法推导的情况
///
/// ```
/// let x = "1";
/// println!("{:?}", x.parse().unwrap());
/// ```
///
/// Base usage: 解决无法推导的情况
///
/// ```
/// let x = "1";
/// println!("{:?}", x.parse::<i32>().unwrap());
/// ```
///
/// Base usage: 另一种类型无法推导的情况
///
/// ```
/// let a = 0;
/// let a_pos = a.is_positive(); // error`[E0599]`: no method named `is_positive` found for type `{integer}` in the current scope
/// ```
pub fn infer_demo() {
    let a = 1;
    let b = 2;
    assert_eq!(sum(a, b), 3);
    let elem = 5u8;
    let mut vec = Vec::new();
    vec.push(elem);
    assert_eq!(vec, [5]);
}
fn sum(a: u32, b: i32) -> u32 {
   a + (b as u32)
}
