/// # 使用Unsafe进行安全抽象
///
/// 安全抽象的时候需要注意什么？
///
/// ---
///
/// Basic usage:  Vec<T>的insert方法源码示意
///
/// 正因为存在下面那两个判断条件，才保证了该方法的安全性
///
/// ```rust
/// pub fn insert(&mut self, index: usize, element: T) {
///     let len = self.len();
///     // 通过该断言保证了数组不能越界
///     assert!(index <= len);
///     // 通过判断长度是否达到容量极限来决定是否进行扩容
///     if len == self.buf.cap() {
///         self.reserve(1);
///     }
///     unsafe {
///         {
///            let p = self.as_mut_ptr().offset(index as isize);
///            ptr::copy(p, p.offset(1), len - index);
///            ptr::write(p, element);
///        }
///        self.set_len(len + 1);
///    }
/// }
/// ```
pub fn contract() {
    unimplemented!();
}

/// # 子类型和型变（variance）
///
///
/// PhantomData<T>规则：
///
/// PhantomData<T>，在T上是协变。
/// PhantomData<&'a T>，在'a 和T上是协变。
/// PhantomData<&'a mut T>，在'a上是协变，在T上是不变。
/// PhantomData<*const T>，在T上是协变。
/// PhantomData<*mut T>，在T上不变。
/// PhantomData<fn(T)>，在T上是逆变，如果以后语法修改的话，会成为不变。
/// PhantomData<fn() -> T>，在T上是协变。
/// PhantomData<fn(T) -> T>，在T上是不变。
/// PhantomData<Cell<&'a ()>>，在'a上是不变。
///
/// 请配合书本中解释理解
///
/// Basic usage:  未合理使用型变将会引起未定义行为
///
/// 虽然可以正常工作，但这里存在未定义行为的风险
/// 原因是协变改变了生命周期，「忽悠」了借用检查
///
/// ```rust
/// // 协变类型
/// struct MyCell<T> {
///     value: T,
/// }
/// impl<T: Copy> MyCell<T> {
///     fn new(x: T) -> MyCell<T> {
///         MyCell { value: x }
///     }
///     fn get(&self) -> T {
///         self.value
///    }
///    fn set(&self, value: T) {
///        use std::ptr;
///        unsafe {
///            ptr::write(&self.value as *const _ as *mut _, value);
///        }
///    }
/// }
///
/// fn step1<'a>(r_c1: &MyCell<&'a i32>) {
///     let val: i32 = 13;
///     step2(&val, r_c1); // step2函数执行完再回到step1
///     println!("step1 value: {}", r_c1.value);
/// } // step1调用完，栈帧将被清理，val将不复存在，&val将成为悬垂指针
///
/// fn step2<'b>(r_val: &'b i32, r_c2: &MyCell<&'b i32>) {
///     r_c2.set(r_val);
/// }
/// static X: i32 = 10;
/// fn main() {
///    let cell = MyCell::new(&X);
///    step1(&cell);
///    println!("  end value: {}", cell.value); //此处 cell.value的值将无法预期，UB风险
/// }
/// ```
///
/// Basic usage:  修改MyCell<T> 类型为不变
///
/// 解决上面示例UB的问题，编译将报错，因为安全检查生效了，成功阻止了UB风险
///
/// ```rust
/// use std::marker::PhantomData;
/// struct MyCell<T> {
///     value: T,
///     mark: PhantomData<fn(T)> , //通过PhantomData<fn(T)>将MyCell<T>改为逆变类型
/// }
/// impl<T: Copy> MyCell<T> {
///     fn new(x: T) -> MyCell<T> {
///         MyCell { value: x , mark: PhantomData}
///     }
///    fn get(&self) -> T {
///        self.value
///    }
///    fn set(&self, value: T) {
///        use std::ptr;
///        unsafe {
///            ptr::write(&self.value as *const _ as *mut _, value);
///        }
///    }
/// }
/// fn step1<'a>(r_c1: &MyCell<&'a i32>) {
///     let val: i32 = 13;
///     step2(&val, r_c1); // error[E0597]: `val` does not live long enough
///     println!("step1 value: {}", r_c1.value);
/// } // step1调用完，栈帧将被清理，val将不复存在，&val将成为悬垂指针
///
/// fn step2<'b>(r_val: &'b i32, r_c2: &MyCell<&'b i32>) {
///     r_c2.set(r_val);
/// }
/// static X: i32 = 10;
/// fn main() {
///    let cell = MyCell::new(&X);
///    step1(&cell);
///    println!("  end value: {}", cell.value);
/// }
/// ````
///
/// Basic usage:  fn(T)逆变示意
///
/// Rust中仅存在函数指针fn(T)的逆变情况
///
/// 请配合书本解释理解
///
/// ```rust
/// trait A {
///     fn foo(&self, s: &'static str);
/// }
/// struct B;
/// impl A for B {
///     fn foo(&self, s: &str){
///         println!("{:?}", s);
///     }
/// }
/// impl B{
///    fn foo2(&self, s: &'static str){
///        println!("{:?}", s);
///    }
/// }
/// fn main() {
///    B.foo("hello");
///    // let s = "hello".to_string();
///   // B.foo2(&s)
/// }
/// ```
///
///
/// Basic usage: 另一个fn(T)逆变示意
///
/// Rust中仅存在函数指针fn(T)的逆变情况
///
/// 请配合书本解释理解
///
///
/// ```rust
/// fn foo(input: &str)  {
///     println!("{:?}", input);               
/// }
/// fn bar(f: fn(&'static str), v: &'static str) {
///     (f)(v);
/// }
/// fn main(){
///     let v : &'static str = "hello";
///     bar(foo, v);
/// }
/// ```
pub fn variances() {
    unimplemented!();
}

