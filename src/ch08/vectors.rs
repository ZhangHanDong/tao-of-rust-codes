/// # Vec 动态数组
///
/// Base usage: Vec基本用法
///
/// ```rust
/// fn main() {
///     let mut vec = Vec::new();
///     vec.push(1);
///     vec.push(2);
///     assert_eq!(vec.len(), 2);
///     assert_eq!(vec[0], 1);
///     assert_eq!(vec.pop(), Some(2));
///     assert_eq!(vec.len(), 1);
///     vec[0] = 7;
///    assert_eq!(vec[0], 7);
///    assert_eq!(vec.get(0), Some(&7));
///    assert_eq!(vec.get(10), None);
///    // vec[10];
///    vec.extend([1, 2, 3].iter().cloned());
///    assert_eq!(vec, [7, 1, 2, 3]);
///    assert_eq!(vec.get(0..2), Some(&[7,1][..]));
///    let mut vec2 = vec![4, 5, 6];
///    vec.append(&mut vec2);
///    assert_eq!(vec, [7, 1, 2, 3, 4, 5, 6]);
///    assert_eq!(vec2, []);
///    vec.swap(1, 3);
///    assert!(vec == [7, 3, 2, 1, 4, 5, 6]);
///    let slice = [1, 2, 3, 4, 5, 6, 7];
///    vec.copy_from_slice(&slice);
///    assert_eq!(vec, slice);
///    let slice = [4, 3, 2, 1];
///    vec.clone_from_slice(&slice);
///    assert_eq!(vec, slice);
/// }
/// ```
///
/// Base usage: 使用with_capacity预分配容量
///
/// ```rust
/// fn main() {
///     let mut vec = Vec::with_capacity(10);
///     for i in 0..10 {vec.push(i);}
///     vec.truncate(0);
///     assert_eq!(10, vec.capacity());
///     for i in 0..10 { vec.push(i);}
///     vec.clear();
///     assert_eq!(10, vec.capacity());
///     vec.shrink_to_fit();
///    assert_eq!(0, vec.capacity());
///    for i in 0..10 {
///        vec.push(i);
///        // output: 4/4/4/4/8/8/8/8/16/16/
///        print!("{:?}/", vec.capacity());
///    }
/// }
/// ```
///
/// Base usage: Vec中存储ZST类型
///
/// ```rust
/// struct Foo;
/// fn main(){
///     let mut vec = Vec::new();
///     vec.push(Foo);
///     assert_eq!(vec.capacity(), std::usize::MAX);
/// }
/// ```
///
/// Base usage: Vec查找
///
/// ```rust
/// fn main() {
///     let v = [10, 40, 30];
///     assert!(v.contains(&30));
///     assert!(!v.contains(&50));
///     assert!(v.starts_with(&[10]));
///     assert!(v.starts_with(&[10, 40]));
///     assert!(v.ends_with(&[30]));
///     assert!(v.ends_with(&[40, 30]));
///     assert!(v.ends_with(&[]));
///    let v: &[u8] = &[];
///    assert!(v.starts_with(&[]));
///    assert!(v.ends_with(&[]));
/// }
/// ```
///
/// Base usage: Vec二分查找
///
/// ```rust
/// fn main() {
///     let s = [0, 1, 1, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
///     assert_eq!(s.binary_search(&13),  Ok(9));
///     assert_eq!(s.binary_search(&4),   Err(7));
///     let r = s.binary_search(&1);
///     assert!(match r { Ok(1...4) => true, _ => false, });
///     let seek = 13;
///     assert_eq!(
///         s.binary_search_by(|probe| probe.cmp(&seek)),
///        Ok(9)
///    );
///    let s = [(0, 0), (2, 1), (4, 1), (5, 1), (3, 1),
///               (1, 2), (2, 3), (4, 5), (5, 8), (3, 13),
///               (1, 21), (2, 34), (4, 55)];
///    assert_eq!(
///        s.binary_search_by_key(&13, |&(a,b)| b),
///        Ok(9)
///    );
/// }
/// ```
///
/// Base usage: Vec排序sort系列方法
///
/// ```rust
/// fn main() {
///     let mut v = [-5i32, 4, 1, -3, 2];
///     v.sort();
///     assert!(v == [-5, -3, 1, 2, 4]);
///     v.sort_by(|a, b| a.cmp(b));
///     assert!(v == [-5, -3, 1, 2, 4]);
///     v.sort_by(|a, b| b.cmp(a));
///     assert!(v == [4, 2, 1, -3, -5]);
///     v.sort_by_key(|k| k.abs());
///    assert!(v == [1, 2, -3, 4, -5]);
/// }
/// ```
///
/// Base usage: 比较操作示意
///
/// ```rust
/// use std::cmp::Ordering;
/// fn main(){
///     let result = 1.0.partial_cmp(&2.0);
///     assert_eq!(result, Some(Ordering::Less));
///     let result = 1.cmp(&1);
///     assert_eq!(result, Ordering::Equal);
///     let result = "abc".partial_cmp(&"Abc");
///     assert_eq!(result, Some(Ordering::Greater));
///     let mut v: [f32; 5] = [5.0, 4.1, 1.2, 3.4, 2.5];
///     v.sort_by(|a, b| a.partial_cmp(b).unwrap());
///     assert!(v == [1.2, 2.5, 3.4, 4.1, 5.0]);
///     v.sort_by(|a, b| b.partial_cmp(a).unwrap());
///     assert!(v == [5.0, 4.1, 3.4, 2.5, 1.2]);
/// }
/// ```
///
/// Base usage: Rust 2018 edition 新语法，匹配数组
///
/// 基于此语法可以实现变长参数函数
/// ```rust
/// fn pick(arr: [i32; 3])  {
///     match arr {
///         [_, _, 3] => println!("ends with 3"),
///         [a, 2, c] => println!("{:?}, 2, {:?}", a,  c),
///         [_, _, _] => println!("pass!"),
///     }
/// }
/// fn main(){
///     let arr = [1, 2, 3];
///     pick(arr);
///     let arr = [1, 2, 5];
///     pick(arr);
///     let arr = [1, 3, 5];
///     pick(arr);
/// }
/// ```
///
/// Base usage: Rust 2018 edition 新语法，匹配切片
///
/// 基于此语法可以实现变长参数函数
/// ```rust
/// fn sum(num: &[i32]) {
///     match num {
///         [one] => println!(" at least two"),
///         [first, second] => println!("{:?} + {:?} = {:?} ", first, second, first+second),
///         _ => println!("sum is {:?}", num.iter().fold(0, |sum, i| sum + i) ),
///     }
/// }
/// fn main() {
///     sum(&[1]);
///     sum(&[1, 2]);
///     sum(&[1, 2, 3]);
///     sum(&[1, 2, 3, 5]);
/// }
/// ```



pub fn bases(){
    unimplemented!();
}
