/// # 字符串编码
///
/// Base usage: 从Unicode到UTF-8编码过程
///
/// ```rust
/// use std::str;
/// fn main() {
///     let tao = str::from_utf8(&[0xE9u8, 0x81u8, 0x93u8]).unwrap(); // 将UTF-8序列转为字符串
///     assert_eq!("道", tao);
///     assert_eq!("道", String::from("\u{9053}")); // 将16进制Unicode码位转为字符串
///     let unicode_x = 0x9053;
///     let utf_x_hex = 0xe98193;
///     let utf_x_bin  = 0b111010011000000110010011;
///     println!("unicode_x: {:b}", unicode_x);
///     println!("utf_x_hex: {:b}", utf_x_hex);
///     println!("utf_x_bin: 0x{:x}", utf_x_bin);
/// }
/// ```
///
/// Base usage: 字符与Unicode标量值一一对应
///
/// ```rust
/// fn main() {
///     let tao = '道';
///     let tao_u32 = tao as u32;
///     assert_eq!(36947, tao_u32);
///     println!("U+{:x}", tao_u32); // U+9053
///     println!("{}", tao.escape_unicode());  //\u{9053}
///     assert_eq!(char::from(65), 'A');
///     assert_eq!(std::char::from_u32(0x9053), Some('道'));
///     assert_eq!(std::char::from_u32(36947), Some('道'));
///     assert_eq!(std::char::from_u32(12901010101), None); // 该数字并不是一个有效的Unicode标量值
/// }
/// ```
///
/// Base usage: 将字符转为字符串要注意字节长度
///
/// ```rust
/// fn main() {
///     let mut b = [0; 3];
///     let tao = '道';
///     assert_eq!(4, tao.len_utf8());
///     let tao_str = tao.encode_utf8(&mut b); // 将tao转为字符串tao_str
///     assert_eq!("道", tao_str);
///     assert_eq!(3, tao.len_utf8());
/// }
/// ```
///
/// Base usage: 包含单个码位的才能声明为字符
///
/// 特此说明：在Rust 1.30版下面代码将不会报错，将支持多码位字符
///
/// ```rust
/// fn main() {
///     let e = 'é';  // Error: 此处包含两个码位
///     println!("{}", e as u32);
/// }
/// ```
/// 
///
/// Base usage: 展示char类型的内建方法
///
/// ```rust
/// fn main(){
///     assert_eq!(true, 'f'.is_digit(16));
///     assert_eq!(Some(15), 'f'.to_digit(16));
///     assert!('a'.is_lowercase());
///     assert!(!'道'.is_lowercase());
///     assert!(!'a'.is_uppercase());
///     assert!('A'.is_uppercase());
///     assert!(!'中'.is_uppercase());
///     assert_eq!('i', 'I'.to_lowercase());
///     assert_eq!('B', 'b'.to_uppercase());
///     assert!(' '.is_whitespace());
///     assert!('\u{A0}'.is_whitespace());
///     assert!(!'越'.is_whitespace());
///     assert!('a'.is_alphabetic());
///     assert!('京'.is_alphabetic());
///     assert!(!'1'.is_alphabetic());
///     assert!('7'.is_alphanumeric());
///     assert!('K'.is_alphanumeric());
///     assert!('藏'.is_alphanumeric());
///     assert!(!'¾'.is_alphanumeric());
///     assert!(''.is_control());
///     assert!(!'q'.is_control());
///     assert!('٣'.is_numeric());
///     assert!('7'.is_numeric());
///     assert!(!'و'.is_numeric());
///     assert!(!'藏'.is_numeric());
///     println!("{}", '\r'.escape_default());
/// }
/// ```

pub fn bases(){
    unimplemented!();
}
