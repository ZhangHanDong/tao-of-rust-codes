// #![crate_type="dylib"]
#![feature(plugin_registrar, rustc_private)]

extern crate syntax;
extern crate rustc;
extern crate rustc_plugin;

use self::syntax::parse::token;
use self::syntax::tokenstream::TokenTree;
use self::syntax::ext::base::{ExtCtxt, MacResult, DummyResult, MacEager};
use self::syntax::ext::build::AstBuilder;  
use self::syntax::ext::quote::rt::Span;
use self::rustc_plugin::Registry;

static ROMAN_NUMERALS: &'static [(&'static str, usize)] = &[
        ("M", 1000), ("CM", 900), ("D", 500), ("CD", 400),
        ("C",  100), ("XC",  90), ("L",  50), ("XL",  40),
        ("X",   10), ("IX",   9), ("V",   5), ("IV",   4),
        ("I",    1)];

//  计算罗马数字，转换为阿拉伯数字
fn expand_roman(cx: &mut ExtCtxt, sp: Span, args: &[TokenTree])
        -> Box<MacResult + 'static> {

    if args.len() != 1 {
        // 验证传入的参数不能为空，或者多于1个参数   
        cx.span_err(
            sp,
            &format!("argument should be a single identifier, but got {} arguments", args.len()));
        return DummyResult::any(sp); // DummyResult为trait object，传递出错的位置信息
    }

    let text = match args[0] {
        // 验证传入的参数必须是标识符 token::Ident
        TokenTree::Token(_, token::Ident(s, _)) => s.to_string(),
        _ => {
            cx.span_err(sp, "argument should be a single identifier");
            return DummyResult::any(sp);
        }
    };
    let mut text = &*text;
    let mut total = 0;
    while !text.is_empty() {
        match ROMAN_NUMERALS.iter().find(|&&(rn, _)| text.starts_with(rn)) {
            Some(&(rn, val)) => {
                total += val;
                text = &text[rn.len()..];
            }
            None => {
                cx.span_err(sp, "invalid Roman numeral");
                return DummyResult::any(sp);
            }
        }
    }
    // MacEager枚举类型，记录了Rust语法结构
    // 此处为表达式类型，返回usize类型total和位置信息sp
    MacEager::expr(cx.expr_usize(sp, total))
}

#[plugin_registrar]
pub fn plugin_registrar_demo(reg: &mut Registry) {
    // 使用register_macro注册为宏
    reg.register_macro("roman_to_digit", expand_roman);
}