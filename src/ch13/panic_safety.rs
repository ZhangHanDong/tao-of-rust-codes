/// # 恐慌安全
///
/// Basic usage: 示例
///
///
///
/// ```rust
/// impl<T: Clone> Vec<T> {
///    fn push_all(&mut self, to_push: &[T]) {
///        self.reserve(to_push.len());
///        unsafe {
///            self.set_len(self.len() + to_push.len());
///            for (i, x) in to_push.iter().enumerate() {
///                // x.clone() 方法可能发生恐慌
///                // 所以整个push_all函数就不是恐慌安全的函数
///                // 但是出于Rust的设计，这些未初始化的内存并不会被暴露出来
///                self.ptr().offset(i as isize).write(x.clone());
///            }
///        }
///   }
///}
/// ```
///
/// # 新增恐慌不安全函数示例
///
/// ```rust
///
/// struct A(u32);
/// impl Drop for A {
///     fn drop(&mut self) {
///         println!("droping {} in A", self.0);
///     }
/// }
///
/// unsafe fn panic_unsafe(x: &mut Vec<A>, a: A) {
///     x.push(a);
///     x.reserve(10);
///     x.set_len(10); // set_len并不会预分配内存
///     panic!("panic in unsafe block");
/// }
///
/// fn main(){
///     let mut x = vec![];
///     unsafe {
///         // 该unsafe函数因为panic而暴露了未初始化内存
///         panic_unsafe(&mut x, A(42))
///     }
/// }
/// ```
///
/// # 新增： 所以恐慌不安全的函数也无法用catch_unwind捕获
///
/// 编译器会提示：`the type `&mut std::vec::Vec` may not be safely transferred across an unwind boundary`
///
/// ```rust
/// struct A(u32);
///
/// impl Drop for A {
///     fn drop(&mut self) {
///         println!("droping {} in A", self.0);
///     }
/// }
///
/// unsafe fn panic_unsafe(x: &mut Vec<A>, a: A) {
///     x.push(a);
///     x.set_len(10);
///     panic!("panic in unsafe block");
/// }
///
/// fn main(){
///     let mut x = vec![];
///     unsafe {
///         // 不能catch_unwind恐慌不安全的函数
///         let result = std::panic::catch_unwind(|| {
///             // 该unsafe函数因为panic而暴露了未初始化内存
///             panic_unsafe(&mut x, A(42))
///         });
///     }
/// }
/// ```
pub fn panic_safety() {
    unimplemented!();
}
