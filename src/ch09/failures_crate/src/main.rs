extern crate failures_crate;
use failures_crate::*;
use std::env;
fn main() -> Result<(), String> {
    let filename = env::args().nth(1);
    match failures_crate::run(filename) {
        Ok(n) => {
            println!("{:?}", n);
            return Ok(());
        },
        Err(e) => {
            return Err("1".to_string());
        }
    }
}
