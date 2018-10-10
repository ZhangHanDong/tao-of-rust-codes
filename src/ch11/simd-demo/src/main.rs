#![feature(stdsimd)] 

// use jetscii::{ascii_chars, bytes, Substring};

use ::std as real_std;
use stdsimd as std;
#[cfg(target_arch = "x86")]
use ::std::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use ::std::arch::x86_64::*;

// LLVM为指定的平台自动向量化为AVX2，生成优化的向量化代码
fn add_quickly(a: &[u8], b: &[u8], c: &mut [u8]) {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {
        if is_x86_feature_detected!("avx2") {
            return unsafe { add_quickly_avx2(a, b, c) }
        }
    }
    add_quickly_fallback(a, b, c)
}
// LLVM为指定的平台自动向量化为AVX2，生成优化的向量化代码
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[target_feature(enable = "avx2")]
unsafe fn add_quickly_avx2(a: &[u8], b: &[u8], c: &mut [u8]) {
    add_quickly_fallback(a, b, c)
}

fn add_quickly_fallback(a: &[u8], b: &[u8], c: &mut [u8]) {
    for ((a, b), c) in a.iter().zip(b).zip(c) {
        *c = *a + *b;
    }
}

fn main() {
    // let part_number = "8-67-J5-2:rev:1";
    // let first = ascii_chars!(':', '-').find(part_number);
    // assert_eq!(first, Some(1));

    // let raw_data = [0x00, 0x01, 0x10, 0xFF, 0x42];
    // let first = bytes!(0x01, 0x10).find(&raw_data);
    // assert_eq!(first, Some(1));

    // let colors = "red, blue, green";
    // let first = Substring::new(", ").find(colors);
    // assert_eq!(first, Some(3));

    // #[cfg(feature = "pattern")] {
    //     let colors = "red, blue, green";
    //     let colors: Vec<_> = colors.split(Substring::new(", ")).collect();
    //     assert_eq!(&colors, &["red", "blue", "green"]);
    // }

    if is_x86_feature_detected!("sse4.2") {
        #[target_feature(enable = "sse4.2")]
        unsafe fn worker() {
            // The string we want to search for with some
            // extra bytes we do not want to search for.
            let needle = b"\r\n\t ignore this ";

             // The string we want to find a substring in
            let haystack = b"Split a \r\n\t line  ";

            let a = _mm_loadu_si128(needle.as_ptr() as *const _);
            let b = _mm_loadu_si128(haystack.as_ptr() as *const _);

            // Note: We explicitly specify we only want to search `b` for the
            // first 3 characters of a.
            let idx = _mm_cmpestri(a, 3, b, 20, _SIDD_CMP_EQUAL_ORDERED);

            assert_eq!(idx, 8);
        }
        unsafe { worker(); }
    }
    // 使用LLVM自动向量化
    let mut dst = [0];
    add_quickly(&[1], &[2], &mut dst);
    assert_eq!(dst[0], 3);
}
