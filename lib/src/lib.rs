pub mod error;
pub mod prelude;

pub use crate::prelude::Result;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;


pub fn greeting() -> Result<()> {
  println!("Hello, Irenes!");

  Ok(())
}

pub fn read_lines_file(filename: &str) -> Result<Vec<String>> {
  let file = File::open(filename)?;
  let mut reader = BufReader::new(file);
  let mut buffer = String::new();

  let mut input: Vec<String> = Vec::new();
  loop {
    reader.read_line(&mut buffer)?;
    if buffer.len() == 0 {
      break;
    }

    let mut line_copy = String::new();
    line_copy.push_str(buffer.trim());
    input.push(line_copy);

    buffer.clear();
  }

  Ok(input)
}


pub fn read_int_file(filename: &str) -> Result<Vec<i64>> {
  let file = File::open(filename)?;
  let mut reader = BufReader::new(file);
  let mut buffer = String::new();

  let mut input: Vec<i64> = Vec::new();
  loop {
    reader.read_line(&mut buffer)?;
    if buffer.len() == 0 {
      break;
    }

    let item = buffer.trim().parse::<i64>()?;

    buffer.clear();

    input.push(item);
  }

  Ok(input)
}
