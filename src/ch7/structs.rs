/// # 结构体
///
/// Base usage: 为结构体实现Copy，字段是Copy语义
///
/// ```rust
/// #[derive(Debug,Copy,Clone)]
/// struct Book<'a> {
///     name: &'a str,
///     isbn: i32,
///     version: i32,
/// }
/// fn main(){
///     let book = Book {
///         name: "Rust编程之道" , isbn: 20181212, version: 1
///     };
///     let book2 = Book { version: 2, ..book};
///     println!("{:?}",book);
///     println!("{:?}",book2);
/// }
/// ```
///
/// Base usage: 为结构体实现Copy，字段是移动语义
///
/// ```rust
/// #[derive(Debug,Copy,Clone)] // Error
/// struct Book {
///     name: String,
///     isbn: i32,
///     version: i32,
/// }
/// fn main(){
///     let book = Book {
///         name: "Rust编程之道".to_string() , isbn: 20171111, version: 1
///    };
///    let book2 = Book { version: 2, ..book};
///    // error[E0382]: use of partially moved value: `book`
///    println!("{:?}",book);
///    println!("{:?}",book2);
/// }
/// ```
pub fn structs_intro() {
    #[derive(Debug, Copy, Clone)]
    struct Book<'a> {
        name: &'a str,
        isbn: i32,
        version: i32,
    }
    fn main() {
        let book = Book {
            name: "Rust编程之道",
            isbn: 20181212,
            version: 1,
        };
        let book2 = Book { version: 2, ..book };
        println!("{:?}", book);
        println!("{:?}", book2);
    }
}

