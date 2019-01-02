/// # 栈：
///
/// Base usage: 简单函数调用栈帧展示
///    参考图4-5
/// ```
/// fn foo(x: u32) {   // ------+
///     let y = x;     //       |  foo 函数栈帧
///     let z = 100;   //       |  其实就是foo函数作用域
/// }                  // ------+
/// fn main(){       // ------+
///     let x = 42;  //       |  main函数栈帧
///     foo(x);      //       |
/// }                // ------+
/// ```
pub fn simple_stack_frame(){

    fn foo(x: u32) {
        let y = x;
        let z = 100;
    }
    fn main(){
        let x = 42;
        foo(x);
    }

}


/// # 内存对齐：
///
/// Base usage: 结构体内存对齐
/// ```
/// struct A {
///     a: u8,
///     b: u32,
///     c: u16,
/// }
/// fn main() {
///     println!("{:?}", std::mem::size_of::<A>());   // 8
/// }             // ------+
/// ```
///
/// Base usage: Union内存对齐
/// ```
/// union U {
///     f1: u32,
///     f2: f32,
///     f3: f64
/// }
/// fn main() {
///     println!("{:?}", std::mem::size_of::<U>());  // 8
/// }
/// ```
pub fn memory_align(){
    use std::mem;
    struct A {
        a: u8,
        b: u32,
        c: u16,
    }
    fn main() {
        println!("{:?}", mem::size_of::<A>());   // 8
    }

}

/// # 复合结构内存布局
///
/// Base usage: 结构体内存对齐
/// ```
/// struct A {
///     a: u32,
///     b: Box<u64>,
/// }
/// struct B(i32, f64, char);
/// struct N;
/// enum E {
///     H(u32),
///     M(Box<u32>)
/// }
/// union U {
///     u: u32,
///     v: u64
/// }
/// fn main(){
///     println!("Box<u32>: {:?}", std::mem::size_of::<Box<u32>>());
///     println!("A: {:?}", std::mem::size_of::<A>());
///     println!("B: {:?}", std::mem::size_of::<B>());
///     println!("N: {:?}", std::mem::size_of::<N>());
///     println!("E: {:?}", std::mem::size_of::<E>());
///     println!("U: {:?}", std::mem::size_of::<U>());
/// }
/// ```
pub fn memory_layout(){
    use std::mem;
    struct A {
        a: u32,
        b: Box<u64>,
    }
    struct B(i32, f64, char);
    struct N;
    enum E {
        H(u32),
        M(Box<u32>)
    }
    union U {
        u: u32,
        v: u64
    }
    fn main(){
        println!("Box<u32>: {:?}", mem::size_of::<Box<u32>>());
        println!("A: {:?}", mem::size_of::<A>());
        println!("B: {:?}", mem::size_of::<B>());
        println!("N: {:?}", mem::size_of::<N>());
        println!("E: {:?}", mem::size_of::<E>());
        println!("U: {:?}", mem::size_of::<U>());
    }
}
