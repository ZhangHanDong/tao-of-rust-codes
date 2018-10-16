// Rust 2015
// #[macro_use] extern crate hashmap_lite;

// Rust 2018
use hashmap_lite::hashmap;
use hashmap_lite as hm;

#[macro_use]
mod macros {
    macro_rules! X { () => { Y!(); } }
    macro_rules! Y { () => {} }
}

fn main(){
    let map = hashmap!{
        "a" => 1,
        "b" => 2,
    };
    assert_eq!(map["a"], 1);
    X!();
    assert_eq!(hm::inc!(1), 2);
}