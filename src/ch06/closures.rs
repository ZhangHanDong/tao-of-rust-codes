/// # 闭包
///
/// Base usage: 将计数函数改为闭包形式（Rust 2015）
///
/// ```rust
/// fn counter(i: i32) -> Box<Fn(i32) -> i32> {
///     Box::new(move |n: i32| n + i )
/// }
/// fn main() {
///     let f = counter(3);
///     assert_eq!(4, f(1));
/// }
/// ```
///
/// Base usage: 将计数函数改为闭包形式 (Rust 2018)
///
/// ```rust
/// fn counter(i: i32) -> impl Fn(i32) -> i32 {
///     move |n: i32|{ n + i }
/// }
/// fn main() {
///     let f = counter(3);
///     assert_eq!(4, f(1));
/// }
/// ```
///
/// Base usage: 闭包参数可以接收任意类型
///
/// ```rust
/// fn val() -> i32 { 5 }
/// fn main(){
///     let add = |a: fn() -> i32, (b, c)| (a)() + b + c;
///     let r = add(val, (2, 3));
///     assert_eq!(r, 10);
/// }
/// ```
///
/// Base usage: 定义一模一样的闭包也不是同一个类型
///
/// ```rust
/// fn main(){
///     let c1 = || {};
///     let c2 = || {};
///     let v = [c1, c2];
/// }
/// ```
pub fn closures(){
    fn counter(i: i32) -> Box<Fn(i32) -> i32> {
        Box::new(move |n: i32| n + i )
    }
    fn main() {
        let f = counter(3);
        assert_eq!(4, f(1));
    }
}

/// # 自定义闭包实现
///
/// Base usage:
///
/// ```rust
/// #![feature(unboxed_closures, fn_traits)]
/// struct Closure {
///     env_var: u32,
/// }
/// impl FnOnce<()> for Closure {
///     type Output = u32;
///     extern "rust-call" fn call_once(self, args: ()) -> u32 {
///         println!("call it FnOnce()");
///         self.env_var + 2
///     }
/// }
/// impl FnMut<()> for Closure {
///     extern "rust-call" fn call_mut(&mut self, args: ()) -> u32 {
///         println!("call it FnMut()");
///         self.env_var + 2
///     }
/// }
/// impl Fn<()> for Closure {
///     extern "rust-call" fn call(&self, args: ()) -> u32 {
///         println!("call it Fn()");
///         self.env_var + 2
///     }
/// }
/// fn call_it<F: Fn() -> u32>(f: &F) -> u32 {
///     f()
/// }
/// fn call_it_mut<F: FnMut() -> u32>(f: &mut F) -> u32 {
///     f()
/// }
/// fn call_it_once<F: FnOnce() -> u32>(f: F) -> u32 {
///     f()
/// }
/// fn main() {
///     let env_var = 1;
///     let mut c = Closure { env_var: env_var };
///     c();
///     c.call(());
///     c.call_mut(());
///     c.call_once(());
///     let mut c = Closure { env_var: env_var };
///     {
///         assert_eq!(3, call_it(&c));
///     }
///     {
///         assert_eq!(3, call_it_mut(&mut c));
///     }
///     {
///         assert_eq!(3, call_it_once(c));
///     }
/// }
/// ```
///
/// Base usage: 等价于下面代码
///
/// ```rust
/// fn main(){
///     let env_var = 1;
///     let c = || env_var + 2;
///     assert_eq!(3, c());
/// }
/// ```
pub fn custom_closures(){
    #![feature(unboxed_closures, fn_traits)]
    struct Closure {
        env_var: u32,
    }
    impl FnOnce<()> for Closure {
        type Output = u32;
        extern "rust-call" fn call_once(self, args: ()) -> u32 {
            println!("call it FnOnce()");
            self.env_var + 2
        }
    }
    impl FnMut<()> for Closure {
        extern "rust-call" fn call_mut(&mut self, args: ()) -> u32 {
            println!("call it FnMut()");
            self.env_var + 2
        }
    }
    impl Fn<()> for Closure {
        extern "rust-call" fn call(&self, args: ()) -> u32 {
            println!("call it Fn()");
            self.env_var + 2
        }
    }
    fn call_it<F: Fn() -> u32>(f: &F) -> u32 {
        f()
    }
    fn call_it_mut<F: FnMut() -> u32>(f: &mut F) -> u32 {
        f()
    }
    fn call_it_once<F: FnOnce() -> u32>(f: F) -> u32 {
        f()
    }
    fn main() {
        let env_var = 1;
        let mut c = Closure { env_var: env_var };
        c();
        c.call(());
        c.call_mut(());
        c.call_once(());
        let mut c = Closure { env_var: env_var };
        {
            assert_eq!(3, call_it(&c));
        }
        {
            assert_eq!(3, call_it_mut(&mut c));
        }
        {
            assert_eq!(3, call_it_once(c));
        }
    }
}

