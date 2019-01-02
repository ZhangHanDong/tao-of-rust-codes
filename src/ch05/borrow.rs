/// # 借用与引用
///
/// Base usage: 为函数传递数组，使用可变绑定作为参数
///    因为传入的是不可变，而经过函数参数的模式匹配，成为了可变参数
/// ```rust
/// fn foo(mut v: [i32; 3]) -> [i32; 3] {
///     v[0] = 3;
///     assert_eq!([3,2,3], v);
///     v
/// }
/// fn main() {
///     let v = [1,2,3];
///     foo(v);
///     assert_eq!([1,2,3], v);
/// }
/// ```
///
/// Base usage: 以可变引用作为参数
///
/// ```rust
/// fn foo(v: &mut [i32; 3]) {
///     v[0] = 3;
/// }
/// fn main() {
///     let mut v = [1,2,3];
///     foo(&mut v);
///     assert_eq!([3,2,3], v);
/// }
/// ```
pub fn borrow(){
    fn foo(mut v: [i32; 3]) -> [i32; 3] {
        v[0] = 3;
        assert_eq!([3,2,3], v);
        v
    }
    fn main() {
        let v = [1,2,3];
        foo(v);
        assert_eq!([1,2,3], v);
    }

}

/// # 借用与引用
///
/// Base usage: 冒泡排序
///
/// ```rust
/// fn bubble_sort(a: &mut Vec<i32>) {
///     let mut n = a.len(); // 获取数组长度
///     while n > 0 {
///         // 初始化遍历游标，max_ptr始终指向最大值
///         let (mut i, mut max_ptr) = (1, 0);
///         // 冒泡开始，如果前者大于后者则互换位置，并设置当前最大值游标
///         while i < n {
///             if a[i-1] > a[i] {
///                 a.swap(i-1, i);
///                 max_ptr = i;
///             }
///             i += 1;
///         }
///         // 本次遍历的最大值位置即是下一轮冒泡的终点
///         n = max_ptr;
///     }
/// }
/// fn main() {
///     let mut a = vec![1, 4, 5, 3, 2];
///     bubble_sort(&mut a);
///     println!("{:?}", a); // [1, 2, 3, 4, 5]
/// }
/// ```
pub fn bubble_sort_demo(){
    fn bubble_sort(a: &mut Vec<i32>) {
        let mut n = a.len(); // 获取数组长度
        while n > 0 {
            // 初始化遍历游标，max_ptr始终指向最大值
            let (mut i, mut max_ptr) = (1, 0);
            // 冒泡开始，如果前者大于后者则互换位置，并设置当前最大值游标
            while i < n {
                if a[i-1] > a[i] {
                    a.swap(i-1, i);
                    max_ptr = i;
                }
                i += 1;
            }
            // 本次遍历的最大值位置即是下一轮冒泡的终点
            n = max_ptr;
        }
    }
    fn main() {
        let mut a = vec![1, 4, 5, 3, 2];
        bubble_sort(&mut a);
        println!("{:?}", a); // [1, 2, 3, 4, 5]
    }

}

/// # 借用检查
///
/// Base usage: 借用检查保证了内存安全
///    试想，input 和 output 如果将同一个变量的 不可变和可变引用同时传入会发生什么？
///
/// ```rust
/// fn compute(input: &u32, output: &mut u32) {
///     if *input > 10 {
///         *output = 1;
///     }
///     if *input > 5 {
///         *output *= 2;
///     }
/// }
/// fn main() {
///     let i = 20;
///     let mut o = 5;
///     compute(&i, &mut o); // o = 2
///  // let mut i = 20;
///  // compute(&i, &mut i);  // 借用检查不会允许该行编译成功

/// }
/// ```
///
/// Base usage: 优化该函数
/// ```
/// fn compute(input: &u32, output: &mut u32) {
///     let cached_input = *input;
///     if cached_input > 10 {
///         *output = 2;
///     } else if cached_input > 5 {
///         *output *= 2;
///     }
/// }
/// fn main() {
///    let i = 20;
///    let mut o = 5;
///    compute(&i, &mut o); // o = 2
/// }
/// ```
pub fn borrow_check(){
    fn compute(input: &u32, output: &mut u32) {
        if *input > 10 {
            *output = 1;
        }
        if *input > 5 {
            *output *= 2;
        }
    }
    fn main() {
        let i = 20;
        let mut o = 5;
        compute(&i, &mut o); // o = 2
    }

}

/// # 解引用操作将获得所有权
///
/// Base usage: 解引用移动语义类型需要注意
///
/// ```rust
/// fn join(s: &String) -> String {
///     let append = *s;
///     "Hello".to_string() + &append
/// }
/// fn main(){
///     let x = " hello".to_string();
///     join(&x);
/// }
/// ```
pub fn deref_move_type(){
    // fn join(s: &String) -> String {
    //     let append = *s;
    //     "Hello".to_string() + &append
    // }
    // fn main(){
    //     let x = " hello".to_string();
    //     join(&x);
    // }
}
