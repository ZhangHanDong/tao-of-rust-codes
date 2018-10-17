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
pub fn panic_safety(){
    unimplemented!();
}