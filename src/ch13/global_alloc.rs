/// # 全局分配器
/// 
/// - Rust 1.28之前默认内存分配器：jemalloc
/// - Rust 1.28内存分配器 : System，提供全局分配器，可自定义
/// 
/// Basic usage: 自定义全局分配器
///
/// ```
/// use std::alloc::{GlobalAlloc, System, Layout};
/// 
/// struct MyAllocator;
/// 
/// unsafe impl GlobalAlloc for MyAllocator {
///     unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
///         System.alloc(layout)
///     }
///     unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
///         System.dealloc(ptr, layout)
///     }
/// }
/// 
/// #[global_allocator]
/// static GLOBAL: MyAllocator = MyAllocator;
/// 
/// fn main() {
///    // 此处Vec的内存会由GLOBAL全局分配器来分配
///    let mut v = Vec::new();
///    v.push(1);
/// }
/// ```
/// 
/// 
/// Basic usage: 指定全局分配器为jemalloc
/// 
/// 需要引入第三方包 jemallocator
///
/// ```
/// extern crate jemallocator;
/// use jemallacator::Jemalloc;
/// #[global_allocator]
/// static GLOBAL: Jemalloc = Jemalloc;
/// ```
pub fn global_alloc(){
    unimplemented!();
}