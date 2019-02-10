/// # 生命周期
///
/// Base usage: 函数内借用检查会识别违反借用规则的情况
///    借用的生命周期不能长于出借方的生命周期
/// ```rust
/// fn main() {
///     let r; // 'a ──────────────────────┐
///     {                       //         │
///         let x = 5; // 'b ────────┐     │
///         r = &x;    //            │     │ r是借用，x是出借方
///     }  // ───────────────────────┘     │ 现在r的生命周期长度'a 大于 x的生命周期长度'b
///     println!("r: {}", r); //           │ 必然会造成悬垂指针，所以报错
/// }// ───────────────────────────────────┘
/// ```
pub fn lifetime_demo() {
    fn main() {
        // let r; // 'a ──────────────────────┐
        // {                       //         │
        //     let x = 5; // 'b ────────┐     │
        //     r = &x;    //            │     │
        // }  // ───────────────────────┘     │
        // println!("r: {}", r); //           │
    } // ──────────────────────────────---─────┘
}

/// # 生命周期参数
///
/// Base usage: 在无参数的时候，不能返回借用
/// ```rust
/// fn return_str<'a>() -> &'a str {
///     let mut s = "Rust".to_string();
///     for i in 0..3 {
///         s.push_str(" Good");
///     }
///     &s[..]                   //"Rust Good Good Good"
/// }
/// fn main() {
///     let x = return_str();
/// }
/// ```
///
/// Base usage: 函数的参数和返回值的引用必须相关联
/// ```rust
/// fn foo<'a>(x: &'a str, y: &'a str) -> &'a str {
///     let result = String::from("really long string");
///     // error[E0597]: `result` does not live long enough
///     // 上面的错误提示在Rust 2018 edtion下又有了更精准的描述：
///     // error[E0515]: cannot return value referencing local variable `result`
///     result.as_str()
/// }
/// fn main() {
///     let x = "hello";
///     let y = "rust";
///     foo(x, y);
/// }
/// ```
///
/// Base usage: 一个必须标准生命周期参数但是没有标准的例子
/// ```rust
/// // missing lifetime specifier 编译器无法推导传入和返回的借用是否合法
/// fn the_longest(s1: &str, s2: &str) -> &str {
///     if s1.len() > s2.len() { s1 } else { s2 }
/// }
/// fn main() {
///     let s1 = String::from("Rust");
///     let s1_r = &s1;
///     {
///         let s2 = String::from("C");
///         let res = the_longest(s1_r, &s2);
///        println!("{} is the longest", res);
///    }
/// }
/// ```
///
/// Base usage: 标准生命周期参数修改上面代码
/// ```rust
/// fn the_longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
///     if s1.len() > s2.len() { s1 } else { s2}
/// }
/// fn main() {
///     let s1 = String::from("Rust");
///     let s1_r = &s1;
///     {
///         let s2 = String::from("C");
///         let res = the_longest(s1_r, &s2);
///        println!("{} is the longest", res); // Rust is the longest
///    }
/// }
/// ```
///
/// Base usage: 结构体生命周期参数
/// 这里的生命周期参数标记，实际上是和编译器约定了一个规则：结构体实例的生命周期应小于或等于任意一个成员的生命周期。
/// ```rust
/// struct Foo<'a> {
///     part: &'a str,
/// }
/// fn main() {
///     let words = String::from("Sometimes think, the greatest sorrow than older");
///     let first = words.split(',').next().expect("Could not find a ','");
///     let f = Foo { part: first }; // 编译器在此处会根据约定的规则进行检查，看part的生命周期是否合法
///     assert_eq!("Sometimes think", f.part);
/// }
/// ```
///
/// Base usage: 为Foo<'a>结构体实现方法
/// ```rust
/// #[derive(Debug)]
/// struct Foo<'a> {
///     part: &'a str,
/// }
/// impl<'a> Foo<'a> {
///     fn split_first(s: &'a str) -> &'a str {
///          s.split(',').next().expect("Could not find a ','")
///     }
///     fn new(s: &'a str) -> Self {
///        Foo {part: Foo::split_first(s)}
///    }
///    //满足 生命周期参数省略规则
///    fn get_part(&self) -> &str {
///        self.part
///    }

