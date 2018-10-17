extern crate cc;
fn main() {
    // 相当于执行 
    // 1. `g++ -Wall -std=c++14 -c sorting.cpp`，使用g++编译sorting.cpp文件
    // 2. `ar rc libsorting.a sorting.o`，通过ar制作一份静态库libsorting.a
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .flag("-Wall")
        .flag("-std=c++14")
        .flag("-c")
        .file("cpp_src/sorting.cpp")
        .compile("sorting");
    // 也可以使用Command::new("g++")来组装命令，但是cc更方便。
}