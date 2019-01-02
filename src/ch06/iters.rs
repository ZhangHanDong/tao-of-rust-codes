/// # 迭代器
///
/// Base usage: 自定义内部迭代器
///
/// ```rust
/// trait InIterator<T: Copy> {
///     fn each<F: Fn(T) -> T>(&mut self, f: F);
/// }
/// impl<T: Copy> InIterator<T> for Vec<T> {
///     fn each<F: Fn(T) -> T>(&mut self, f: F) {
///         let mut i = 0;
///         while i < self.len() {
///             self[i] = f(self[i]);
///             i += 1;
///         }
///     }
/// }
///
/// fn main(){
///     let mut v = vec![1,2,3];
///     v.each(|i| i * 3);
///     assert_eq!([3, 6, 9], &v[..3]);
/// }
/// ```
///
/// Base usage: 外部迭代器 for
///
/// ```rust
/// fn main() {
///     let v = vec![1, 2, 3, 4, 5];
///     for i in v {
///         println!("{}", i);
///     }
/// }
/// ```
///
/// Base usage:  for循环的等价代码
///
/// ```rust
/// fn main() {
///     let v = vec![1, 2, 3, 4, 5];
///     {  // 等价于for循环的scope
///         let mut _iterator = v.into_iter();
///         loop {
///             match _iterator.next() {
///                 Some(i) => {
///                     println!("{}", i);
///                 }
///                None => break,
///            }
///        }
///    }
/// }
/// ```
///
/// Base usage:  为自定义结构体实现Iterator
///
/// ```rust
/// struct Counter {
///     count: usize,
/// }
/// impl Iterator for Counter {
///     type Item = usize;
///     fn next(&mut self) -> Option<usize> {
///         self.count += 1;
///         if self.count < 6 {
///             Some(self.count)
///        } else {
///            None
///        }
///    }
/// }
/// fn main() {
///    let mut counter = Counter { count: 0 };
///    assert_eq!(Some(1), counter.next());
///    assert_eq!(Some(2), counter.next());
///    assert_eq!(Some(3), counter.next());
///    assert_eq!(Some(4), counter.next());
///    assert_eq!(Some(5), counter.next());
///    assert_eq!(None, counter.next());
/// }
/// ```
///
/// Base usage:  size_hint方法，可获取迭代器剩余长度的边界信息
///
/// ```rust
/// fn main() {
///     let a : [i32; 3]= [1, 2, 3];
///     let mut iter = a.iter();
///     assert_eq!((3, Some(3)), iter.size_hint());
///     iter.next();
///     assert_eq!((2, Some(2)), iter.size_hint());
/// }
/// ```
///
/// Base usage: 切片实现了Iter和IterMut
///
/// ```rust
/// fn main(){
///     let arr = [1, 2, 3, 4, 5];
///     for i in arr.iter() {
///         println!("{:?}", i);
///     }
///     println!("{:?}", arr);
/// }
/// ```
///
/// Base usage: Map是Rust里最常见的迭代器适配器
///
/// 此示例还包括了其他适配器
/// ```rust
/// fn main() {
///     let arr1 = [1, 2, 3, 4, 5];
///     let c1 = arr1.iter().map(|x| 2 * x).collect::<Vec<i32>>();
///     assert_eq!(&c1[..], [2, 4, 6, 8, 10]);
///     let arr2 = ["1", "2", "3", "h"];
///     let c2 = arr2.iter().filter_map(|x| x.parse().ok())
///                           .collect::<Vec<i32>>();
///     assert_eq!(&c2[..], [1,2,3]);
///     let arr3 = ['a', 'b', 'c'];
///    for (idx, val) in arr3.iter().enumerate() {
///        println!("idx: {:?}, val: {}", idx, val.to_uppercase());
///    }
/// }
/// ```
///
/// Base usage: Rev 反向迭代器适配器
///
/// ```rust
/// fn main() {
///     let a = [1, 2, 3];
///     let mut iter = a.iter().rev();
///     assert_eq!(iter.next(), Some(&3));
///     assert_eq!(iter.next(), Some(&2));
///     assert_eq!(iter.next(), Some(&1));
///     assert_eq!(iter.next(), None);
/// }
/// ```
///
/// Base usage: next_back方法示意
///
/// ```rust
/// fn main() {
///     let numbers = vec![1, 2, 3, 4, 5, 6];
///     let mut iter = numbers.into_iter();
///     assert_eq!(Some(1), iter.next());
///     assert_eq!(Some(6), iter.next_back());
///     assert_eq!(Some(5), iter.next_back());
///     assert_eq!(Some(2), iter.next());
///     assert_eq!(Some(3), iter.next());
///     assert_eq!(Some(4), iter.next());
///    assert_eq!(None, iter.next());
///    assert_eq!(None, iter.next_back());
/// }
/// ```
///
/// Base usage: 消费器any和fold示意
///
/// ```rust
/// fn main() {
///     let a = [1, 2, 3];
///     assert_eq!(a.iter().any(|&x| x != 2), true);
///     let sum = a.iter().fold(0, |acc, x| acc + x);
///     assert_eq!(sum, 6);
/// }
/// ```
///
/// Base usage: any示例
///
/// ```rust
/// fn main() {
///     let arr = [1, 2, 3];
///     let result1 = arr.iter().any(|&x| x != 2);
///     let result2 = arr.iter().any(|x| *x != 2);
///     // error:
///     // the trait bound `&{integer}: std::cmp::PartialEq<{integer}>` is not satisfied
///     // let result2 = arr.iter().any(|x| x != 2);
///     assert_eq!(result1, true);
///     assert_eq!(result2, true);
/// }
/// ```
///
/// Base usage: 使用fold对数组求和
///
/// ```rust
/// fn main() {
///     let arr = vec![1, 2, 3];
///     let sum1 = arr.iter().fold(0, |acc, x| acc + x);
///     let sum2 = arr.iter().fold(0, |acc, x| acc + *x);
///     let sum3 = arr.iter().fold(0, |acc, &x| acc + x);
///     let sum4 = arr.into_iter().fold(0, |acc, x| acc + x);
///     assert_eq!(sum1, 6);
///     assert_eq!(sum2, 6);
///     assert_eq!(sum3, 6);
///    assert_eq!(sum4, 6);
/// }
/// ```
///
/// Base usage: 自定义集合MyVec实现FromIterator
///
///  实现FromIterator就可以拥有Collect的能力
///
/// ```rust
/// use std::iter::FromIterator;
/// #[derive(Debug)]
/// struct MyVec(Vec<i32>);
/// impl MyVec {
///     fn new() -> MyVec {
///         MyVec(Vec::new())
///     }
///     fn add(&mut self, elem: i32) {
///         self.0.push(elem);
///    }
/// }
/// impl FromIterator<i32> for MyVec {
///    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
///        let mut c = MyVec::new();
///        for i in iter {
///            c.add(i);
///        }
///        c
///    }
/// }
/// fn main() {
///    let iter = (0..5).into_iter();
///    let c = MyVec::from_iter(iter);
///    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
///    let iter = (0..5).into_iter();
///    let c: MyVec = iter.collect();
///    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
///    let iter = (0..5).into_iter();
///    let c = iter.collect::<MyVec>();
///    assert_eq!(c.0, vec![0, 1, 2, 3, 4]);
/// }
/// ```
///
/// Base usage: 自定义适配器
///
/// ```rust
/// #[derive(Clone, Debug)]
/// #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
/// pub struct Step<I> {
///     iter: I,
///     skip: usize,
/// }
/// impl<I> Iterator for Step<I>
///     where I: Iterator,
///     {
///     type Item = I::Item;
///     fn next(&mut self) -> Option<I::Item> {
///         let elt = self.iter.next();
///         if self.skip > 0 {
///             self.iter.nth(self.skip - 1);
///         }
///        elt
///    }
/// }
/// pub fn step<I>(iter: I, step: usize) -> Step<I>
/// where I: Iterator,
/// {
///     assert!(step != 0);
///     Step {
///         iter: iter,
///             skip: step - 1,
///     }
/// }
/// pub trait IterExt: Iterator {
///     fn step(self, n: usize) -> Step<Self>
///     where Self: Sized,
///     {
///         step(self, n)
///     }
/// }
/// impl<T: ?Sized> IterExt for T where T: Iterator {}
/// fn main() {
///     let arr = [1,2,3,4,5,6];
///     let sum = arr.iter().step(2).fold(0, |acc, x| acc + x);
///     assert_eq!(9, sum); // [1, 3, 5]
/// }
/// ```
///
/// Base usage: 第三方包Itertools中实现的Positions适配器示例
///
/// ```rust
/// #[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
/// #[derive(Debug)]
/// pub struct Positions<I, F> {
///     iter: I,
///     f: F,
///     count: usize,
/// }
/// pub fn positions<I, F>(iter: I, f: F) -> Positions<I, F>
///     where I: Iterator,
///    F: FnMut(I::Item) -> bool,
///    {
///    Positions {
///        iter: iter,
///        f: f,
///        count: 0
///    }
/// }
/// impl<I, F> Iterator for Positions<I, F>
///    where I: Iterator,
///    F: FnMut(I::Item) -> bool,
/// {
///    type Item = usize;
///    fn next(&mut self) -> Option<Self::Item> {
///        while let Some(v) = self.iter.next() {
///            let i = self.count;
///            self.count = i + 1;
///            if (self.f)(v) {
///                return Some(i);
///            }
///       }
///       None
///    }
///    fn size_hint(&self) -> (usize, Option<usize>) {
///        (0, self.iter.size_hint().1)
///    }
/// }
/// impl<I, F> DoubleEndedIterator for Positions<I, F>
///    where I: DoubleEndedIterator + ExactSizeIterator,
///    F: FnMut(I::Item) -> bool,
/// {
///    fn next_back(&mut self) -> Option<Self::Item> {
///        while let Some(v) = self.iter.next_back() {
///            if (self.f)(v) {
///                return Some(self.count + self.iter.len())
///            }
///        }
///        None
///   }
/// }
/// pub trait Itertools: Iterator {
///    fn positions<P>(self, predicate: P) -> Positions<Self, P>
///    where Self: Sized,
///    P: FnMut(Self::Item) -> bool,
///    {
///        positions(self, predicate)
///    }
/// }
/// impl<T: ?Sized> Itertools for T where T: Iterator {}
/// fn main() {
///    let data = vec![1, 2, 3, 3, 4, 6, 7, 9];
///    let r = data.iter().positions(|v| v % 3 == 0);
///    let rev_r = data.iter().positions(|v| v % 3 == 0).rev();
///    for i in r { println!("{:?}", i); } // OUTPUT: 2  3  5 7
///    for i in rev_r { println!("{:?}", i); } // OUTPUT: 7 5 3 2
/// }
/// ```
pub fn iters(){
    trait InIterator<T: Copy> {
        fn each<F: Fn(T) -> T>(&mut self, f: F);
    }
    impl<T: Copy> InIterator<T> for Vec<T> {
        fn each<F: Fn(T) -> T>(&mut self, f: F) {
            let mut i = 0;
            while i < self.len() {
                self[i] = f(self[i]);
                i += 1;
            }
        }
    }
    fn main(){
        let mut v = vec![1,2,3];
        v.each(|i| i * 3);
        assert_eq!([3, 6, 9], &v[..3]);
    }

}
