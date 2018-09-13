/// # 字符串String
///
/// Base usage: 组成String的三部分展示
///
/// 1. 指向堆中字节序列的指针（as_ptr方法）
/// 2. 记录堆中字节序列的字节长度（len方法）
/// 3. 堆分配的容量（capacity方法）
///
/// ```rust
/// fn main() {
///     let mut a = String::from("fooα");
///     println!("{:p}", a.as_ptr());
///     println!("{:p}", &a);
///     assert_eq!(a.len(), 5);
///     a.reserve(10);
///     assert_eq!(a.capacity(), 15);
/// }
/// ```
///
/// Base usage: 创建字符串的各种方法
///
///
/// ```rust
/// fn main() {
///     let string: String = String::new();
///     assert_eq!("", string);
///     let string: String = String::from("hello rust");
///     assert_eq!("hello rust", string);
///     let string: String = String::with_capacity(20);
///     assert_eq!("", string);
///     let str: &'static str = "the tao of rust";
///     let string: String =
///        str.chars().filter(|c| !c.is_whitespace()).collect();
///     assert_eq!("thetaoofrust", string);
///     let string: String = str.to_owned();
///     assert_eq!("the tao of rust", string);
///     let string: String = str.to_string();
///     let str: &str = &string[11..15];
///     assert_eq!("rust", str);
/// }
/// ```
///
/// Base usage: chars和bytes方法展示
///
/// 字符串的操作大致分为两种方式：按字节处理和按字符处理
///
/// ```rust
/// fn main() {
///     let str = "borös";
///     let mut chars = str.chars();
///     assert_eq!(Some('b'), chars.next());
///     assert_eq!(Some('o'), chars.next());
///     assert_eq!(Some('r'), chars.next());
///     assert_eq!(Some('ö'), chars.next());
///     assert_eq!(Some('s'), chars.next());
///     let mut bytes = str.bytes();
///     assert_eq!(6, str.len());
///     assert_eq!(Some(98), bytes.next());
///     assert_eq!(Some(111), bytes.next());
///     assert_eq!(Some(114), bytes.next());
///     assert_eq!(Some(195), bytes.next());
///     assert_eq!(Some(182), bytes.next());
///     assert_eq!(Some(115), bytes.next());
/// }
/// ```
///
/// Base usage: 使用get和get_mut
///
/// 字符串不能按index来字节访问字符，所以需要get和get_mut
///
/// ```rust
/// fn main() {
///     let mut v = String::from("borös");
///     assert_eq!(Some("b"), v.get(0..1));
///     assert_eq!(Some("ö"), v.get(3..5));
///     assert_eq!(Some("orös"), v.get(1..));
///     assert!(v.get_mut(4..).is_none()); // 访问了非法的utf8序列
///     assert!(!v.is_char_boundary(4)); // 验证字符是否属于合法边界
///     assert!(v.get_mut(..8).is_none());
///     assert!(v.get_mut(..42).is_none());
/// }
/// ```
///
/// Base usage: 使用split_at和split_at_mut切割字符串
///
/// 需要注意字符串的合法边界
///
/// ```rust
/// fn main() {
///     let s = "Per Martin-Löf";
///     let (first, last) = s.split_at(12);
///     assert_eq!("Per Martin-L", first);
///     assert_eq!("öf", last);
///     // 'main' panicked：byte index 13 is not a char boundary
///     // let (first, last) = s.split_at(13);
/// }
/// ```
///
/// Base usage: 追加字符串
///
///
/// ```rust
/// fn main() {
///     let mut hello = String::from("Hello, ");
///     hello.push('R');
///     hello.push_str("ust!");
///     assert_eq!("Hello, Rust!", hello);
/// }
/// ```
///
/// Base usage: 使用extend迭代器追加字符串
///
///
/// ```rust
/// fn main() {
///     let mut message = String::from("hello");
///     message.extend([',', 'r', 'u'].iter());
///     message.extend("st ".chars());
///     message.extend("w o r l d".split_whitespace());
///     assert_eq!("hello,rust world", &message);
/// }
/// ```
///
/// Base usage: 使用insert和insert_str插入字符串
///
///
/// ```rust
/// fn main() {
///     let mut s = String::with_capacity(3);
///     s.insert(0, 'f');
///     s.insert(1, 'o');
///     s.insert(2, 'o');
///     s.insert_str(0, "bar");
///     assert_eq!("barfoo", s);
/// }
/// ```
///
/// Base usage: 字符串连接
///
///
/// ```rust
/// fn main() {
///     let left = "the tao".to_string();
///     let mut right = "Rust".to_string();
///     assert_eq!(left + " of " + &right, "the tao of Rust");
///     right += "!";
///     assert_eq!(right, "Rust!");
/// }
/// ```
///
/// Base usage: 更新字符串
///
/// ```rust
/// use std::ascii::{AsciiExt};
/// fn main() {
///     let s = String::from("fooαbar");
///     let mut result = s.into_bytes();
///     (0..result.len()).for_each( |i|
///         if i % 2 == 0 {
///             result[i] = result[i].to_ascii_lowercase();
///         }else {
///             result[i] = result[i].to_ascii_uppercase();
///         }
///    );
///    assert_eq!("fOoαBaR", String::from_utf8(result).unwrap());
/// }
/// ```
///
/// Base usage: 按字符迭代处理字符串
///
/// ```rust
/// fn main() {
///     let s = String::from("fooαbar");
///     let s: String = s.chars().enumerate().map(|(i, c)| {
///         if i % 2 == 0 {
///             c.to_lowercase().to_string()
///         } else {
///             c.to_uppercase().to_string()
///         }
///     }).collect();
///    assert_eq!("fOoΑbAr", s);
/// }
/// ```
///
/// Base usage: 字符串删除
///
/// ```rust
/// fn main() {
///     let mut s = String::from("hαllo");
///     s.remove(3);
///     assert_eq!("hαlo", s);
///     assert_eq!(Some('o'), s.pop());
///     assert_eq!(Some('l'), s.pop());
///     assert_eq!(Some('α'), s.pop());
///     assert_eq!("h", s);
///     let mut s = String::from("hαllo");
///     s.truncate(3);
///     assert_eq!("hα", s);
///     s.clear();
///     assert_eq!(s, "");
///     let mut s = String::from("α is alpha, β is beta");
///     let beta_offset = s.find('β').unwrap_or(s.len());
///     let t: String = s.drain(..beta_offset).collect();
///     assert_eq!(t, "α is alpha, ");
///     assert_eq!(s, "β is beta");
///     s.drain(..);
///     assert_eq!(s, "");
/// }
/// ```
pub fn strings(){
    unimplemented!();
}