/// # 闭包生成类型
///
/// Base usage: Copy语义自动实现Fn
///
/// ```rust
/// fn main() {
///     let s = "hello";
///     let c = ||{ println!("{:?}", s)  };
///     c();
///     c();
///     println!("{:?}", s);
/// }
/// ```
///
/// Base usage: 模拟闭包生成的结构体和trait
///
/// ```rust
/// #![feature(unboxed_closures, fn_traits)]
/// struct Closure<'a> {
///     env_var: &'a u32
/// }
/// impl<'a> FnOnce<()> for Closure<'a> {
///     type Output = ();
///     extern "rust-call" fn call_once(self, args: ()) -> () {
///         println!("{:?}", self.env_var);
///     }
/// }
/// impl<'a> FnMut<()> for Closure<'a> {
///     extern "rust-call" fn call_mut(&mut self, args: ()) -> () {
///         println!("{:?}", self.env_var);
///     }
/// }
/// impl<'a> Fn<()> for Closure<'a> {
///    extern "rust-call" fn call(&self, args: ()) -> () {
///        println!("{:?}", self.env_var);
///    }
/// }
/// fn main(){
///    let env_var = 42;
///    let mut c = Closure{env_var: &env_var};
///    c(); //42
///    c.call_mut(()); // 42
///    c.call_once(()); // 42
/// }
/// ```
///
/// Base usage: 实现Fn，必然要实现FnMut和FnOnce
///
/// ```rust
/// #![feature(fn_traits)]
/// fn main() {
///     let s = "hello";
///     let mut c = ||{ println!("{:?}", s)  };
///     c();  // "hello"
///     c();  // "hello"
///     c.call_mut(());  // "hello"
///     c.call_once(());  // "hello"
///     c(); // "hello" 这里没有被转移所有权是因为生成的FnOnce自动实现的Copy
///     println!("{:?}", s);  // "hello"
/// }
/// ```
///
/// Base usage: 转移语义类型自动实现FnOnce
///
/// ```rust
/// fn main() {
///     let s = "hello".to_string();
///     let c =  || s;
///     c();
///     // c(); // error: use of moved value: `c`
///     // println!("{:?}", s); // error: use of moved value: `s`
/// }
/// ```
///
/// Base usage: 实现FnOnce，不需要实现Fn和FnMut
///
/// ```rust
/// #![feature(fn_traits)]
/// fn main() {
///     let mut s = "hello".to_string();
///     let c = || s;
///     c();
///     // error: expected a closure that implements the `FnMut` trait,
///     //       but this closure only implements `FnOnce`
///     // c.call(());
///     // error: expected a closure that implements the `FnMut` trait,
///    //        but this closure only implements `FnOnce`
///    // c.call_mut(());
///    // c(); // error: use of moved value: `c`
///    // println!("{:?}", s); // error use of moved value: `s`
/// }
/// ```
///
/// Base usage: 使用move关键字
///
/// ```rust
/// fn main() {
///     let s = "hello";
///     let c = move ||{ println!("{:?}", s)  };
///     c();
///     c();
///     println!("{:?}", s);
/// }
/// ```
///
/// Base usage: 使用move关键字会影响到闭包自身吗
///
/// ```rust
/// fn call<F: FnOnce()>(f: F) { f() }
/// // 未使用move
/// fn main() {
///     let mut x = 0;
///     let incr_x = || x += 1;
///     call(incr_x);
///     // call(incr_x); // ERROR: `incr_x` moved in the call above.
///     // 使用move
///     let mut x = 0;
///     let incr_x = move  || x += 1;
///     call(incr_x);
///     call(incr_x);
///     // 对移动语义类型使用move
///     let mut x = vec![];
///     let expend_x = move || x.push(42);
///     call(expend_x);
///     // call(expend_x); // ERROR:  use of moved value: `expend_x`
/// }
/// ```
///
/// Base usage: 修改环境变量自动实现FnMut
///
/// ```rust
/// fn main() {
///     let mut s = "rush".to_string();
///     {
///         let mut c =  ||{ s += " rust" };
///         c();
///         c();
///         //   error: cannot borrow `s` as immutable
///         //          because it is also borrowed as mutable
///         //   println!("{:?}", s);
///    }
///   println!("{:?}", s);
/// }
/// ```
///
/// Base usage: 实现FnMut必然会实现FnOnce，但不会实现Fn
///
/// ```rust
/// #![feature(fn_traits)]
/// fn main () {
///     let mut s = "rush".to_string();
///    {
///        let mut c = || s += " rust";
///         c();
///         // error: expected a closure that implements the `Fn` trait,
///         //        but this closure only implements `FnMut`
///         // c.call(());
///        c.call_once(());
///        //   error: cannot borrow `s` as immutable
///        //          because it is also borrowed as mutable
///        // println!("{:?}",s);
///    }
///    println!("{:?}",s); // "rush rust rust"
/// }
/// ```
///
/// Base usage: 未捕获任何环境变量自动实现Fn
///
/// ```rust
/// fn main() {
///     let c = ||{ println!("hhh")  };
///     c();
///     c();
/// }
/// ```
pub fn closure_gen_type(){
    fn main() {
        let s = "hello";
        let c = ||{ println!("{:?}", s)  };
        c();
        c();
        println!("{:?}", s);
    }
}

