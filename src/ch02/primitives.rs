/// # 基本数据类型：布尔值
///
/// Basic usage:
///
/// ```
/// fn bool_type() {
///     let x = true;  // bool类型可以自动推断
///     let y: bool = false;  // bool值分为true和false
///     let x = 5;
///     if x > 1 { println!( "x is bigger than 1")};  // 比较操作会产生一个bool类型
///     assert_eq!(x as i32, 1);
///     assert_eq!(y as i32, 0);  // 通过as操作符可以将bool转换为数字
/// }
/// bool_type();
/// ```
pub fn bool_type() {
    let x = true;
    let y: bool = false;
    let i = 5;
    if i > 1 { println!( "i is bigger than 1")};
    assert_eq!(x as i32, 1);
    assert_eq!(y as i32, 0);
}

/// # 基本数据类型：数字类型
///
/// Basic usage:
///
/// ```
/// fn number_type() {
///     let num = 42u32;
///     let num: u32 = 42;
///     let num = 0x2A;  // 16进制
///     let num = 0o106;  // 8进制
///     let num = 0b1101_1011; // 2进制
///     assert_eq!(b'*', 42u8); // 字节字面量
///     assert_eq!(b'\'', 39u8);
///     let num = 3.1415926f64;
///     assert_eq!(-3.14, -3.14f64);
///     assert_eq!(2., 2.0f64);
///     assert_eq!(2e4, 20000f64);
///     println!("{:?}", std::f32::INFINITY);
///     println!("{:?}", std::f32::NEG_INFINITY);
///     println!("{:?}", std::f32::NAN);
///     println!("{:?}", std::f32::MIN);
///     println!("{:?}", std::f32::MAX);
/// }
/// number_type();
/// ```
pub fn number_type() {
    use std::f32::{INFINITY, NEG_INFINITY, NAN, MIN, MAX};
    let num = 42u32;
    let num: u32 = 42;
    let num = 0x2A;  // 16进制
    let num = 0o106;  // 8进制
    let num = 0b1101_1011; // 2进制
    assert_eq!(b'*', 42u8); // 字节字面量
    assert_eq!(b'\'', 39u8);
    let num = 3.1415926f64;
    assert_eq!(-3.14, -3.14f64);
    assert_eq!(2., 2.0f64);
    assert_eq!(2e4, 20000f64);
    println!("{:?}", INFINITY);
    println!("{:?}", NEG_INFINITY);
    println!("{:?}", NAN);
    println!("{:?}", MIN);
    println!("{:?}", MAX);
}


/// # 基本数据类型：char类型
///
/// Basic usage:
///
/// ```
/// fn char_type(){
///     let x = 'r';
///     let x = 'Ú';
///     println!("{}", '\'');
///     println!("{}", '\\');
///     println!("{}", '\n');
///     println!("{}", '\r');
///     println!("{}", '\t');
///     assert_eq!('\x2A', '*');
///     assert_eq!('\x25', '%');
///     assert_eq!('\u{CA0}', 'ಠ');
///     assert_eq!('\u{151}', 'ő');
///     assert_eq!('%' as i8, 37);
///     assert_eq!('ಠ' as i8, -96); //该字符值的高位会被截断，最终得到-96
/// }
/// char_type();
/// ```
pub fn char_type(){
    let x = 'r';
    let x = 'Ú';
    println!("{}", '\'');
    println!("{}", '\\');
    println!("{}", '\n');
    println!("{}", '\r');
    println!("{}", '\t');
    assert_eq!('\x2A', '*');
    assert_eq!('\x25', '%');
    assert_eq!('\u{CA0}', 'ಠ');
    assert_eq!('\u{151}', 'ő');
    assert_eq!('%' as i8, 37);
    assert_eq!('ಠ' as i8, -96); //该字符值的高位会被截断，最终得到-96
}

