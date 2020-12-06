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

  let mut input = Vec::new();
  loop {
    reader.read_line(&mut buffer)?;
    if buffer.len() == 0 {
      break;
    }

    let mut line_copy = String::new();
    match buffer.strip_suffix("\n") {
      Some(stripped) => {
        line_copy.push_str(stripped);
      }
      None => {
        line_copy.push_str(&buffer);
      }
    }
    input.push(line_copy);

    buffer.clear();
  }

  Ok(input)
}

pub fn group_lines_by_blanks(lines: Vec<String>) -> Vec<Vec<String>> {
  let mut all_groups = Vec::new();
  let mut current_group = Vec::new();

  for line in lines {
    if line.len() == 0 {
      all_groups.push(current_group);
      current_group = Vec::new();
    } else {
      current_group.push(line);
    }
  }

  if current_group.len() > 0 {
    all_groups.push(current_group);
  }

  all_groups
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
