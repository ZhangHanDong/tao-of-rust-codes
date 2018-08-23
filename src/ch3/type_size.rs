/// # 动态大小类型：str
/// ### 探索&str的组成
///
/// Basic usage:
///
/// ```
/// fn str_compose(){
///     let str = "Hello Rust";
///     let ptr = str.as_ptr();
///     let len = str.len();
///     println!("{:p}", ptr); // 0x555db4b96c00
///     println!("{:?}", len); // 10
/// }
/// str_compose();
/// ```
pub fn str_compose(){
    let str = "Hello Rust";
    let ptr = str.as_ptr();
    let len = str.len();
    println!("{:p}", ptr); // 0x555db4b96c00
    println!("{:?}", len); // 10
}

/// # 动态大小类型：`[T]`
/// ### 探索数组
///
/// Error usage:
///
/// ```
/// // Error: `[u32]` does not have a constant size known at compile-time
/// fn reset(mut arr: [u32]) {
///     arr[0] = 5;
///     arr[1] = 4;
///     arr[2] = 3;
///     arr[3] = 2;
///     arr[4] = 1;
///     println!("reset arr {:?}", arr);
/// }
/// let arr: [u32] = [1, 2, 3, 4, 5];
/// reset(arr);
/// println!("origin arr {:?}", arr);
/// ```
///
/// Right usage 1: 指定固定长度
///
/// ```
/// fn reset(mut arr: [u32; 5]) {
///     arr[0] = 5;
///     arr[1] = 4;
///     arr[2] = 3;
///     arr[3] = 2;
///     arr[4] = 1;
///     println!("reset arr {:?}", arr);
/// }
/// let arr: [u32; 5] = [1, 2, 3, 4, 5];
/// reset(arr);
/// println!("origin arr {:?}", arr);
/// ```
///
/// Right usage 2: 使用胖指针
///
/// ```
/// fn reset(arr: &mut[u32]) {
///     arr[0] = 5;
///     arr[1] = 4;
///     arr[2] = 3;
///     arr[3] = 2;
///     arr[4] = 1;
///     println!("reset arr {:?}", arr);
/// }
/// let mut arr = [1, 2, 3, 4, 5];
/// println!("reset before : origin array {:?}", arr);
/// {
///     let mut_arr: &mut[u32] = &mut arr;
///     reset(mut_arr);
/// }
/// println!("reset after : origin array {:?}", arr);
/// ```
pub fn reset(mut arr: [u32; 5]) {
    arr[0] = 5;
    arr[1] = 4;
    arr[2] = 3;
    arr[3] = 2;
    arr[4] = 1;
    println!("reset arr {:?}", arr);
}


/// # 动态大小类型：比较`&[u32; 5]`和`&mut [u32]`的空间占用
///
/// Base usage:
///
/// ```
/// fn compare_size(){
///     assert_eq!(std::mem::size_of::<&[u32; 5]>(), 8);
///     assert_eq!(std::mem::size_of::<&mut [u32]>(), 16);
/// }
/// compare_size();
/// ```
pub fn compare_size(){
    use std::mem::size_of;
    assert_eq!(size_of::<&[u32; 5]>(), 8);
    assert_eq!(size_of::<&mut [u32]>(), 16);
}