/// # 闭包作为参数或返回值传递
///
/// Base usage: 闭包作为参数
///
/// ```rust
/// fn boxed_closure(c: &mut Vec<Box<Fn()>>){
///     let s = "second";
///     c.push(Box::new(|| println!("first")));
///     c.push(Box::new(move || println!("{}", s)));
///     c.push(Box::new(|| println!("third")));
/// }
/// fn main(){
///     let mut c: Vec<Box<Fn()>> = vec![];
///     boxed_closure(&mut c);
///     for f in c {
///         f(); // first / second / third
///     }
/// }
/// ```
///
/// Base usage: 实现any方法，静态分发
///
/// 注意：此处自定义Any，不同于标准库提供的Any
///
/// ```rust
/// use std::ops::Fn;
/// trait Any {
///     fn any<F>(&self,  f: F) -> bool where
///     Self: Sized,
///     F: Fn(u32) -> bool;
/// }
/// impl Any for Vec<u32> {
///     fn any<F>(&self, f: F) -> bool where
///     Self: Sized,
///    F: Fn(u32) -> bool
///    {
///        for &x in self {
///            if f(x) {
///                return true;
///            }
///        }
///        false
///    }
/// }
/// fn main(){
///     let  v = vec![1,2,3];
///     let b = v.any(|x| x == 3);
///     println!("{:?}", b);
/// }
/// ```
///
/// Base usage: 函数指针也实现了Fn/FnMut/FnOnce
///
/// ```rust
/// fn call<F>(closure: F) -> i32
/// where F: Fn(i32) -> i32
/// {
///     closure(1)
/// }
/// fn counter(i: i32) -> i32 { i+1 }
/// fn main(){
///     let result = call(counter);
///     assert_eq!(2, result);
/// }
/// ```
///
/// Base usage: 实现any方法，动态分发
///
/// 注意：此处自定义Any，不同于标准库提供的Any
///
/// ```rust
/// trait Any {
///   fn any(&self,  f: &(Fn(u32) -> bool)) -> bool;
/// }
/// impl Any for Vec<u32> {
///     fn any(&self, f: &(Fn(u32) -> bool)) -> bool {
///         for &x in self.iter() {
///             if f(x) {
///                 return true;
///             }
///        }
///       false
///    }
/// }
/// fn main(){
///    let  v = vec![1,2,3];
///    let b = v.any(&|x| x == 3);
///    println!("{:?}", b);
/// }
/// ```
///
/// Base usage: 只有逃逸闭包可装箱
///
/// ```rust
/// fn main(){
///     let s = "hello";
///     let c: Box<Fn() + 'static> = Box::new( move||{ s;});
/// }
/// ```
///
/// Base usage: 闭包作为返回值
///
/// ```rust
/// fn square() -> Box<Fn(i32) -> i32> {
///     Box::new(|i| i*i )
/// }
/// fn main(){
///     let square = square();
///     assert_eq!(4, square(2));
///     assert_eq!(9, square(3));
/// }
/// ```
///
/// Base usage: 闭包作为返回值 报错
///
/// ```rust
/// fn square() -> Box<FnOnce(i32) -> i32> {
///     Box::new( |i| {i*i })
/// }
/// fn main(){
///     let square = square();
///     assert_eq!(4, square(2));
/// }
/// ```
///
/// Base usage: 使用FnBox修正
///
/// ```rust
/// #![feature(fnbox)]
/// use std::boxed::FnBox;
/// fn square() -> Box<FnBox(i32) -> i32> {
///     Box::new( |i| {i*i })
/// }
/// fn main(){
///     let square = square();
///     assert_eq!(4, square(2));
/// }
/// ```
///
/// Base usage: 使用impl Trait
///
/// ```rust
/// fn square() -> impl FnOnce(i32) -> i32 {
///     |i| {i*i }
/// }
/// fn main(){
///     let square = square();
///     assert_eq!(4, square(2));
/// }
/// ```
pub fn pass_closure(){
    fn boxed_closure(c: &mut Vec<Box<Fn()>>){
        let s = "second";
        c.push(Box::new(|| println!("first")));
        c.push(Box::new(move || println!("{}", s)));
        c.push(Box::new(|| println!("third")));
    }
    fn main(){
        let mut c: Vec<Box<Fn()>> = vec![];
        boxed_closure(&mut c);
        for f in c {
            f(); // first / second / third
        }
    }

}