/// # 基本数据类型：array类型
///
/// Basic usage:
///
/// ```
/// fn array_type() {
///     let arr: [i32; 3] = [1, 2, 3];       // 定义一个[i32; 3]类型的数组，默认不可变
///     let mut mut_arr = [1, 2, 3];        // 定义一个可变数组
///     assert_eq!(1, mut_arr[0]);         // 数组索引从0开始，验证第一位元素等于1
///     mut_arr[0] = 3;                            // 修改mut_arr第一个元素为3，因为它是可变数组
///     assert_eq!(3, mut_arr[0]);        // 验证修改之后的mut_arr数组第一个元素为3
///     let init_arr = [0; 10];                  // 创建一个初始值为0，长度为10的数组
///     assert_eq!(0, init_arr[5]);         // 通过数组下标访问数组元素，验证init_arr数组中任意一个元素的值是否为0
///     assert_eq!(10, init_arr.len());  // 验证数组的长度是否为10
///     // println!("{:?}", arr[5]); // error: 索引超出范围
/// }
/// array_type();
/// ```
pub fn array_type() {
    let arr: [i32; 3] = [1, 2, 3];
    let mut mut_arr = [1, 2, 3];
    assert_eq!(1, mut_arr[0]);
    mut_arr[0] = 3;
    assert_eq!(3, mut_arr[0]);
    let init_arr = [0; 10];
    assert_eq!(0, init_arr[5]);
    assert_eq!(10, init_arr.len());
    // println!("{:?}", arr[5]); //error
}

/// # 基本数据类型：range类型
///
/// Basic usage:
///
/// ```
/// fn range_type(){
///     // (1..5)是结构体std::ops::Range的一个实例
///     assert_eq!((1..5), std::ops::Range{ start: 1, end: 5 });
///     // (1..=5)是结构体std::ops::Range的一个实例
///     assert_eq!((1..=5), std::ops::RangeInclusive::new(1, 5));
///     assert_eq!(3+4+5, (3..6).sum());
///     assert_eq!(3+4+5+6, (3..=6).sum());
///     // 每个范围都是一个迭代器，可用for 循环打印范围内的元素
///     for i in (1..5) {
///         println!("{}", i);
///     }
///     for i in (1..=5) {
///         println!("{}", i);
///     }
/// }
/// range_type();
/// ```
pub fn range_type(){
    // (3..5)是结构体std::ops::Range的一个实例
    use std::ops::{Range, RangeInclusive};
    assert_eq!((1..5), Range{ start: 1, end: 5 });
    assert_eq!((1..=5), RangeInclusive::new(1, 5));
    assert_eq!(3+4+5, (3..6).sum());
    assert_eq!(3+4+5+6, (3..=6).sum());

    // 每个范围都是一个迭代器，可用for 循环打印范围内的元素
    for i in (1..5) {
        println!("{}", i);
    }
    for i in (1..=5) {
        println!("{}", i);
    }
}

/// # 基本数据类型：slice类型
///
/// Basic usage:
///
/// ```
/// fn slice_type() {
///     let arr: [i32; 5] = [1, 2, 3, 4, 5];
///     assert_eq!(&arr, &[1, 2,3,4,5]);
///     assert_eq!(&arr[1..], [2,3,4,5]);
///     assert_eq!(&arr.len(), &5);
///     assert_eq!(&arr.is_empty(), &false);
///     let arr = &mut [1, 2, 3];
///     arr[1] = 7;
///     assert_eq!(arr, &[1, 7, 3]);
///     let vec = vec![1, 2, 3];
///     assert_eq!(&vec[..], [1,2,3]);
///     let str_slice: &[&str] = &["one", "two", "three"];
///     assert_eq!(str_slice, ["one", "two", "three"]);
/// }
/// slice_type();
/// ```
pub fn slice_type() {
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(&arr, &[1, 2,3,4,5]);
    assert_eq!(&arr[1..], [2,3,4,5]);
    assert_eq!(&arr.len(), &5);
    assert_eq!(&arr.is_empty(), &false);
    let arr = &mut [1, 2, 3];
    arr[1] = 7;
    assert_eq!(arr, &[1, 7, 3]);
    let vec = vec![1, 2, 3];
    assert_eq!(&vec[..], [1,2,3]);
    let str_slice: &[&str] = &["one", "two", "three"];
    assert_eq!(str_slice, ["one", "two", "three"]);
}

