/// # 枚举类型： 无参数
///
/// Basic usage:
///
/// ```
/// enum Number {
///     Zero,
///     One,
///     Two,
/// }
/// let a = Number::One;
/// match a {
///     Number::Zero => println!("0"),
///     Number::One => println!("1"),
///     Number::Two => println!("2"),
/// }
/// ```
pub enum Number {
    Zero,
    One,
    Two,
}

/// # 枚举类型： like C
///
/// Basic usage:
///
/// ```
/// enum Color {
///    Red = 0xff0000,
///    Green = 0x00ff00,
///    Blue = 0x0000ff,
///}
/// println!("roses are #{:06x}", Color::Red as i32);
/// println!("violets are #{:06x}", Color::Blue as i32);
/// ```
pub enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

/// # 枚举类型： 带参数
///
/// Basic usage:
///
/// ```
/// enum IpAddr {
///     V4(u8, u8, u8, u8),
///     V6(String),
/// }
/// let f : fn(u8, u8, u8, u8) -> IpAddr = IpAddr::V4;
/// let ff : fn(String) -> IpAddr = IpAddr::V6;
/// let home = IpAddr::V4(127, 0, 0, 1);
/// ```
pub enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

/// # 枚举类型：Option
///
/// Basic usage:
///
/// ```
/// enum Option{
///     Some(i32),
///     None,
/// }
/// let s = Some(42);
/// assert_eq!(s.unwrap(), 42);
/// match s {
///     Some(n) => println!("num is: {}", n),
///     None => (),
/// };
/// ```
pub enum MyOption{
    Some(i32),
    None,
}

/// # match 匹配引用 (Rust 2015 用法)
///
/// Basic usage:
///
/// ```
/// fn match_ref_2015() {
///     let s: &Option<String> = &Some("hello".to_string());
///     match s {
///         &Some(ref s) => println!("s is: {}", s),
///         _ => (),
///     };
/// }
/// match_ref_2015();
/// ```
pub fn match_ref_2015() {
    let s: &Option<String> = &Some("hello".to_string());
    match s {
        &Some(ref s) => println!("s is: {}", s),
        _ => (),
    };
}


/// # match 匹配引用 (Rust 2018 用法)
///
/// Basic usage:
///
/// ```
/// fn match_ref_2018() {
///     let s: &Option<String> = &Some("hello".to_string());
///     match s {
///         Some(s) => println!("s is: {}", s),
///         _ => (),
///     };
/// }
/// match_ref_2018();
/// ```
pub fn match_ref_2018() {
    let s: &Option<String> = &Some("hello".to_string());
    match s {
        Some(s) => println!("s is: {}", s),
        _ => (),
    };
}