/// # 字符串查找
///
/// Base usage: 存在性判断
///
///
/// ```rust
/// fn main() {
///     let bananas = "bananas";
///     assert!(bananas.contains('a'));
///     assert!(bananas.contains("an"));
///     assert!(bananas.contains(char::is_lowercase));
///     assert!(bananas.starts_with('b'));
///     assert!(!bananas.ends_with("nana"));
/// }
/// ```
///
/// Base usage: find方法查找位置
///
///
/// ```rust
/// fn main() {
///     let s = "Löwe 老虎 Léopard";
///     assert_eq!(s.find('w'), Some(3));
///     assert_eq!(s.find('老'), Some(6));
///     assert_eq!(s.find('虎'), Some(9));
///     assert_eq!(s.find("é"), Some(14));
///     assert_eq!(s.find("Léopard"), Some(13));
///     assert_eq!(s.rfind('L'), Some(13));
///     assert_eq!(s.find(char::is_whitespace), Some(5));
///     assert_eq!(s.find(char::is_lowercase), Some(1));
/// }
/// ```
///
/// Base usage: split系列方法分割字符串
///
///
/// ```rust
/// fn main() {
///     let s = "Löwe 虎 Léopard";
///     let v  = s.split( |c|
///         (c as u32) >= (0x4E00 as u32) &&  (c as u32) <= (0x9FA5 as u32)
///     ).collect::<Vec<&str>>();
///     assert_eq!(v, ["Löwe ", " Léopard"]);
///     let v = "abc1defXghi".split(|c|
///         c == '1' || c == 'X'
///     ).collect::<Vec<&str>>();;
///    assert_eq!(v, ["abc", "def", "ghi"]);
///    let v = "Mary had a little lambda"
///        .splitn(3, ' ')
///        .collect::<Vec<&str>>();;
///    assert_eq!(v, ["Mary", "had", "a little lambda"]);
///    let v = "A.B.".split(".").collect::<Vec<&str>>();;
///    assert_eq!(v, ["A", "B", ""]);
///    let v = "A.B.".split_terminator('.').collect::<Vec<&str>>();;
///    assert_eq!(v, ["A", "B"]);
///    let v = "A..B..".split(".").collect::<Vec<&str>>();;
///    assert_eq!(v, ["A", "", "B", "", ""]);
///    let v = "A..B..".split_terminator(".").collect::<Vec<&str>>();;
///    assert_eq!(v, ["A", "", "B", ""]);
/// }
/// ```
///
/// Base usage: matches系列方法匹配并捕获字符串
///
///
/// ```rust
/// fn main() {
///     let v = "abcXXXabcYYYabc"
///         .matches("abc").collect::<Vec<&str>>();
///     assert_eq!(v, ["abc", "abc", "abc"]);
///     let v = "1abc2abc3"
///         .rmatches(char::is_numeric).collect::<Vec<&str>>();
///     assert_eq!(v, ["3", "2", "1"]);
///     let v = "abcXXXabcYYYabc"
///        .match_indices("abc").collect::<Vec<_>>();
///    assert_eq!(v, [(0, "abc"), (6, "abc"), (12, "abc")]);
///    let v = "abcXXXabcYYYabc"
///        .rmatch_indices("abc").collect::<Vec<_>>();
///    assert_eq!(v, [(12, "abc"), (6, "abc"), (0, "abc")]);
/// }
/// ```
///
/// Base usage: trim系列方法删除字符串空格、制表符、换行符
///
///
/// ```rust
/// fn main() {
///     let s = " Hello\tworld\t";
///     assert_eq!("Hello\tworld", s.trim());
///     assert_eq!("Hello\tworld\t", s.trim_left());
///     assert_eq!(" Hello\tworld", s.trim_right());
/// }
/// ```
///
/// Base usage: trim_matches系列方法自定义删除规则
///
///
/// ```rust
/// fn main() {
///     assert_eq!("Hello\tworld\t".trim_matches('\t'), "Helloworld");
///     assert_eq!("11foo1bar11".trim_matches('1'), "foo1bar");
///     assert_eq!("123foo1bar123"
///         .trim_matches(char::is_numeric), "foo1bar");
///     let x: &[char] = &['1', '2'];
///     assert_eq!("12foo1bar12".trim_matches(x), "foo1bar");
///     assert_eq!(
///         "1foo1barXX".trim_matches(|c| c == '1' || c == 'X'),
///        "foo1bar"
///     );
///    assert_eq!("11foo1bar11".trim_left_matches('1'), "foo1bar11");
///    assert_eq!(
///        "123foo1bar123".trim_left_matches(char::is_numeric),
///        "foo1bar123");
///    let x: &[char] = &['1', '2'];
///    assert_eq!("12foo1bar12".trim_left_matches(x), "foo1bar12");
///    assert_eq!(
///        "1fooX".trim_right_matches(|c| c == '1' || c == 'X'),
///        "1foo"
///    );
/// }
/// ```
///
/// Base usage: replace替换匹配
///
///
/// ```rust
/// fn main() {
///     let s = "Hello\tworld\t";
///     assert_eq!("Hello world ", s.replace("\t", " "));
///     assert_eq!("Hello world", s.replace("\t", " ").trim());
///     let s = "this is old old 123";
///     assert_eq!("this is new new 123", s.replace("old", "new"));
///     assert_eq!("this is new old 123", s.replacen("old", "new", 1));
///     assert_eq!("this is ald ald 123", s.replacen('o', "a", 3));
///     assert_eq!(
///         "this is old old new23",
///        s.replacen(char::is_numeric, "new", 1)
///    );
/// }
/// ```
pub fn strings_search(){
    unimplemented!();
}