/// # 结构体: 面向对象示例
///
/// Base usage: 让终端的输出带上颜色 color.rs
///
/// ```rust
/// use std::fmt;
///
/// struct ColoredString {
///     input: String,
///     fgcolor: String,
///     bgcolor: String,
/// }
///
/// impl Default for ColoredString {
///     fn default() -> Self {
///         ColoredString {
///             input: String::default(),
///             fgcolor: String::new(),
///             bgcolor: String::new(),
///         }
///     }
/// }
///
/// impl ColoredString{
///     fn compute_style(&self) -> String {
///         let mut res = String::from("\x1B[");
///         let mut has_wrote = false;
///         if !self.bgcolor.is_empty() {
///             res.push_str(&self.bgcolor);
///             has_wrote = true;
///         }
///         if !self.fgcolor.is_empty() {
///             if has_wrote {
///                 res.push(';');
///             }
///             res.push_str(&self.fgcolor);
///         }
///         res.push('m');
///         res
///     }
/// }
///
/// trait Colorize {
///     fn red(self) -> ColoredString;
///     fn on_yellow(self) -> ColoredString;
/// }
/// impl<'a> Colorize for ColoredString {
///     fn red(self) -> ColoredString {
///         ColoredString{ fgcolor: String::from("31"), ..self  }
///     }
///     fn on_yellow(self) -> ColoredString {
///         ColoredString { bgcolor: String::from("43"), ..self }
///     }
/// }
/// impl<'a> Colorize for &'a str {
///     fn red(self) -> ColoredString {
///         ColoredString {
///             fgcolor: String::from("31"),
///             input: String::from(self),
///             ..ColoredString::default()
///         }
///     }
///     fn on_yellow(self) -> ColoredString {
///         ColoredString {
///             bgcolor: String::from("43"),
///             input: String::from(self),
///             ..ColoredString::default()
///         }
///     }
/// }
///
/// impl fmt::Display for ColoredString {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         let mut input = &self.input.clone();
///         try!(f.write_str(&self.compute_style()));
///         try!(f.write_str(input));
///         try!(f.write_str("\x1B[0m"));
///         Ok(())
///     }
/// }
///
/// /*
/// 格式参考
/// $ echo "\e[31;43mHello\e[0m"
/// $ echo "\x1B[31;43mHello\x1B[0m"
/// $ echo "\033[31;43mHello\033[0m"
///
/// */
/// fn main() {
///     let hi = "Hello".red().on_yellow();
///     println!("{}", hi);
///     let hi = "Hello".on_yellow();
///     println!("{}", hi);
///     let hi = "Hello".red();
///     println!("{}", hi);
///     let hi = "Hello".on_yellow().red();
///     println!("{}", hi);
/// }
/// ```
///
/// Base usage: 【重构】让终端的输出带上颜色 color.rs
///
/// 下面代码最好手工复制到`play.rust-lang.org`执行
///
/// ```rust
/// use std::convert::From;
/// use std::str::FromStr;
/// use std::string::String;
/// use std::fmt;
///
/// #[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// enum Color {
///     Red,
///     Yellow,
///     Blue,
/// }
///
/// impl Color {
///     pub fn to_fg_str(&self) -> &str {
///         match *self {
///             Color::Red => "31",
///             Color::Yellow => "33",
///             Color::Blue => "34",
///         }
///     }
///     pub fn to_bg_str(&self) -> &str {
///         match *self {
///             Color::Red => "41",
///             Color::Yellow => "43",
///             Color::Blue => "44",
///         }
///     }
/// }
///
/// impl<'a> From<&'a str> for Color {
///     fn from(src: &str) -> Self {
///         src.parse().unwrap_or(Color::Red)
///     }
/// }
/// impl From<String> for Color {
///     fn from(src: String) -> Self {
///         src.parse().unwrap_or(Color::Red)
///     }
/// }
/// impl FromStr for Color {
///     type Err = ();
///     fn from_str(src: &str) -> Result<Self, Self::Err> {
///         let src = src.to_lowercase();
///         match src.as_ref() {
///             "red" => Ok(Color::Red),
///             "yellow" => Ok(Color::Yellow),
///             "blue" => Ok(Color::Blue),
///             _ => Err(()),
///         }
///     }
/// }
///
/// #[derive(Clone, Debug, PartialEq, Eq)]
/// struct ColoredString {
///     input: String,
///     fgcolor: Option<Color>,
///     bgcolor: Option<Color>,
/// }
///
///
///
/// impl ColoredString {
///     fn compute_style(&self) -> String {
///         let mut res = String::from("\x1B[");
///         let mut has_wrote = false;
///         if let Some(ref bgcolor) = self.bgcolor {
///             if has_wrote {
///                 res.push(';');
///             }
///             res.push_str(bgcolor.to_bg_str());
///             has_wrote = true;
///         }
///         if let Some(ref fgcolor) = self.fgcolor {
///             if has_wrote {
///                 res.push(';');
///             }
///             res.push_str(fgcolor.to_fg_str());
///         }
///         res.push('m');
///         res
///     }
/// }
///
///
/// impl Default for ColoredString {
///     fn default() -> Self {
///         ColoredString {
///             input: String::default(),
///             fgcolor: None,
///             bgcolor: None,
///         }
///     }
/// }
///
/// // impl<'a> From<&'a str> for ColoredString {
/// //     fn from(s: &'a str) -> Self {
/// //         ColoredString { input: String::from(s), ..ColoredString::default() }
/// //     }
/// // }
///
/// trait Colorize {
///     fn red(self) -> ColoredString;
///     fn yellow(self) -> ColoredString;
///     fn blue(self) -> ColoredString;
///     fn color<S: Into<Color>>(self, color: S) -> ColoredString;
///     fn on_red(self) -> ColoredString;
///     fn on_yellow(self) -> ColoredString;
///     fn on_blue(self) -> ColoredString;
///     fn on_color<S: Into<Color>>(self, color: S) -> ColoredString;
/// }
/// impl Colorize for ColoredString {
///     fn red(self) -> ColoredString {self.color(Color::Red)}
///     fn yellow(self) -> ColoredString {self.color(Color::Yellow)}
///     fn blue(self) -> ColoredString {self.color(Color::Blue)}
///     fn color<S: Into<Color>>(self, color: S) -> ColoredString {
///         ColoredString { fgcolor: Some(color.into()), ..self }
///     }
///     fn on_red(self) -> ColoredString {self.on_color(Color::Red)}
///     fn on_yellow(self) -> ColoredString {self.on_color(Color::Yellow)}
///     fn on_blue(self) -> ColoredString {self.on_color(Color::Blue)}
///     fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
///         ColoredString { bgcolor: Some(color.into()), ..self }
///     }
/// }
/// impl<'a> Colorize for &'a str {
///     fn red(self) -> ColoredString {self.color(Color::Red)}
///     fn yellow(self) -> ColoredString {self.color(Color::Yellow)}
///     fn blue(self) -> ColoredString {self.color(Color::Blue)}
///     fn color<S: Into<Color>>(self, color: S) -> ColoredString {
///         ColoredString {
///             fgcolor: Some(color.into()),
///             input: String::from(self),
///             ..ColoredString::default()
///         }
///     }
///     fn on_red(self) -> ColoredString {self.on_color(Color::Red)}
///     fn on_yellow(self) -> ColoredString {self.on_color(Color::Yellow)}
///     fn on_blue(self) -> ColoredString {self.on_color(Color::Blue)}
///     fn on_color<S: Into<Color>>(self, color: S) -> ColoredString {
///         ColoredString {
///             bgcolor: Some(color.into()),
///             input: String::from(self),
///             ..ColoredString::default()
///         }
///     }
/// }
///
/// impl fmt::Display for ColoredString {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         let mut input = &self.input.clone();
///
///         try!(f.write_str(&self.compute_style()));
///         try!(f.write_str(input));
///         try!(f.write_str("\x1B[0m"));
///         Ok(())
///     }
/// }
///
/// fn main() {
///     let red = "red".red();
///     println!("{}", red);
///     let yellow = "yellow".yellow().on_blue();
///     println!("{}", yellow);
///     let blue = "blue".blue();
///     println!("{}", blue);
///     let red = "red".color("red");
///     println!("{}", red);
///     let yellow = "yellow".on_color("yellow");
///     println!("{}", yellow);
/// }
/// ```
pub fn color_terminal() {
    unimplemented!();
}

