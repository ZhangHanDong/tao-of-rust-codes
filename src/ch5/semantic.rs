/// # RAII自动资源管理
///
/// Base usage: 等价C++代码
/// ```c++
/// #include <iostream>
/// #include <memory>
/// using namespace std;
/// int main ()
/// {
///     unique_ptr<int> orig(new int(5));
///     cout << *orig << endl;
///     auto stolen = move(orig);
///     cout << *orig << endl;
/// }
/// ```
///
/// Base usage: 等价Rust代码
/// ```
/// fn main() {
///     let orig = Box::new(5);
///     println!("{}", *orig);
///     let stolen = orig;  // orig 会move，因为是移动语义
///     println!("{}", *orig);
/// }
/// ```
pub fn raii_demo(){
    fn main() {
        let orig = Box::new(5);
        println!("{}", *orig);
        let stolen = orig;
        // println!("{}", *orig);
    }

}

/// # 值语义 == Copy语义 | 引用语义 == Move语义
///
/// Base usage: 简单类型就是值语义，也就是Copy语义，因为可以安全在栈上进行复制
/// ```
/// let x = 5;
/// let y = x;
/// assert_eq!(x, 5);
/// assert_eq!(y, 5);
/// ```
///
/// Base usage: 智能指针一般是引用语义，也就是Move语义，因为不能进行安全栈复制
///  以下会报错
/// ```
/// // error[E0204]: the trait `Copy` may not be implemented for this type
/// #[derive(Copy, Clone)]
/// struct A{
///     a: i32,
///     b: Box<i32>,
/// }
/// fn main(){}
/// ```
pub fn value_semantic(){
    let x = 5;
    let y = x;
    assert_eq!(x, 5);
    assert_eq!(y, 5);
}

/// # 所有权是否转移
///
/// Base usage: 结构体默认没有实现Copy
/// ```
/// #[derive(Debug)]
/// struct A{
///     a: i32,
///     b: u32,
/// }
/// fn main(){
///     let a = A {a: 1, b: 2};
///     let b = a;  // a的所有权会转移
///     println!("{:?}", a);
/// }
/// ```
///
/// Base usage: 手工为结构体实现Copy
/// ```
/// #[derive(Debug, Copy, Clone)]
/// struct A{
///     a: i32,
///     b: u32,
/// }
/// fn main(){
///     let a = A {a: 1, b: 2};
///     let b = a;  // a的所有权没有转移，因为这里是复制语义
///     println!("{:?}", a);
/// }
/// ```
///
/// Base usage: 元组默认实现Copy
/// ```
/// fn main(){
///     let a = ("a".to_string(), "b".to_string());
///     let b = a;  // String是移动语义，所以此处a会移动
///     // println!("{:?}", a);
///     let c = (1,2,3); // 此处为复制语义
///     let d = c;
///     println!("{:?}", c);
/// }
/// ```
pub fn ownership_change(){
    #[derive(Debug, Copy, Clone)]
    struct A{
        a: i32,
        b: u32,
    }
    fn main(){
        let a = A {a: 1, b: 2};
        let b = a;
        println!("{:?}", a);
    }

}
