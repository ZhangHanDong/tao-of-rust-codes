// 链接名为libsorting的静态库
#[link(name = "sorting", kind = "static")]
extern {
    // 绑定CPP中的interop_sort函数
    // 注意参数类型要和CPP中的类型相对应
    fn interop_sort(arr: &[i32;10], n: u32);
}

// 对interop_sort函数的安全抽象
pub fn sort_from_cpp(arr: &[i32;10], n: u32) {
    unsafe {
        interop_sort(arr, n);
    }
}
fn main() {
   let my_arr : [i32; 10]  = [10, 42, -9, 12, 8, 25, 7, 13, 55, -1];
   println!("Before sorting...");
   println!("{:?}\n", my_arr);
   sort_from_cpp(&my_arr, 10);
   println!("After sorting...");
   println!("{:?}", my_arr);
}