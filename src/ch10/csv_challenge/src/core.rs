pub mod read;
pub mod write;
use crate::err::Error;
use std::{
  path::PathBuf,
  fs::File,
  io::{Read, Write},
};
