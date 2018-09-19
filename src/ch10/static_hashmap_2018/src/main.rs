// #![feature(uniform_paths)]

mod read_func;
use self::read_func::{read_kv, rw_mut_kv};

fn main() {
    read_kv();
    match rw_mut_kv() {
        Ok(()) => {
            let m = read_func::static_kv::MAP_MUT.read().map_err(|e| e.to_string()).unwrap();
            assert_eq!("baz", *m.get(&1).unwrap_or(&read_func::static_kv::NF));
        }
        Err(e) => println!("Error {}", e),
    }
}