/// # 字符串和其他类型转换
///
/// Base usage: parse转换
///
///
/// ```rust
/// fn main() {
///     let four: u32 = "4".parse().unwrap();
///     assert_eq!(4, four);
///     let four = "4".parse::<u32>();
///     assert_eq!(Ok(4), four);
/// }
/// ```
///
/// Base usage: 为自定义类型实现FromStr，以便使用parse转换
///
///
/// ```rust
/// use std::str::FromStr;
/// use std::num::ParseIntError;
/// #[derive(Debug, PartialEq)]
/// struct Point {
///     x: i32,
///     y: i32
/// }
/// impl FromStr for Point {
///     type Err = ParseIntError;
///    fn from_str(s: &str) -> Result<Self, Self::Err> {
///        let coords = s.trim_matches(|p| p == '{' || p == '}' )
///                        .split(",")
///                        .collect::<Vec<&str>>();
///        let x_fromstr = coords[0].parse::<i32>()?;
///        let y_fromstr = coords[1].parse::<i32>()?;
///        Ok(Point { x: x_fromstr, y: y_fromstr })
///    }
/// }
/// fn main(){
///    let p = Point::from_str("{1,2}");
///    assert_eq!(p.unwrap(), Point{ x: 1, y: 2} );
///    let p = Point::from_str("{3,u}");
///    //Err(ParseIntError { kind: InvalidDigit })
///    println!("{:?}", p);
/// }
/// ```
///
/// Base usage: 其他类型转为字符串
///
///
/// ```rust
/// fn main(){
///     let s: String = format!("{}Rust", "Hello");
///     assert_eq!(s, "HelloRust");
///     assert_eq!(format!("{:5}", "HelloRust"), "HelloRust");
///     assert_eq!(format!("{:5.3}", "HelloRust"), "Hel  ");
///     assert_eq!(format!("{:10}", "HelloRust"), "HelloRust ");
///     assert_eq!(format!("{:<12}", "HelloRust"), "HelloRust   ");
///     assert_eq!(format!("{:>12}", "HelloRust"), "   HelloRust");
///     assert_eq!(format!("{:^12}", "HelloRust"), " HelloRust  ");
///    assert_eq!(format!("{:^12.5}", "HelloRust"), "   Hello    ");
///    assert_eq!(format!("{:=^12.5}", "HelloRust"), "===Hello====");
///    assert_eq!(format!("{:*^12.5}", "HelloRust"), "***Hello****");
///    assert_eq!(format!("{:5}", "th\u{e9}"), "thé  ");
/// }
/// ```
///
/// Base usage: 针对整数进行格式化生成字符串
///
///
/// ```rust
/// fn main(){
///     assert_eq!(format!("{:+}", 1234), "+1234");
///     assert_eq!(format!("{:+x}", 1234), "+4d2");
///     assert_eq!(format!("{:+#x}", 1234), "+0x4d2");
///     assert_eq!(format!("{:b}", 1234), "10011010010");
///     assert_eq!(format!("{:#b}", 1234), "0b10011010010");
///     assert_eq!(format!("{:#20b}", 1234), "       0b10011010010");
///     assert_eq!(format!("{:<#20b}", 1234), "0b10011010010       ");
///     assert_eq!(format!("{:^#20b}", 1234), "   0b10011010010    ");
///    assert_eq!(format!("{:>+#15x}", 1234), "         +0x4d2");
///    assert_eq!(format!("{:>+#015x}", 1234), "+0x0000000004d2");
/// }
/// ```
///
/// Base usage: 针对浮点数进行格式化生成字符串
///
///
/// ```rust
/// fn main(){
///     assert_eq!(format!("{:.4}", 1234.5678), "1234.5678");
///     assert_eq!(format!("{:.2}", 1234.5618), "1234.56");
///     assert_eq!(format!("{:.2}", 1234.5678), "1234.57");
///     assert_eq!(format!("{:<10.4}", 1234.5678), "1234.5678 ");
///     assert_eq!(format!("{:^12.2}", 1234.5618), "  1234.56   ");
///     assert_eq!(format!("{:0^12.2}", 1234.5678), "001234.57000");
///     assert_eq!(format!("{:e}", 1234.5678), "1.2345678e3");
/// }
/// ```
///
/// Base usage: 自定义类型格式化生成字符串，必须实现Display
///
///
/// ```rust
/// use std::fmt::{self, Formatter, Display};
/// struct City {
///     name: &'static str,
///     lat: f32,
///     lon: f32,
/// }
/// impl Display for City {
///     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
///        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
///       let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };
///       write!(f, "{}: {:.3}°{} {:.3}°{}",
///           self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
///    }
/// }
/// fn main() {
///    let city = City { name: "Beijing", lat: 39.90469, lon: -116.40717 };
///    assert_eq!(format!("{}", city), "Beijing: 39.905°N 116.407°W");
///    println!("{}", city);
/// }
/// ```
pub fn strings_convert(){
    unimplemented!();
}