/// # 未绑定生命周期（Unbound Lifetime）
///
/// 未绑定生命周期（Unbound Lifetime），即可以被随意推断的生命周期，
/// 在Unsafe中很容易产生，导致跳过Rust借用检查，而产生UB风险。
///
/// Basic usage:  产生未绑定生命周期的情况
///
/// 在Debug模式下正常，但是在Release下会产生UB
///
/// ```rust
/// fn foo<'a>(input: *const u32) -> &'a u32 {
///     unsafe {
///         return &*input // 等价于&(*input)
///     }
/// }
/// fn main() {
///     let x;
///     {
///          let y = 42;
///          x = foo(&y);
///    }
///    println!("hello: {}", x);
/// }
/// ```
///
/// Basic usage:  另一种产生未绑定生命周期的情况
///
/// 在Debug模式下正常，但是在Release下会产生UB
///
/// ```rust
/// use std::mem::transmute;
/// fn main() {
///     let x: &i32;
///     {
///         let a = 12;
///         let ptr = &a as *const i32;
///         x = unsafe { transmute::<*const i32, &i32>(ptr) };
///     }
///     println!("hello {}", x);
/// }
/// ```
pub fn unbound_lifetime() {
    unimplemented!();
}

/// # Drop检查
///
/// 使用不当的话，Drop检查会引起UB
///
/// Basic usage:  声明元组变量测试dropck
///
/// 在Safe Rust中由dropck引起的问题，借用检查会安全识别，从而引起错误阻止程序编译
///
/// ```rust
/// use std::fmt;
///	#[derive(Copy, Clone, Debug)]
///	enum State { InValid, Valid }
///	#[derive(Debug)]
///	struct Hello<T: fmt::Debug>(&'static str, T, State);
///	impl<T: fmt::Debug> Hello<T> {
///	    fn new(name: &'static str, t: T) -> Self {
///	        Hello(name, t, State::Valid)
///	    }
///	}
///	impl<T: fmt::Debug> Drop for Hello<T> {
///	    fn drop(&mut self) {
///	        println!("drop Hello({}, {:?}, {:?})",
///	                 self.0,
///	                 self.1,
///	                 self.2);
///	        self.2 = State::InValid;
///	    }
///	}
///	struct WrapBox<T> {
///	    v: Box<T>,
///	}
///	impl<T> WrapBox<T> {
///	    fn new(t: T) -> Self {
///	        WrapBox { v: Box::new(t) }
///	    }
///	}
///	fn f1() {
///	   // let x; let y; // 使用此注释的x和y声明顺序即可解决问题
///	   let (x, y);
///	   x = Hello::new("x", 13);
///    // error[E0597]: `x` does not live long enough
///	   y = WrapBox::new(Hello::new("y", &x));
///	}
///	fn main() {
///	   f1();
///	}
/// ```
///
///
/// Basic usage:  `dropck` - V1
///
/// 会因为安全检查而报错，修改变量声明顺序修复问题
///
/// ```rust
/// #![feature(alloc_layout_extra)]
/// #![feature(allocator_api, dropck_eyepatch)]
/// use std::alloc::{GlobalAlloc, System, Layout};
/// use std::ptr;
/// use std::mem;
/// use std::fmt;
///	#[derive(Copy, Clone, Debug)]
///	enum State { InValid, Valid }
///	#[derive(Debug)]
///	struct Hello<T: fmt::Debug>(&'static str, T, State);
///	impl<T: fmt::Debug> Hello<T> {
///	    fn new(name: &'static str, t: T) -> Self {
///	        Hello(name, t, State::Valid)
///	    }
///	}
///	impl<T: fmt::Debug> Drop for Hello<T> {
///	    fn drop(&mut self) {
///	        println!("drop Hello({}, {:?}, {:?})",
///	                 self.0,
///	                 self.1,
///	                 self.2);
///	        self.2 = State::InValid;
///	    }
///	}
///	struct WrapBox<T> {
///	    v: Box<T>,
///	}
///	impl<T> WrapBox<T> {
///	    fn new(t: T) -> Self {
///	        WrapBox { v: Box::new(t) }
///	    }
///	}
///	fn f1() {
///	   // let x; let y; // 使用此注释的x和y声明顺序即可解决问题
///	   let (x, y);
///	   x = Hello::new("x", 13);
///    // error[E0597]: `x` does not live long enough
///	   y = WrapBox::new(Hello::new("y", &x));
///	}
///
/// struct MyBox<T> {
///     v: *const T,
/// }
/// impl<T> MyBox<T> {
///    fn new(t: T) -> Self {
///        unsafe {
///            let p = System.alloc(Layout::array::<T>(1).unwrap());
///            let p = p as *mut T;
///            ptr::write(p, t);
///            MyBox { v: p }
///        }
///    }
/// }
///
/// impl<T> Drop for MyBox<T> {
///    fn drop(&mut self) {
///        unsafe {
///            let p = self.v as *mut _;
///            System.dealloc(p,
///                Layout::array::<T>(mem::align_of::<T>()).unwrap());       
///        }
///    }
/// }
///
/// fn f2() {
///    {
///        let (x1, y1);
///        x1 = Hello::new("x1", 13);
///        y1 = MyBox::new(Hello::new("y1", &x1));
///    }
///    {
///        let (x2, y2);
///        x2 = Hello::new("x2", 13);
///        y2 = MyBox::new(Hello::new("y2", &x2));
///    }
/// }
///
///	fn main() {
///	   // f1();
///    f2();
///	}
/// ```
///
///
/// Basic usage:  `#[may_dangle]` 属性与`dropck` - V2
///
/// 通过 `#[may_dangle]`修复问题
///
/// ```rust
/// #![feature(alloc_layout_extra)]
/// #![feature(allocator_api, dropck_eyepatch)]
/// use std::alloc::{GlobalAlloc, System, Layout};
/// use std::ptr;
/// use std::mem;
/// use std::fmt;
///	#[derive(Copy, Clone, Debug)]
///	enum State { InValid, Valid }
///	#[derive(Debug)]
///	struct Hello<T: fmt::Debug>(&'static str, T, State);
///	impl<T: fmt::Debug> Hello<T> {
///	    fn new(name: &'static str, t: T) -> Self {
///	        Hello(name, t, State::Valid)
///	    }
///	}
///	impl<T: fmt::Debug> Drop for Hello<T> {
///	    fn drop(&mut self) {
///	        println!("drop Hello({}, {:?}, {:?})",
///	                 self.0,
///	                 self.1,
///	                 self.2);
///	        self.2 = State::InValid;
///	    }
///	}
///	struct WrapBox<T> {
///	    v: Box<T>,
///	}
///	impl<T> WrapBox<T> {
///	    fn new(t: T) -> Self {
///	        WrapBox { v: Box::new(t) }
///	    }
///	}
///	fn f1() {
///	   let x; let y;
///	   x = Hello::new("x", 13);
///	   y = WrapBox::new(Hello::new("y", &x));
///	}
///
/// struct MyBox<T> {
///     v: *const T,
/// }
/// impl<T> MyBox<T> {
///    fn new(t: T) -> Self {
///        unsafe {
///            let p = System.alloc(Layout::array::<T>(1).unwrap());
///            let p = p as *mut T;
///            ptr::write(p, t);
///            MyBox { v: p }
///        }
///    }
/// }
///
/// // 使用#[may_dangle]来修复问题
///  unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
///     fn drop(&mut self) {
///         unsafe {
///             let p = self.v as *mut _;
///             System.dealloc(p,
///                 Layout::array::<T>(mem::align_of::<T>()).unwrap());
///         }
///    }
/// }
///
///
/// fn f2() {
///    {
///        let (x1, y1);
///        x1 = Hello::new("x1", 13);
///        y1 = MyBox::new(Hello::new("y1", &x1));
///    }
///    {
///        let (x2, y2);
///        x2 = Hello::new("x2", 13);
///        y2 = MyBox::new(Hello::new("y2", &x2));
///    }
/// }
///
///
///	fn main() {
///	   // f1();
///    f2();
///	}
/// ```
///
///
/// Basic usage:  `#[may_dangle]` 属性与`dropck` - V3
///
/// 通过 `#[may_dangle]`修复问题，但如果在drop中使用了T，则会产生悬垂指针
///
/// ```rust
/// #![feature(alloc_layout_extra)]
/// #![feature(allocator_api, dropck_eyepatch)]
/// use std::alloc::{GlobalAlloc, System, Layout};
/// use std::ptr;
/// use std::mem;
/// use std::fmt;
///	#[derive(Copy, Clone, Debug)]
///	enum State { InValid, Valid }
///	#[derive(Debug)]
///	struct Hello<T: fmt::Debug>(&'static str, T, State);
///	impl<T: fmt::Debug> Hello<T> {
///	    fn new(name: &'static str, t: T) -> Self {
///	        Hello(name, t, State::Valid)
///	    }
///	}
///	impl<T: fmt::Debug> Drop for Hello<T> {
///	    fn drop(&mut self) {
///	        println!("drop Hello({}, {:?}, {:?})",
///	                 self.0,
///	                 self.1,
///	                 self.2);
///	        self.2 = State::InValid;
///	    }
///	}
///	struct WrapBox<T> {
///	    v: Box<T>,
///	}
///	impl<T> WrapBox<T> {
///	    fn new(t: T) -> Self {
///	        WrapBox { v: Box::new(t) }
///	    }
///	}
///	fn f1() {
///	   let x; let y;
///	   x = Hello::new("x", 13);
///	   y = WrapBox::new(Hello::new("y", &x));
///	}
///
/// struct MyBox<T> {
///     v: *const T,
/// }
/// impl<T> MyBox<T> {
///    fn new(t: T) -> Self {
///        unsafe {
///            let p = System.alloc(Layout::array::<T>(1).unwrap());
///            let p = p as *mut T;
///            ptr::write(p, t);
///            MyBox { v: p }
///        }
///    }
/// }
///
/// // 使用#[may_dangle]来修复问题
/// unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
///     fn drop(&mut self) {
/// 	    unsafe {
/// 	        ptr::read(self.v); // 此处新增
/// 	        let p = self.v as *mut _;
/// 	        System.dealloc(p,
/// 	            Layout::array::<T>(mem::align_of::<T>()).unwrap());
/// 	    }
///     }
/// }
/// fn f2() {
///    {
///        let (x1, y1);
///        x1 = Hello::new("x1", 13);
///        y1 = MyBox::new(Hello::new("y1", &x1));
///    }
///    {
///        let (y2, x2); // 此处改变
///        x2 = Hello::new("x2", 13);
///        y2 = MyBox::new(Hello::new("y2", &x2));
///    }
/// }
///
///	fn main() {
///	   // f1();
///    f2();
///	}
/// ```
///
///
///
/// Basic usage:  `#[may_dangle]`属性 与`dropck` - V4
///
/// 使用`PhantomData<T>`配合`#[may_dangle]`来得到更严格的drop检查
///
/// 标准库中Vec<T>和LinkedList<T>源码中也有相关的实践
///
/// ```rust
/// #![feature(alloc_layout_extra)]
/// #![feature(allocator_api, dropck_eyepatch)]
/// use std::alloc::{GlobalAlloc, System, Layout};
/// use std::ptr;
/// use std::mem;
/// use std::fmt;
///	use std::marker::PhantomData;
///
///	#[derive(Copy, Clone, Debug)]
///
///	enum State { InValid, Valid }
///
///	#[derive(Debug)]
///	struct Hello<T: fmt::Debug>(&'static str, T, State);
///	impl<T: fmt::Debug> Hello<T> {
///	    fn new(name: &'static str, t: T) -> Self {
///	        Hello(name, t, State::Valid)
///	    }
///	}
///	impl<T: fmt::Debug> Drop for Hello<T> {
///	    fn drop(&mut self) {
///	        println!("drop Hello({}, {:?}, {:?})",
///	                 self.0,
///	                 self.1,
///	                 self.2);
///	        self.2 = State::InValid;
///	    }
///	}
///	struct WrapBox<T> {
///	    v: Box<T>,
///	}
///	impl<T> WrapBox<T> {
///	    fn new(t: T) -> Self {
///	        WrapBox { v: Box::new(t) }
///	    }
///	}
///	fn f1() {
///	   let x; let y;
///	   x = Hello::new("x", 13);
///	   y = WrapBox::new(Hello::new("y", &x));
///	}
///
/// struct MyBox<T> {
///     v: *const T,
/// }
/// impl<T> MyBox<T> {
///    fn new(t: T) -> Self {
///        unsafe {
///            let p = System.alloc(Layout::array::<T>(1).unwrap());
///            let p = p as *mut T;
///            ptr::write(p, t);
///            MyBox { v: p }
///        }
///    }
/// }
///
/// // 使用#[may_dangle]来修复问题
/// unsafe impl<#[may_dangle] T> Drop for MyBox<T> {
///     fn drop(&mut self) {
/// 	    unsafe {
/// 	        ptr::read(self.v); // 此处新增
/// 	        let p = self.v as *mut _;
/// 	        System.dealloc(p,
/// 	            Layout::array::<T>(mem::align_of::<T>()).unwrap());
/// 	    }
///     }
/// }
/// fn f2() {
///    {
///        let (x1, y1);
///        x1 = Hello::new("x1", 13);
///        y1 = MyBox::new(Hello::new("y1", &x1));
///    }
///    {
///        let (y2, x2); // 此处改变
///        x2 = Hello::new("x2", 13);
///        y2 = MyBox::new(Hello::new("y2", &x2));
///    }
/// }
///
///	struct MyBox2<T> {
///	    v: *const T,
///	    _pd: PhantomData<T>,
///	}
///	impl<T> MyBox2<T> {
///	    fn new(t: T) -> Self {
///	        unsafe{
///	           let p = System.alloc(Layout::array::<T>(1).unwrap());
///	           let p = p as *mut T;
///	           ptr::write(p, t);
///	           MyBox2 { v: p, _pd: Default::default() }
///	       }
///	   }
///	}
///	unsafe impl<#[may_dangle] T> Drop for MyBox2<T> {
///	    fn drop(&mut self) {
///	        unsafe {
///	            ptr::read(self.v);
///	            let p = self.v as *mut _;
///	            System.dealloc(p,
///	                Layout::array::<T>(mem::align_of::<T>()).unwrap());
///	        }
///	    }
///	}
///	fn f3() {
///	   // let (y, x);
///	   // let (x, y);
///	   let x; let y;
///	   x = Hello::new("x", 13);
///	   y = MyBox2::new(Hello::new("y", &x));
///	}
///
///
///	fn main() {
///	   // f1();
///    // f2();
///    f3();
///	}
/// ```
pub fn drop_ck_test() {
    unimplemented!();
}

