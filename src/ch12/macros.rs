/// # 宏系统: 声明宏
///
/// Basic usage:
///
/// ```rust
/// macro_rules! unless {
///     ($arg:expr, $branch:expr) => ( if !$arg { $branch };); 
/// } 
/// fn cmp(a: i32, b: i32) {
///     unless!( a > b, {
///         println!("{} < {}", a, b);
///     });
/// }
/// fn main() {
///    let (a, b) = (1, 2);
///    cmp(a, b);
/// }
/// ```
/// 
///
/// Basic usage: 使用宏2.0语法，定义unless!宏
///
/// ```
/// #![feature(decl_macro)]
/// 
/// macro unless($arg:expr, $branch:expr) {
///     ( if !$arg { $branch }); 
/// } 
/// fn cmp(a: i32, b: i32) {
///     unless!( a > b, {
///         println!("{} < {}", a, b);
///     });
/// }
/// fn main() {
///    let (a, b) = (1, 2);
///    cmp(a, b);
/// }
/// ```
///
/// Basic usage: 如何实现hashmap!宏 V1.0
///
/// 使用 
///   
///   `cargo rustc -- -Z unstable-options --pretty=expanded`
/// 
/// 或
/// 
///   `rustc -Z unstable-options --pretty=expanded main.rs `
/// 
/// 命令查看宏展开代码
/// 
/// ```rust
/// 
/// macro_rules! hashmap {
///     ($($key:expr => $value:expr),* ) => {
///         {
///             let mut _map = ::std::collections::HashMap::new();
///             $(  
///                 _map.insert($key, $value); 
///             )*
///             _map
///         }
///     };
/// }
/// fn main(){
///     let map = hashmap!{
///         "a" => 1,
///         "b" => 2
///       //  "c" => 3, // V1.0不支持结尾有逗号
///     };
///     assert_eq!(map["a"], 1);
/// }
/// ```
/// 
/// 
/// Basic usage:  V2.0 - hashmap!递归调用消去最后键值对的结尾逗号
/// 
/// ```rust
/// macro_rules! hashmap {
///     ($($key:expr => $value:expr,)*) =>
///         {  hashmap!($($key => $value),*) };
///     ($($key:expr => $value:expr),* ) => {
///         {
///             let mut _map = ::std::collections::HashMap::new();
///             $(
///                 _map.insert($key, $value);
///             )*
///            _map
///        }
///    };
/// }
/// fn main(){
///     let map = hashmap!{
///         "a" => 1,
///         "b" => 2,
///         "c" => 3, 
///     };
///     assert_eq!(map["a"], 1);
/// }
/// ```
/// 
/// Basic usage:  V3.0 - 利用重复匹配规则
/// 
/// ```rust
/// macro_rules! hashmap {
///     ($($key:expr => $value:expr),* $(,)*) => {
///         {
///             let mut _map = ::std::collections::HashMap::new();
///             $(
///                 _map.insert($key, $value);
///             )*
///             _map
///         }
///    };
/// }
/// fn main(){
///     let map = hashmap!{
///         "a" => 1,
///         "b" => 2,
///         "c" => 3, 
///     };
///     assert_eq!(map["a"], 1);
/// }
/// ```
/// 
/// Basic usage:  V4.0 - 如何为hashmap!宏添加根据给定的键值对数量预分配空间的功能？
/// 
/// ```rust
/// macro_rules! unit {
///     ($($x:tt)*) => (());
/// }
/// macro_rules! count {
///     ($($key:expr),*) => (<[()]>::len(&[$(unit!($key)),*]));
/// }
/// macro_rules! hashmap {
///     ($($key:expr => $value:expr),* $(,)*) => {
///         {
///            let _cap = count!($($key),*);
///            let mut _map 
///                = ::std::collections::HashMap::with_capacity(_cap);
///            $(
///                _map.insert($key, $value);
///            )*
///            _map
///        }
///    };
/// }
/// fn main(){
///    let map = hashmap!{
///        "a" => 1,
///        "b" => 2,
///        "c" => 3, 
///    };
///    assert_eq!(map["a"], 1);
/// }
/// ```
/// 
/// Basic usage:  V5.0 - 重构V4.0版hashmap!宏，消除外部宏unit!和count!
/// 
/// ```rust
/// macro_rules! hashmap {
///     (@unit $($x:tt)*) => (());
///     (@count $($rest:expr),*) => 
///         (<[()]>::len(&[$(hashmap!(@unit $rest)),*]));
///     ($($key:expr => $value:expr),* $(,)*) => {
///         {
///             let _cap = hashmap!(@count $($key),*);
///             let mut _map = 
///                 ::std::collections::HashMap::with_capacity(_cap);
///            $(
///                _map.insert($key, $value);
///            )*
///            _map
///        }
///    };
/// }
/// fn main(){
///    let map = hashmap!{
///        "a" => 1,
///        "b" => 2,
///        "c" => 3, 
///    };
///    assert_eq!(map["a"], 1);
/// }
/// ```
/// 
/// 
/// Basic usage: 使用`#![feature(trace_macros)]`特性调试hashmap!宏
/// 
/// ```rust
/// #![feature(trace_macros)]
/// macro_rules! hashmap {
///     (@unit $($x:tt)*) => (());
///     (@count $($rest:expr),*) => 
///         (<[()]>::len(&[$(hashmap!(@unit $rest)),*]));
///     ($($key:expr => $value:expr),* $(,)*) => {
///         {
///             let _cap = hashmap!(@count $($key),*);
///             let mut _map = 
///                 ::std::collections::HashMap::with_capacity(_cap);
///            $(
///                _map.insert($key, $value);
///            )*
///            _map
///        }
///    };
/// }
/// fn main(){
///    trace_macros!(true);
///    let map = hashmap!{
///        "a" => 1,
///        "b" => 2,
///        "c" => 3, 
///    };
///    assert_eq!(map["a"], 1);
/// }
/// ```
/// # 宏导入导出
///
/// 请查阅源码中的相关crate
///   - hashmap_lite
pub fn declarative_macros(){
    unimplemented!();
}


/// # 宏系统: 声明宏的卫生性
///
/// Basic usage: 展示宏的卫生性
///
/// 该代码报错
/// 
/// ```
/// macro_rules! sum {
///     ($e:expr) => ({
///         let a = 2;
///         $e + a
///     })
/// }
/// fn main(){
///     // error[E0425]: cannot find value `a` in this scope
///     let four = sum!(a);
/// }
/// ```
/// 
/// 假想中sum!宏会生成的代码
/// 
/// ```rust
/// // fn main(){
/// //     let four = {
/// //         let a = 2;
/// // 	       a + a
/// // 	   };
/// // }
/// ```
/// 
/// 说明：
/// 
/// > 宏定义中的变量a和外部传入的同名a，Rust会识别它们为两个不同的变量.
/// > 宏定义中的作用域是独立的。但是目前Rust的卫生性并不完善，生命周期、类型等都无法保证卫生性。
/// 
pub fn hygienic_test(){
    unimplemented!();
}


/// # 宏系统: 过程宏
///
/// 请查阅源码中的相关crate
///   - simple_proc_macro
///   - derive-new (支持syn 0.15 和 quote 0.6)
/// 
/// 
pub fn proc_macros(){
    unimplemented!();
}

/// # 宏系统: 编译器插件
///
/// 请查阅源码中的相关crate
///   - plugin_demo
/// 
/// 
pub fn plugin_demo(){
    unimplemented!();
}