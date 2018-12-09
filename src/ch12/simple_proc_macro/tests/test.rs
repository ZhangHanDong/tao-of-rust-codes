// #![feature(custom_attribute)]
// #![feature(proc_macro_non_items)]

#![feature(proc_macro_hygiene)]


use simple_proc_macro::{A, attr_with_args, hashmap};

// 自定义派生属性
#[derive(A)]
struct A;

// 自定义编译器插件
#[attr_with_args("Hello, Rust!")]
fn foo() {}

// 测试自定义派生属性
#[test]
fn test_derive_a() {
    assert_eq!("hello from impl A".to_string(), A.a());
}

// 测试自定义编译器插件
#[test]
fn test_foo() {
    let r = foo();
    assert_eq!(r, "Hello, Rust!") ;
}

// 测试Bang宏
// 过程宏-bang宏还无法用于表达式位置，所以下面测试暂时会报错，等支持的时候应该就会通过
// 相关issues：https://github.com/rust-lang/rust/issues/54727
// https://github.com/rust-lang/blog.rust-lang.org/issues/285
#[test]
fn test_hashmap() {
    let hm = hashmap!{ "a" => 1,"b" => 2,"c" => 3};
    assert_eq!(hm["c"], 3);
    let hm = hashmap!{ "a": 1, "b": 2,};
    assert_eq!(hm["a"], 1);
}