/// # 结构体: 析构顺序
///
/// Base usage: 本地变量
///
/// ```rust
/// struct PrintDrop(&'static str);
///     impl Drop for PrintDrop {
///         fn drop(&mut self) {
///             println!("Dropping {}", self.0)
///     }
/// }
/// fn main() {
///     let x = PrintDrop("x");
///     let y = PrintDrop("y");
/// }
/// ```
///
/// Base usage: 元组
///
/// ```rust
/// struct PrintDrop(&'static str);
///     impl Drop for PrintDrop {
///         fn drop(&mut self) {
///             println!("Dropping {}", self.0)
///     }
/// }
/// fn main() {
///     let tup1 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
///     let tup2 = (PrintDrop("x"), PrintDrop("y"), PrintDrop("z"));
/// }
/// ```
///
/// Base usage: 元组，携带`panic!`的元素
///
/// ```rust
/// struct PrintDrop(&'static str);
///     impl Drop for PrintDrop {
///         fn drop(&mut self) {
///             println!("Dropping {}", self.0)
///     }
/// }
/// fn main() {
///     let tup1 = (PrintDrop("a"), PrintDrop("b"), PrintDrop("c"));
///     let tup2 = (PrintDrop("x"), PrintDrop("y"), panic!());
/// }
/// ```
///
/// Base usage: 闭包捕获变量
///
/// ```rust
/// struct PrintDrop(&'static str);
///     impl Drop for PrintDrop {
///         fn drop(&mut self) {
///             println!("Dropping {}", self.0)
///     }
/// }
/// fn main() {
///     let z = PrintDrop("z");
///     let x = PrintDrop("x");
///     let y = PrintDrop("y");
///     let closure = move || { y; z; x; };
/// }
/// ```
///
/// Base usage: 闭包捕获变量，包含内部作用域
///
/// ```rust
/// struct PrintDrop(&'static str);
///     impl Drop for PrintDrop {
///         fn drop(&mut self) {
///             println!("Dropping {}", self.0)
///     }
/// }
/// fn main() {
///     let y = PrintDrop("y");
///     let x = PrintDrop("x");
///     let z = PrintDrop("z");
///     let closure = move || {
///         { let z_ref = &z; }
///         x; y; z;
///     };
/// }
/// ```
///
/// Base usage: 结构体析构顺序
///
/// ```rust
/// struct PrintDrop(&'static str);
///
/// impl Drop for PrintDrop {
///     fn drop(&mut self) {
///         println!("Dropping {}", self.0)
///     }
/// }
///
/// struct Foo {
///     bar: PrintDrop,
///     baz: PrintDrop,
/// }
///
/// impl Drop for Foo {
///     fn drop(&mut self) {
///         println!("Dropping Foo")
///     }
/// }
///
/// fn main() {
///     let foo = Foo {
///         bar: PrintDrop("bar"),
///         baz: PrintDrop("baz"),
///     };
/// }
/// ```
///
/// Base usage: 结构体和枚举析构顺序
///
/// ```rust
/// struct PrintDrop(&'static str);
/// impl Drop for PrintDrop {
///     fn drop(&mut self) {
///         println!("Dropping {}", self.0)
///     }
/// }
/// enum E {
///     Foo(PrintDrop, PrintDrop),
/// }
/// struct Foo {
///     x: PrintDrop,
///     y: PrintDrop,
///     z: PrintDrop,
/// }
/// fn main() {
///     let e = E::Foo(PrintDrop("a"), PrintDrop("b"));
///     let f = Foo {
///         x: PrintDrop("x"),
///         y: PrintDrop("y"),
///         z: PrintDrop("z"),
///     };
/// }
/// ```
pub fn drop_order() {
    unimplemented!();
}
