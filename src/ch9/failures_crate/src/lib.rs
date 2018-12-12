//！ # failure 使用代码示例
//!
//!  使用第三方failure库统一管理错误
// extern crate failure;
// #[macro_use] extern crate failure_derive;

use failure::{Backtrace, Context, Fail};
use std::fs::File;
use std::io::prelude::*;

/// # 使用Error来统一管理错误
#[derive(Debug)]
pub struct Error {
    inner: Context<ErrorKind>,
}
/// # ErrorKind记录各种错误
#[derive(Debug, Fail)]
pub enum ErrorKind {
    #[fail(display = "IoError")]
    Io(#[cause] std::io::Error),
    #[fail(display = "ParseError")]
    Parse(#[cause] std::num::ParseIntError),
    // ADD new Error Kind
}

impl Fail for Error {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }
    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.inner, f)
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error {
            inner: Context::new(ErrorKind::Io(err)),
        }
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(err: std::num::ParseIntError) -> Error {
        Error {
            inner: Context::new(ErrorKind::Parse(err)),
        }
    }
}

type ParseResult<i32> = Result<i32, Error>;

pub fn run(filename: Option<String>) -> ParseResult<i32> {
    let mut file = File::open(filename.unwrap())?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let mut sum = 0;
    for c in contents.lines() {
        let n: i32 = c.parse::<i32>()?;
        sum += n;
    }
    Ok(sum)
}
