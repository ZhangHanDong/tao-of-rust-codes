/// # 原生指针
///
/// 用途：
/// - 在需要的时候跳过Rust安全检查
/// - 与C语言打交道
/// 
/// Basic usage: 解引用原生指针
///
/// ```
/// fn main() {
///     let mut s = "hello".to_string();
///     let r1 = &s as *const String;
///     let r2 = &mut s as *mut String;
///     assert_eq!(r1, r2);
///     let address = 0x7fff1d72307d;
///     let r3 = address as *const String;
///     unsafe {
///         println!("r1 is: {}", *r1);
///        println!("r2 is: {}", *r2);
///        // Segmentation fault  
///        // assert_eq!(*r1, *r3)
///    }
/// }
/// ```
/// 
/// 
/// Basic usage: 创建空指针
///
/// ```rust
/// fn main() {
///     let p: *const u8 = std::ptr::null();
///     assert!(p.is_null());
///     let s: &str = "hello";
///     let ptr: *const u8 = s.as_ptr();
///     assert!(!ptr.is_null());
///     let mut s = [1, 2, 3];
///     let ptr: *mut u32 = s.as_mut_ptr();
///     assert!(!ptr.is_null());
/// }
/// ```
/// 
/// Basic usage: 使用offset偏移量
///
/// offset方法不能保证传入的偏移量合法，故为unsafe
/// 
/// ```rust
/// fn main() {
///     let s: &str = "Rust";
///     let ptr: *const u8 = s.as_ptr();
///     unsafe {
///         println!("{:?}", *ptr.offset(1) as char); // u
///         println!("{:?}", *ptr.offset(3) as char); // t
///         println!("{:?}", *ptr.offset(255) as char); // ÿ 有UB风险
///     }
/// }
/// ```
/// 
/// Basic usage: 使用read/write
///
/// read/write也是unsafe方法
/// 
/// ```rust
/// fn main() {
///     let x = "hello".to_string();
///     let y: *const u8 = x.as_ptr();
///     unsafe {
///         assert_eq!(y.read() as char, 'h');
///     }
///     let x = [0, 1, 2, 3];
///     let y = x[0..].as_ptr() as *const [u32; 4];
///     unsafe {
///        assert_eq!(y.read(), [0,1,2,3]);
///    }
///    let x = vec![0, 1, 2, 3];
///    let y = &x as *const Vec<i32>;
///    unsafe {
///         assert_eq!(y.read(), [0,1,2,3]);
///    }
///    let mut x = "";
///    let y = &mut x as *mut &str;
///    let z = "hello";
///    unsafe {
///        y.write(z);
///        assert_eq!(y.read(), "hello");
///    }
/// }
/// ```
/// 
/// Basic usage: 使用replace/swap
///
/// 也是unsafe方法，使用它们要注意自引用问题
/// 
/// ```rust
/// fn main() {
///    let mut v: Vec<i32> = vec![1, 2];
///    let v_ptr : *mut i32 = v.as_mut_ptr();
///    unsafe{
///        let old_v = v_ptr.replace(5);
///        assert_eq!(1, old_v);
///        assert_eq!([5, 2], &v[..]);    
///    }
///    let mut v: Vec<i32> = vec![1, 2];
///   let v_ptr  = &mut v as *mut Vec<i32>;
///   unsafe{
///       let old_v = v_ptr.replace(vec![3,4,5]);
///       assert_eq!([1, 2], &old_v[..]);
///       assert_eq!([3, 4, 5], &v[..]);   
///   }
///   let mut array = [0, 1, 2, 3];
///   let x = array[0..].as_mut_ptr() as *mut [u32; 2];
///   let y = array[1..].as_mut_ptr() as *mut [u32; 2];
///   unsafe {
///       assert_eq!([0, 1], x.read());
///       assert_eq!([1, 2], y.read());
///       x.swap(y);
///       assert_eq!([1, 0, 1, 3], array);
///   }
///}
///```
pub fn raw_pointer(){
    unimplemented!();
}