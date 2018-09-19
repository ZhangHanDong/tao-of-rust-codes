/// # 模块
/// 
/// 本章相关包：
/// 
/// - static_hashmap
/// - static_hashmap_2015
/// - static_hashmap_2018
/// - csv_challenge
/// 
/// 请参考本书随书GitHub仓库相关源码
///
/// Basic usage: 模块可见性 Rust 2015 Edtion
///
/// ```
/// pub mod outer_mod {
///     pub(self) fn outer_mod_fn() {}
///     pub mod inner_mod {
///         // use outer_mod::outer_mod_fn;
///         // 对外层模块 `outer_mod` 可见
///         pub(in outer_mod) fn outer_mod_visible_fn() {}
///         // 对整个crate可见
///         pub(crate) fn crate_visible_fn() {}
///         // `outer_mod` 内部可见
///         pub(super) fn super_mod_visible_fn() {
///             // 访问同一模块的函数
///             inner_mod_visible_fn();
///             // 访问父模块的函数需要使用“::”前缀
///             ::outer_mod::outer_mod_fn();
///         }
///         // 仅在`inner_mod`可见
///         pub(self) fn inner_mod_visible_fn() {}
///     }
///     
///     pub fn foo() {
///         inner_mod::outer_mod_visible_fn();
///         inner_mod::crate_visible_fn();
///         inner_mod::super_mod_visible_fn();
///     
///         // 不能使用inner_mod 的私有函数
///         // inner_mod::inner_mod_visible_fn();
///     }
/// }
/// fn bar() {
///     // 该函数对整个crate可见
///     outer_mod::inner_mod::crate_visible_fn();
/// 
///     // 该函数只对outer_mod可见
///     // outer_mod::inner_mod::super_mod_visible_fn();
/// 
///     // 该函数只对outer_mod可见
///     // outer_mod::inner_mod::outer_mod_visible_fn();
///     
///     // 通过foo函数调用内部细节
///     outer_mod::foo();
/// }
/// fn main() { bar() }
/// ```
/// 
///
/// Basic usage: 模块可见性 Rust 2018 Edtion
/// 
/// 请在Play手动选择Rust edition运行
///
/// ```
/// pub mod outer_mod {
///     pub(self) fn outer_mod_fn() {}
///     
///     pub mod inner_mod {
///         // 在Rust 2018 edtion 模块系统必须使用use导入
///         use crate::outer_mod::outer_mod_fn;
///         // 对外层模块 `outer_mod` 可见
///         pub(in crate::outer_mod)  fn outer_mod_visible_fn() {}
///         // 对整个crate可见
///         pub(crate) fn crate_visible_fn() {}
///         // `outer_mod` 内部可见
///         pub(super) fn super_mod_visible_fn() {
///             // 访问同一模块的函数
///             inner_mod_visible_fn();
///             // 使用use导入了outer_mod
///             outer_mod_fn();
///         }
///         // 仅在`inner_mod`可见
///         pub(self) fn inner_mod_visible_fn() {}
///     }
///     
///     pub fn foo() {
///         inner_mod::outer_mod_visible_fn();
///         inner_mod::crate_visible_fn();
///         inner_mod::super_mod_visible_fn();
///     
///         // 不能使用inner_mod 的私有函数
///         // inner_mod::inner_mod_visible_fn();
///     }
/// }
/// fn bar() {
///     // 该函数对整个crate可见
///     outer_mod::inner_mod::crate_visible_fn();
/// 
///     // 该函数只对outer_mod可见
///     // outer_mod::inner_mod::super_mod_visible_fn();
/// 
///     // 该函数只对outer_mod可见
///     // outer_mod::inner_mod::outer_mod_visible_fn();
///     
///     // 通过foo函数调用内部细节
///     outer_mod::foo();
/// }
/// fn main() { bar() }
/// ```
pub fn visibility(){
    unimplemented!();
}