/// # 高阶生命周期
///
/// Base usage: Error版本
///
/// ```rust
/// use std::fmt::Debug;
/// trait DoSomething<T> {
///     fn do_sth(&self, value: T);
/// }
/// impl<'a, T: Debug> DoSomething<T> for &'a usize {
///     fn do_sth(&self, value: T) {
///         println!("{:?}", value);
///     }
/// }
/// fn foo<'a>(b: Box<DoSomething<&'a usize>>) {
///     let s: usize = 10;
///     b.do_sth(&s) // error[E0597]: `s` does not live long enough
/// }
/// fn main(){
///     let x  = Box::new(&2usize);
///     foo(x);
/// }
/// ```
///
/// Base usage: 使用for<>高阶生命周期语法
///
/// ```rust
/// use std::fmt::Debug;
/// trait DoSomething<T> {
///     fn do_sth(&self, value: T);
/// }
/// impl<'a, T: Debug> DoSomething<T> for &'a usize {
///     fn do_sth(&self, value: T) {
///         println!("{:?}", value);
///     }
/// }
/// fn bar(b: Box<for<'f> DoSomething<&'f usize>>) {
///     let s: usize = 10;
///     b.do_sth(&s);
/// }
/// fn main(){
///     let x  = Box::new(&2usize);
///     bar(x);
/// }
/// ```
///
/// Base usage: 以闭包为结构体字段，自动推断引用的生命周期参数
///
/// ```rust
/// struct Pick<F> {
///     data: (u32, u32),
///     func: F,
/// }
/// impl<F> Pick<F>
///     where F: Fn(&(u32, u32)) -> &u32
///     {
///     fn call(&self) -> &u32 {
///         (self.func)(&self.data)
/// }
/// }
/// fn max(data: &(u32, u32)) -> &u32 {
///     if data.0 > data.1{
///         &data.0
///     }else{
///         &data.1
///     }
///
/// }
/// fn main() {
///    let elm = Pick { data: (3, 1), func: max };
///    println!("{}", elm.call());
/// }
/// ```
///
/// Base usage: 以闭包为结构体字段，显式指定引用的生命周期参数
///
/// ```rust
/// struct Pick<F> {
///     data: (u32, u32),
///     func: F,
/// }
/// impl<F> Pick<F>
///     where F: for<'f> Fn(&'f (u32, u32)) -> &'f u32, // 显式指定
///     {
///     fn call(&self) -> &u32 {
///         (self.func)(&self.data)
///  }
/// }
/// fn max(data: &(u32, u32)) -> &u32 {
///     if data.0 > data.1{
///         &data.0
///     }else{
///         &data.1
///     }
/// }
/// fn main() {
///    let elm = Pick { data: (3, 1), func: max };
///    println!("{}", elm.call());
/// }
/// ```
pub fn higher_kind_lifetime(){
    use std::fmt::Debug;
    trait DoSomething<T> {
        fn do_sth(&self, value: T);
    }
    impl<'a, T: Debug> DoSomething<T> for &'a usize {
        fn do_sth(&self, value: T) {
            println!("{:?}", value);
        }
    }
    fn bar(b: Box<for<'f> DoSomething<&'f usize>>) {
        let s: usize = 10;
        b.do_sth(&s);
    }
    fn main(){
        let x  = Box::new(&2usize);
        bar(x);
    }
}