/// # 小例子
///
/// /*
/// The program:
/// You are given a text grid of size N by N.
///
/// Your program must choose words which are placed on the main and the secondary diagonals of the /// given grid.
///
/// The main diagonal is laid from the top-left to the bottom-right corner.
/// The secondary diagonal is laid from the top-right to the bottom-left corner.​
///
/// INPUT:
/// Line 1: An integer number N representing the grid size.
/// Next N lines: N characters.
///
/// OUTPUT:
/// Two words from the diagonals separated by a whitespace.
///
/// CONSTRAINTS:
/// 0 < N ≤ 10
/// A grid contains only latin lowercase letters.
///
/// EXAMPLE:
/// Input
/// 4
/// mooa
/// oano
/// otio
/// ioon
///
/// Output
/// main anti
///
/// */
///
/// 解法1:
///
/// ```rust
/// fn main(){
///     let s = r"1234
///                 5678
///                 9876
///                 4321";
///     let (mut x, mut y) = (0, 0);
///     for (idx, val) in s.lines().enumerate() {
///         let val = val.trim();
///         let left = val.get(idx..idx+1)
///                         .unwrap().parse::<u32>().unwrap();
///        let right = val.get((3 - idx)..(3 - idx+1))
///                          .unwrap().parse::<u32>().unwrap();
///        x += left;
///        y += right;
///    }
///    assert_eq!(38, x+y);
/// }
/// ```
///
/// 解法2:
///
/// ```rust
/// fn main(){
///     let s = r"mooa
///               oano
///               otio
///               ioon";
///     let v = s.split('\n').collect::<Vec<_>>();
///     let mut s1 = String::new();
///     let mut s2 = String::new();
///     for (idx, val) in v.iter().enumerate() {
///         let x = val.trim();
///         let y = x.chars().collect::<Vec<_>>();
///         println!("{:?}", y);
///         s1.push(y[idx]);
///         s2.push(y[3 - idx]);
///     }
///     s1.push(' ');
///     println!("{:?}", s1 +  &s2);
/// }
/// ```
///
/// 解法3:
///
/// ```rust
/// fn main(){
///     let s = r"mooa
///               oano
///               otio
///               ioon";
///     let (mut s1, mut s2) = (String::with_capacity(4), String::with_capacity(4));
///     for (idx, val) in s.lines().enumerate() {
///         let val = val.trim();
///         s1.push_str(val.get(idx..idx+1).unwrap());
///         s2.push_str(val.get((3 - idx)..(3 - idx+1)).unwrap());
///         // println!("s1 cap {:?}", s1.capacity());
///         // println!("s2 cap {:?}", s2.capacity());
///     }
///     s1.push(' ');
///     println!("{:?}", s1 +  &s2);
/// }
/// ```
pub fn practices(){
    unimplemented!();
}
