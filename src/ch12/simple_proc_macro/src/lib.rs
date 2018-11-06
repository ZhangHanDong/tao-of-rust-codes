// #![feature(custom_attribute)]
// #![feature(proc_macro_non_items)]

extern crate proc_macro;
use self::proc_macro::TokenStream;

// 自定义派生属性
#[proc_macro_derive(A)]
pub fn derive(input: TokenStream) -> TokenStream {
    let _input = input.to_string();
    assert!(_input.contains("struct A;"));
    r#"
        impl A {
            fn a(&self) -> String{
               format!("hello from impl A")
           }
       }
   "#.parse().unwrap()
}

// 自定义编译器插件
#[proc_macro_attribute]
pub fn attr_with_args(args: TokenStream, input: TokenStream)
    -> TokenStream {
    let args = args.to_string();
    let _input = input.to_string();
    format!("fn foo() -> &'static str {{ {} }}", args)
        .parse().unwrap()
}

// Bang宏：定义hashmap!
#[proc_macro]
pub fn hashmap(input: TokenStream) -> TokenStream {
    // 转换input为字符串
    let _input = input.to_string();
    // 将input字符串结尾的逗号去掉，否则在下面迭代中将报错
    let input = _input.trim_right_matches(',');
    // 用split将字符串分割为slice，然后用map去处理
    // 为了支持「"a" : 1」或 「"a" => 1」这样的语法
    let input: Vec<String> = input.split(",").map(|n| {
        let mut data = if n.contains(":") {  n.split(":") }
                       else { n.split(" => ") };
        let (key, value) =
           (data.next().unwrap(), data.next().unwrap());
       format!("hm.insert({}, {})", key, value)
    }).collect();
    let count: usize = input.len();
    let tokens = format!("
        {{
        let mut hm =
            ::std::collections::HashMap::with_capacity({});
            {}
            hm
        }}", count,
        input.iter().map(|n| format!("{};", n)).collect::<String>()
    );
    // parse函数会将字符串转为Result<TokenStream>
    tokens.parse().unwrap()
}