/// }
/// fn main() {
///     let words = String::from("Sometimes think, the greatest sorrow than older");
///     println!("{:?}",Foo::new(words.as_str()));
/// }
/// ```
///
/// Base usage: 省略生命周期参数
/// ```rust
/// fn first_word(s: &str) -> &str {
///     let bytes = s.as_bytes();
///     for (i, &item) in bytes.iter().enumerate() {
///         if item == b' ' {
///             return &s[0..i];
///         }
///     }
///     &s[..]
/// }
/// fn main() {
///     println!("{:?}", first_word("hello Rust"));
/// }
/// ```
///
/// Base usage: 省略生命周期参数示例
/// ```rust
/// fn print(s: &str);                                           // 省略
/// fn print<'a>(s: &'a str);                                   // 展开
/// fn debug(lvl: uint, s: &str);                              // 省略
/// fn debug<'a>(lvl: uint, s: &'a str);                      // 展开
/// fn substr(s: &str, until: uint) -> &str;                 // 省略
/// fn substr<'a>(s: &'a str, until: uint) -> &'a str;     // 展开
/// fn get_str() -> &str;                                   // 非法
/// fn frob(s: &str, t: &str) -> &str;                    // 非法
/// fn get_mut(&mut self) -> &mut T;                           // 省略
/// fn get_mut<'a>(&'a mut self) -> &'a mut T;              // 展开
/// // 省略
/// fn args<T:ToCStr>(&mut self, args: &[T]) -> &mut Command
/// // 展开
/// fn args<'a, 'b, T:ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command
/// fn new(buf: &mut [u8]) -> BufWriter;                      // 省略
/// fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>          // 展开
/// ```
///
/// Base usage: 生命周期限定
/// ```rust
/// use std::fmt::Debug;
/// #[derive(Debug)]
/// struct Ref<'a, T: 'a>(&'a T);
/// fn print<T>(t: T)
/// where
///     T: Debug,
/// {
///     println!("`print`: t is {:?}", t);
/// }
/// fn print_ref<'a, T>(t: &'a T)
/// where
///    T: Debug + 'a,
/// {
///    println!("`print_ref`: t is {:?}", t);
/// }
/// fn main() {
///    let x = 7;
///    let ref_x = Ref(&x);
///    print_ref(&ref_x);
///    print(ref_x);
/// }
/// ```
///
/// Base usage: 带生命周期参数的trait 对象
/// ```rust
/// trait Foo {}
/// struct Bar<'a> {
///     x: &'a i32,
/// }
/// impl<'a> Foo for Bar<'a> {}
/// fn main() {
///     let num = 5;
///     let box_bar = Box::new(Bar { x: &num });
///     let obj = box_bar as Box<Foo>;
/// }
/// ```
///
/// Base usage: 带生命周期参数的trait 对象，出错
/// ```rust
/// trait Foo<'a> {}
/// struct FooImpl<'a> {
///     s: &'a [u32],
/// }
/// impl<'a> Foo<'a> for FooImpl<'a> {
/// }
/// fn foo<'a>(s: &'a [u32]) -> Box<Foo<'a>> {
///     Box::new(FooImpl { s: s })
/// }
/// fn main(){}
/// ```
///
/// Base usage: 带生命周期参数的trait 对象，修正
/// ```rust
/// trait Foo<'a> {}
/// struct FooImpl<'a> {
///     s: &'a [u32],
/// }
/// impl<'a> Foo<'a> for FooImpl<'a> {
/// }
/// fn foo<'a>(s: &'a [u32]) -> Box<Foo<'a> + 'a> {
///     Box::new(FooImpl { s: s })
/// }
/// fn main(){}
/// ```
pub fn lifetime_params() {
    // fn return_str<'a>() -> &'a str {
    //     let mut s = "Rust".to_string();
    //     for i in 0..3 {
    //         s.push_str("Good ");
    //     }
    //     &s[..]                   //"Rust Good Good Good"
    // }
    fn main() {
        // let x = return_str();
    }
}
