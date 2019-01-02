use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;
type ParseResult<i32> = Result<i32, Box<Error>>;
fn run(filename: &str) -> ParseResult<i32> {
    File::open(filename).map_err(|e| e.into())
    .and_then(|mut f|{
        let mut contents = String::new();
        f.read_to_string(&mut contents)
        .map_err(|e| e.into()).map(|_|contents)
    })
    .and_then(|contents|{
        let mut sum = 0;
       for c in contents.lines(){
           match c.parse::<i32>() {
               Ok(n) => {sum += n;},
               Err(err) => {
                   let err: Box<Error> = err.into();
                   println!("error info: {}, cause: {:?}"
                   , err.description(),err.cause());
               },
               // Err(err) => { return Err(From::from(err));},
           }
       }
       Ok(sum)
   })
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);
    match run(filename) {
       Ok(n) => { println!("{:?}", n); },
       Err(e) => {
           println!("main error: {}", e);
           process::exit(1);
       }
   }
}