/// # 基本数据类型：str字符串类型
///
/// Basic usage:
///
/// ```
/// fn str_type(){
///     let truth = "Rust是一门优雅的语言";
///     let ptr = truth.as_ptr();
///     let len = truth.len();
///     assert_eq!(28, len);
///     let s = unsafe {
///         let slice = std::slice::from_raw_parts(ptr, len);
///         std::str::from_utf8(slice)
///     };
///     assert_eq!(s, Ok(truth));
/// }
/// str_type();
/// ```
pub fn str_type(){
    use std::slice::from_raw_parts;
    use std::str::from_utf8;
    let truth: &'static str = "Rust是一门优雅的语言";
    let ptr = truth.as_ptr();
    let len = truth.len();
    assert_eq!(28, len);
    let s = unsafe {
        let slice = from_raw_parts(ptr, len);
        from_utf8(slice)
    };
    assert_eq!(s, Ok(truth));
}

fn move_coords( x: (i32,i32) ) -> (i32, i32) {
    (x.0 + 1, x.1 + 1)
}
/// # 基本数据类型：元组类型
///
/// Basic usage:
///
/// ```
/// // 利用元组返回多个值
/// fn move_coords( x: (i32,i32) ) -> (i32, i32) {
///     (x.0 + 1, x.1 + 1)
/// }
///
/// fn tuple_type(){
///     let tuple : (&'static str, i32, char) = ("hello", 5, 'c');  // 长度为3的异构序列
///     assert_eq!(tuple.0, "hello");  // 通过索引获取元组元素
///     assert_eq!(tuple.1, 5);
///     assert_eq!(tuple.2, 'c');
///     let coords = (0, 1);
///     let result = move_coords(coords);
///     assert_eq!(result, (1, 2));  //  move_coords函数返回元组
///     let (x, y) =  move_coords(coords); //  let 模式匹配解构操作
///     assert_eq!(x, 1);
///     assert_eq!(y, 2);
/// }
/// tuple_type();
/// ```
pub fn tuple_type(){
    let tuple = ("hello", 5, 'c');
    assert_eq!(tuple.0, "hello");
    assert_eq!(tuple.1, 5);
    assert_eq!(tuple.2, 'c');
    let coords = (0, 1);
    let result = move_coords(coords);
    assert_eq!(result, (1, 2));
    let (x, y) =  move_coords(coords);
    assert_eq!(x, 1);
    assert_eq!(y, 2);
}

/// # 基本数据类型：原始指针类型
///
/// Basic usage:
///
/// ```
/// // 利用元组返回多个值
/// fn raw_pointer() {
///     let mut x = 10;
///     let ptr_x = &mut x as *mut i32; // 通过as转换为*mut T原始指针
///     let y = Box::new(20);  // 使用Box语法代表在堆内存中存放数字20
///     let ptr_y = &*y as *const i32; // 通过as转换为*const T原始指针
///     unsafe {
///     // 使用它们必须置于unsafe块中
///     // *ptr_x 是可变的，*ptr_y是只读的
///     // 所以这里不会有安全问题
///     *ptr_x += *ptr_y;
///     }
///     assert_eq!(x, 30);
/// }
/// raw_pointer();
/// ```
pub fn raw_pointer() {
    let mut x = 10;
    let ptr_x = &mut x as *mut i32;
    let y = Box::new(20);
    let ptr_y = &*y as *const i32;
    unsafe {
      *ptr_x += *ptr_y;
    }
    assert_eq!(x, 30);
}

/// # 基本数据类型：nerver类型
///
/// Basic usage:
///
/// ```
/// #![feature(never_type)]
///
/// fn foo() -> u32 {
///     let x: ! = {
///         return 123
///     };
/// }
///
/// fn nerver(){
///     let num: Option<u32> = Some(42);
///     match num {
///         Some(num) => num,
///         None => panic!("Nothing!"),
///     };
/// }
/// foo();
/// nerver();
/// ```
pub fn nerver(){
    let num: Option<u32> = Some(42);
    match num {
        Some(num) => num,
        None => panic!("Nothing!"),
    };
}

fn foo() -> u32 {
    let x: ! = {
        return 123
    };
}
