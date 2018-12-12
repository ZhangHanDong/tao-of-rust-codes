use std::env;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::process;
use std::io;
use std::num;
use std::fmt;

#[derive(Debug)]
enum CliError {
    Io(io::Error),
    Parse(num::ParseIntError),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            CliError::Io(ref err) => write!(f, "IO error: {}", err),
            CliError::Parse(ref err) => write!(f, "Parse error: {}", err),
        }
    }
}

impl Error for CliError {
    fn description(&self) -> &str {
        match *self {
            CliError::Io(ref err) => err.description(),
            CliError::Parse(ref err) => Error::description(err),
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            CliError::Io(ref err) => Some(err),
            CliError::Parse(ref err) => Some(err),
        }
    }
}
impl From<io::Error> for CliError {
    fn from(err: io::Error) -> CliError {
        CliError::Io(err)
    }
}
impl From<num::ParseIntError> for CliError {
    fn from(err: num::ParseIntError) -> CliError {
        CliError::Parse(err)
    }
}
fn run(filename: &str) -> ParseResult<i32> {
    let mut file = File::open(filename)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut sum = 0;
    for c in contents.lines(){
        let n: i32 = c.parse::<i32>()?;
        sum += n;
    }
    Ok(sum)
}
type ParseResult<i32> = Result<i32, CliError>;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    println!("In file {}", filename);

    match run(filename) {
        Ok(n) => {
            println!("{:?}", n);
        },
        Err(e) => {
            println!("main error: {}", e);
            process::exit(1);
        }
    }
}