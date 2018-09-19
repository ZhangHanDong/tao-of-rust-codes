/// # 智能指针和所有权: Box<T>
///
/// Base usage: Box<T>独占所有权
///
/// ```rust
/// fn main(){
///     let x = Box::new("hello");
///     let y = x;
///     // error[E0382]: use of moved value: `x`
///     // println!("{:?}", x);
/// }
/// ```
//
/// Base usage: Box<T>可以使用解引用，并且支持解引用移动所有权操作
///
/// ```rust
/// fn main(){
///     let a = Box::new("hello");
///     let b = Box::new("Rust".to_string());
///     let c = *a;
///     let d = *b;
///     println!("{:?}", a);
///     // error[E0382]: use of moved value: `b`
///     // println!("{:?}", b);
/// }
/// ```
//
/// Base usage: Rc<T>和Arc<T>不支持解引用移动
///
/// ```rust
/// use std::rc::Rc;
/// use std::sync::Arc;
/// fn main(){
///     let r = Rc::new("Rust".to_string());
///     let a = Arc::new(vec![1.0, 2.0, 3.0]);
///     // error[E0507]: cannot move out of borrowed content
///     // let x = *r;
///     // println!("{:?}", r);
///     // error[E0507]: cannot move out of borrowed content
///    // let f = *foo;
/// }
/// ```
pub fn box_demo(){

}

/// # 智能指针和所有权: Rc<T> / Arc<T>
///
/// Base usage: 共享所有权
///
/// ```rust
/// use std::rc::Rc;
/// fn main() {
///     let x = Rc::new(45);
///     let y1 = x.clone(); // 增加强引用计数
///     let y2 = x.clone();  // 增加强引用计数
///     println!("{:?}", Rc::strong_count(&x));
///     let w =  Rc::downgrade(&x);  // 增加弱引用计数
///     println!("{:?}", Rc::weak_count(&x));
///     let y3 = &*x;         // 不增加计数
///     println!("{}", 100 - *x);
/// }
/// ```
///
/// Base usage: 使用弱引用解决循环引用内存泄漏问题
///
/// use std::rc::Rc;
/// use std::rc::Weak;
/// use std::cell::RefCell;
/// struct Node {
///     next: Option<Rc<RefCell<Node>>>,
///     head: Option<Weak<RefCell<Node>>>
/// }
/// impl Drop for Node {
///     fn drop(&mut self) {
///         println!("Dropping!");
///    }
/// }
/// fn main() {
///    let first = Rc::new(RefCell::new(Node { next: None, head: None }));
///    let second = Rc::new(RefCell::new(Node { next: None, head: None }));
///    let third = Rc::new(RefCell::new(Node { next: None, head: None }));
///    first.borrow_mut().next = Some(second.clone());
///    second.borrow_mut().next = Some(third.clone());
///    third.borrow_mut().head = Some(Rc::downgrade(&first));
/// }
/// ```
pub fn share_ownership(){

}

/// # 智能指针和所有权: 内部可变性
///    是一种可以外部不可变但是内部可变的一种容器
/// Base usage: Cell<T>
///   Cell<T>适合于实现Copy的类型，无运行时开销
///
/// ```rust
/// use std::cell::Cell;
/// struct Foo {
///     x: u32,
///     y: Cell<u32>
/// }
/// fn main(){
///     let foo = Foo { x: 1 , y: Cell::new(3)};
///     assert_eq!(1, foo.x);
///     assert_eq!(3,foo.y.get());
///    foo.y.set(5);
///    assert_eq!(5,foo.y.get());
/// }
/// ```
///
/// Base usage: RefCell<T>
///   RefCell<T>适合没有实现Copy的类型，有运行时开销，维护运行时借用检查器
///
/// ```rust
/// use std::cell::RefCell;
/// fn main(){
///     let x = RefCell::new(vec![1,2,3,4]);
///     println!("{:?}", x.borrow());
///     x.borrow_mut().push(5);
///     println!("{:?}", x.borrow());
/// }
/// ```
///
/// Base usage: RefCell<T> 违反运行时借用检查时panic
///
/// ```rust
/// use std::cell::RefCell;
/// fn main(){
///     let x = RefCell::new(vec![1,2,3,4]);
///     let mut mut_v = x.borrow_mut();
///     mut_v.push(5);
///     // thread 'main' panicked at 'already borrowed: BorrowMutError',
///     // let mut mut_v2 = x.borrow_mut();
/// }
/// ```
pub fn  inner_mutable(){

}