/// # 使用`std::mem::forget`跳过Drop
///
///
/// Basic usage: 转移结构体中字段所有权 - V1
///
///
/// ```rust
///	struct A;
///	struct B;
///	struct Foo {
///	    a: A,
///	    b: B
///	}
///	impl Foo {
///	    fn take(self) -> (A, B) {
///         // error[E0509]: cannot move out of type `Foo`, which implements the `Drop` trait
///	        (self.a, self.b)
///	   }
///	}
/// impl Drop for Foo {
///     fn drop(&mut self) {
///         // do something
///     }
/// }
///
///	fn main(){}
/// ```
///
///  
/// Basic usage: 转移结构体中字段所有权 - V2
///
/// 如果必须要转移结构体所有权，则可以使用std::mem::uninitialized和std::mem::forget
///
///
/// ```rust
///	struct A;
///	struct B;
///	struct Foo {
///	    a: A,
///	    b: B
///	}
///	impl Foo {
///     // 重新实现take
///	    fn take(mut self) -> (A, B) {
///         // 通过std::mem::uninitialized()进行伪初始化
///         // 用于跳过Rust的内存初始化检查
///         // 如果此时对a或b进行读写，则有UB风险，谨慎使用
///	        let a = std::mem::replace(
///	             &mut self.a, unsafe { std::mem::uninitialized() }
///	        );
///	        let b = std::mem::replace(
///	            &mut self.b, unsafe { std::mem::uninitialized() }
///	       );
///        // 通过forget避免调用结构体实例的drop
///	       std::mem::forget(self);
///	       (a, b)
///	   }
///	}
/// impl Drop for Foo {
///     fn drop(&mut self) {
///         // do something
///     }
/// }
///
///	fn main(){}
/// ```
pub fn forget_drop() {
    unimplemented!();
}

/// # 使用`std::mem:ManuallyDrop`手工Drop
///
///
/// `ManuallyDrop<T>`是一个联合体(注意，Rust 1.32 stable已经改为结构体和Lang Item实现)，Rust不会为联合体自动实现Drop。
/// 因为联合体是所有字段共用内存，不能随便被析构，否则会引起未定义行为。
///
/// `std::mem::forget<T>`函数的实现就是用了ManuallyDrop::new方法
///
/// Basic usage: `ManuallyDrop`示例
///
///
/// ```rust
/// use std::mem::ManuallyDrop;
/// struct Peach;
/// struct Banana;
/// struct Melon;
/// struct FruitBox {
///     peach: ManuallyDrop<Peach>,
///     melon: Melon,
///     banana: ManuallyDrop<Banana>,
/// }
/// impl Drop for FruitBox {
///    fn drop(&mut self) {
///        unsafe {
///            ManuallyDrop::drop(&mut self.peach);
///            ManuallyDrop::drop(&mut self.banana);
///        }
///    }
/// }
/// fn main(){}
/// ```
pub fn manually_drop() {
    unimplemented!();
}
