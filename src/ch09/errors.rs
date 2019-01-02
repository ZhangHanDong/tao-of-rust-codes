/// # 分层错误处理
///
/// 1. Option
/// 2. Error
/// 3. Panic
/// 4. Abort
///
/// Base usage: 使用Option<T>
///
/// ```rust
/// fn get_shortest(names: Vec<&str>) -> Option<&str> {
///     if names.len() > 0 {
///         let mut shortest = names[0];
///         for name in names.iter() {
///             if name.len() < shortest.len() {
///                 shortest = *name;
///             }
///         }
///         Some(shortest)
///    } else {
///        None
///    }
/// }
/// fn show_shortest(names: Vec<&str>) -> &str {
///    match get_shortest(names) {
///        Some(shortest) => shortest,
///        None             => "Not Found",
///    }
/// }
/// fn main(){
///    assert_eq!(show_shortest(vec!["Uku", "Felipe"]), "Uku");
///    assert_eq!(show_shortest(Vec::new()), "Not Found");
/// }
/// ```
///
/// Base usage: 高效处理Option<T>
///
/// ```rust
/// fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
///     match get_shortest(names) {
///         Some(shortest) => Some(shortest.len()),
///         None             => None,
///     }
/// }
/// fn main(){
///     assert_eq!(get_shortest_length(vec!["Uku","Felipe"]),Some(3));
///     assert_eq!(get_shortest_length(Vec::new()), None);
/// }
/// ```
///
/// Base usage: 使用map处理Option<T>
///
/// ```rust
/// fn get_shortest_length(names: Vec<&str>) -> Option<usize> {
///     get_shortest(names).map(|name| name.len())
/// }
/// fn main(){
///     assert_eq!(get_shortest_length(vec!["Uku","Felipe"]),Some(3));
///     assert_eq!(get_shortest_length(Vec::new()), None);
/// }
/// ```
///
/// Base usage: 使用map和and_than处理Option<T>
///
/// ```rust
/// fn double(value: f64) -> f64 {
///     value * 2.
/// }
/// fn square(value: f64) -> f64 {
///     value.powi(2 as i32)
/// }
/// fn inverse(value: f64) -> f64 {
///     value * -1.
/// }
/// fn log(value: f64) -> Option<f64> {
///    match value.log2() {
///        x if x.is_normal() => Some(x),
///        _                      => None
///    }
/// }
/// fn sqrt(value: f64) -> Option<f64> {
///    match value.sqrt() {
///        x if x.is_normal() => Some(x),
///        _                      => None
///    }
/// }
/// fn main () {
///    let number: f64 = 20.;
///    let result = Option::from(number)
///        .map(inverse).map(double).map(inverse)
///        .and_then(log).map(square).and_then(sqrt);
///    match result {
///        Some(x) => println!("Result was {}.", x),
///        None    => println!("This failed.")
///    }
/// }
/// ```
///
/// Base usage: 处理Result<T, E>
///
/// ```rust
/// fn main(){
///     let n = "1";
///     assert_eq!(n.parse::<i32>(), Ok(1));
///     let n = "a";
///     // 输出 Err(ParseIntError { kind: InvalidDigit })
///     println!("{:?}", n.parse::<i32>());
/// }
/// ```
///
/// Base usage: 使用map处理Result<T, E>
///
/// ```rust
/// use std::num::ParseIntError;
/// // fn square(number_str: &str) -> Result<i32, ParseIntError>
/// // {
/// //    number_str.parse::<i32>().map(|n| n.pow(2))
/// // }
/// type ParseResult<T> = Result<T, ParseIntError>;
/// fn square(number_str: &str) -> ParseResult<i32>
/// {
///     number_str.parse::<i32>().map(|n| n.pow(2))
/// }
/// fn main() {
///     match square("10") {
///         Ok(n) => assert_eq!(n, 100),
///         Err(err) => println!("Error: {:?}", err),
///     }
/// }
/// ```
///
/// Base usage: 从文件中读取数字并计算求和，展示统一的错误处理
///
/// 将此代码复制到自定义的io_origin.rs文件里，rustc编译之后在终端执行命令：
///
/// `$ ./io_origin test_txt`
///
/// ```rust
/// use std::env;
/// use std::fs::File;
/// use std::io::prelude::*;
/// fn main() {
///     let args: Vec<String> = env::args().collect();
///     println!("{:?}", args);
///     let filename = &args[1];
///     let mut f = File::open(filename).unwrap();
///     let mut contents = String::new();
///     f.read_to_string(&mut contents).unwrap();
///    let mut sum = 0;
///    for c in contents.lines(){
///        let n = c.parse::<i32>().unwrap();
///        sum += n;
///    }
///    println!("{:?}", sum);
/// }
/// ```
///
/// Base usage: 【重构】从文件中读取数字并计算求和，展示统一的错误处理
///
/// 将此代码复制到自定义的io_origin.rs文件里，rustc编译之后在终端执行命令：
///
/// `$ ./io_origin test_txt`
///
/// ```rust
/// use std::env;
/// use std::error::Error;
/// use std::fs::File;
/// use std::io::prelude::*;
/// use std::process;
/// type ParseResult<i32> = Result<i32, Box<Error>>;
/// fn run(filename: &str) -> ParseResult<i32> {
///     File::open(filename).map_err(|e| e.into())
///     .and_then(|mut f|{
///         let mut contents = String::new();
///         f.read_to_string(&mut contents)
///         .map_err(|e| e.into()).map(|_|contents)
///     })
///     .and_then(|contents|{
///         let mut sum = 0;
///        for c in contents.lines(){
///            match c.parse::<i32>() {
///                Ok(n) => {sum += n;},
///                Err(err) => {
///                    let err: Box<Error> = err.into();
///                    println!("error info: {}, cause: {:?}"
///                    , err.description(),err.cause());
///                },
///                // Err(err) => { return Err(From::from(err));},
///            }
///        }
///        Ok(sum)
///    })
/// }
/// fn main() {
///     let args: Vec<String> = env::args().collect();
///     let filename = &args[1];
///     println!("In file {}", filename);
///     match run(filename) {
///        Ok(n) => { println!("{:?}", n); },
///        Err(e) => {
///            println!("main error: {}", e);
///            process::exit(1);
///        }
///    }
/// }
/// ```
///
/// Base usage: 【再次重构】从文件中读取数字并计算求和，展示统一的错误处理
///
/// 动态分发改为静态分发
///
/// 将此代码复制到自定义的io_origin.rs文件里，rustc编译之后在终端执行命令：
///
/// `$ ./io_origin test_txt`
///
/// ```rust
/// use std::env;
/// use std::error::Error;
/// use std::fs::File;
/// use std::io::prelude::*;
/// use std::process;
/// use std::io;
/// use std::num;
/// use std::fmt;
/// #[derive(Debug)]
/// enum CliError {
///     Io(io::Error),
///     Parse(num::ParseIntError),
/// }
///
/// impl fmt::Display for CliError {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         match *self {
///             CliError::Io(ref err) => write!(f, "IO error: {}", err),
///             CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
///         }
///     }
/// }
///
/// impl Error for CliError {
///     fn description(&self) -> &str {
///         match *self {
///             CliError::Io(ref err) => err.description(),
///             CliError::Parse(ref err) => Error::description(err),
///         }
///     }
///
///     fn cause(&self) -> Option<&Error> {
///         match *self {
///             CliError::Io(ref err) => Some(err),
///             CliError::Parse(ref err) => Some(err),
///         }
///     }
/// }
/// impl From<io::Error> for CliError {
///     fn from(err: io::Error) -> CliError {
///         CliError::Io(err)
///     }
/// }
/// impl From<num::ParseIntError> for CliError {
///     fn from(err: num::ParseIntError) -> CliError {
///         CliError::Parse(err)
///     }
/// }
/// fn run(filename: Option<String>) -> ParseResult<i32> {
///     let mut file = File::open(filename.unwrap())?;
///
///     let mut contents = String::new();
///     file.read_to_string(&mut contents)?;
///     let mut sum = 0;
///     for c in contents.lines(){
///         let n: i32 = c.parse::<i32>()?;
///         sum += n;
///     }
///     Ok(sum)
/// }
/// type ParseResult<i32> = Result<i32, CliError>;
/// fn main() {
///     let filename = env::args().nth(1);
///     match run(filename) {
///         Ok(n) => {
///             println!("{:?}", n);
///         },
///         Err(e) => {
///             println!("main error: {}", e);
///             process::exit(1);
///         }
///     }
/// }
/// ```
///
/// Base usage: 【三次重构】从文件中读取数字并计算求和，展示统一的错误处理
///
/// 使用try!的简化问号操作符
///
/// 将此代码复制到自定义的io_origin.rs文件里，rustc编译之后在终端执行命令：
///
/// `$ ./io_origin test_txt`
///
/// ```rust
/// use std::env;
/// use std::error::Error;
/// use std::fs::File;
/// use std::io::prelude::*;
/// use std::process;
/// use std::io;
/// use std::num;
/// use std::fmt;
///
/// #[derive(Debug)]
/// enum CliError {
///     Io(io::Error),
///     Parse(num::ParseIntError),
/// }
///
/// impl fmt::Display for CliError {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         match *self {
///             CliError::Io(ref err) => write!(f, "IO error: {}", err),
///             CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
///         }
///     }
/// }
///
/// impl Error for CliError {
///     fn description(&self) -> &str {
///         match *self {
///             CliError::Io(ref err) => err.description(),
///             CliError::Parse(ref err) => Error::description(err),
///         }
///     }
///
///     fn cause(&self) -> Option<&Error> {
///         match *self {
///             CliError::Io(ref err) => Some(err),
///             CliError::Parse(ref err) => Some(err),
///         }
///     }
/// }
/// impl From<io::Error> for CliError {
///     fn from(err: io::Error) -> CliError {
///         CliError::Io(err)
///     }
/// }
/// impl From<num::ParseIntError> for CliError {
///     fn from(err: num::ParseIntError) -> CliError {
///         CliError::Parse(err)
///     }
/// }
/// fn run(filename: &str) -> ParseResult<i32> {
///     let mut file = File::open(filename)?;
///
///     let mut contents = String::new();
///     file.read_to_string(&mut contents)?;
///     let mut sum = 0;
///     for c in contents.lines(){
///         let n: i32 = c.parse::<i32>()?;
///         sum += n;
///     }
///     Ok(sum)
/// }
/// type ParseResult<i32> = Result<i32, CliError>;
///
/// fn main() {
///     let args: Vec<String> = env::args().collect();
///     let filename = &args[1];
///     println!("In file {}", filename);
///
///     match run(filename) {
///         Ok(n) => {
///             println!("{:?}", n);
///         },
///         Err(e) => {
///             println!("main error: {}", e);
///             process::exit(1);
///         }
///     }
/// }
/// ```
///
/// Base usage: 【四次重构】从文件中读取数字并计算求和，展示统一的错误处理
///
///  使用NoneError
///
/// 将此代码复制到自定义的io_origin.rs文件里，rustc编译之后在终端执行命令：
///
/// `$ ./io_origin test_txt`
///
/// ```rust
/// #![feature(try_trait)]
/// use std::env;
/// use std::error::Error;
/// use std::fs::File;
/// use std::io::prelude::*;
/// use std::process;
/// use std::io;
/// use std::num;
/// use std::fmt;
/// use std::option::NoneError;
///
/// #[derive(Debug)]
/// enum CliError {
///     Io(io::Error),
///     Parse(num::ParseIntError),
///     NoneError(NoneError),
/// }
/// impl fmt::Display for CliError {
///     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
///         match *self {
///             CliError::Io(ref err) => write!(f, "IO error: {}", err),
///             CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
///             CliError::NoneError(ref err) => write!(f, "Command args error: {:?}", err),
///         }
///     }
/// }
/// impl Error for CliError {
///     fn description(&self) -> &str {
///         match *self {
///             CliError::Io(ref err) => err.description(),
///             CliError::Parse(ref err) => Error::description(err),
///             CliError::NoneError(ref err) => "NoneError",
///         }
///     }
///     fn cause(&self) -> Option<&Error> {
///         match *self {
///             CliError::Io(ref err) => Some(err),
///             CliError::Parse(ref err) => Some(err),
///             _ => None,
///         }
///     }
/// }
///
/// impl From<io::Error> for CliError {
///     fn from(err: io::Error) -> CliError {
///         CliError::Io(err)
///     }
/// }
///
///
/// impl From<NoneError> for CliError {
///     fn from(err: NoneError) -> CliError {
///         CliError::NoneError(err)
///     }
/// }
///
/// impl From<num::ParseIntError> for CliError {
///     fn from(err: num::ParseIntError) -> CliError {
///         CliError::Parse(err)
///     }
/// }
///
/// fn run(filename: Option<String>) -> ParseResult<i32> {
///     let mut file = File::open(filename?)?;
///     let mut contents = String::new();
///     file.read_to_string(&mut contents)?;
///     let mut sum = 0;
///     for c in contents.lines(){
///         let n: i32 = c.parse::<i32>()?;
///         sum += n;
///     }
///     Ok(sum)
/// }
///
/// type ParseResult<i32> = Result<i32, CliError>;
///
/// fn main() {
///     let filename = env::args().nth(1);
///     match run(filename) {
///         Ok(n) => {
///             println!("{:?}", n);
///         },
///         Err(e) => {
///             eprintln!("main error: {}", e);
///             process::exit(1);
///         }
///     }
/// }
/// ```
pub fn error_handle(){
    unimplemented!();
}