/// # 智能指针和所有权: Cow写时复制
///    是一个枚举类型的智能指针，包括两个可选值：Borrowed，用于包裹引用，以及Owned，用于包裹所有者
///    一般用于读多写少的场景
/// Base usage: Cow
///
/// ```rust
/// use std::borrow::Cow;
/// fn abs_all(input: &mut Cow<[i32]>) {
///     for i in 0..input.len() {
///         let v = input[i];
///         if v < 0 {
///             input.to_mut()[i] = -v;
///         }
///     }
/// }
/// fn abs_sum(ns: &[i32]) -> i32 {
///     let mut lst = Cow::from(ns);
///     abs_all(&mut lst);
///     lst.iter().fold(0, |acc, &n| acc + n)
/// }
/// fn main() {
///     // 这里没有可变需求，所以不会克隆
///     let s1 = [1,2,3];
///     let mut i1 = Cow::from(&s1[..]);
///     abs_all(&mut i1);
///     println!("IN: {:?}", s1);
///     println!("OUT: {:?}", i1);
///     // 这里有可变需求，所以会克隆
///     // 注意: 借用数据被克隆为了新的对象
///     //       s2 != i2. 实际上, s2 不可变，也不会被改变
///     let s2 = [1,2,3, -45, 5];
///     let mut i2 = Cow::from(&s2[..]);
///     abs_all(&mut i2);
///     println!("IN: {:?}", s2);
///     println!("OUT: {:?}", i2);
///     // 这里不会克隆，因为数据本身拥有所有权
///     // 注意: 在本例中，v1是本身就是可变的
///     let mut v1 = Cow::from(vec![1,2,-3,4]);
///     abs_all(&mut v1);
///     println!("IN/OUT: {:?}", v1);
///     //没有可变需求所以没有克隆
///     let s3 = [1,3,5,6];
///     let sum1 = abs_sum(&s3[..]);
///     println!("{:?}", s3);
///     println!("{}", sum1);
///     // 这里有可变需求所以这里发生了克隆
///     let s4 = [1,-3,5,-6];
///     let sum2 = abs_sum(&s4[..]);
///     println!("{:?}", s4);
///     println!("{}", sum2);
/// }
/// ```
///
/// Base usage: 利用Cow来统一实现规范
///   还可以跨线程安全传递字符串切片
/// ```rust
/// use std::borrow::Cow;
/// use std::thread;
/// #[derive(Debug)]
/// struct Token<'a> {
///     raw: Cow<'a, str>,
/// }
/// impl<'a> Token<'a> {
///     pub fn new<S>(raw: S) -> Token<'a>
///     where
///         S: Into<Cow<'a, str>>,
///    {
///         Token { raw: raw.into() }
///    }
/// }
/// fn main() {
///    let token = Token::new("abc123");
///    let token = Token::new("api.example.io".to_string());
///    thread::spawn(move || {
///        println!("token: {:?}", token);
///    }).join().unwrap();
/// }
/// ```
///
/// Base usage: 不使用Cow，字符串切片无法跨线程传递
///
/// ```rust
/// fn main() {
///     let raw = String::from("abc");
///     let s = &raw[..];
///     let token = Token::new(s);
///         thread::spawn(move || {
///         println!("token: {:?}", token);
///     }).join().unwrap();
/// }
/// ```
pub fn cow(){

}
