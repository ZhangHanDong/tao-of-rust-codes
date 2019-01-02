/// # 资源管理：
///
/// Base usage: 变量与函数
///     变量默认存储在栈中
/// ```
/// fn main() {
///     let x: i32;   // Rust会检查未初始化的变量，并报错
///     println!("{}", x);
/// }
/// ```
///
/// Base usage: if 分支检查
/// ```
/// fn main() {
///     let x: i32;
///     if true {
///         x = 1;
///     } else {    // 如果去掉else，则编译器会报错
///         x = 2;
///     }
///     println!("{}", x); // 如果去掉此行，再去掉else则不会报错，因为没有使用到x的地方，就算未初始化也没有关系
/// }
/// ```
///
/// Base usage: break会将分支中的变量返回
/// ```
/// fn main() {
///     let x: i32;
///     loop {
///         if true {
///             x = 2;
///             break;
///         }
///     }
///     println!("{}", x); // 因为break会返回分支中的变量，所以该行可以正确打印2
/// }
/// ```
///
/// Base usage: 初始化数组
/// ```
/// fn main() {
///     let a: Vec<i32> = vec![]; // 必须指定类型，因为无法做类型推断
///     let b: [i32; 0] = [];
/// }
/// ```
///
/// Base usage: 当将一个已初始化变量y绑定给另外一个变量y2时，Rust会把变量y看作是逻辑上的未初始化变量
/// ```
/// fn main() {
///     let x = 42;
///     let y = Box::new(5);
///     println!("{:p}", y); // 0x7f5ff041f008
///     let x2 = x;
///     let y2 = y;
///     // println!("{:p}", y); // y实际上已经变成了未初始化变量
/// }
/// ```
pub fn binding_and_func(){
    fn main() {
        let x: i32 = 1;
        println!("{}", x);
    }

}
