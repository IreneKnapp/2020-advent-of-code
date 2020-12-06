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

pub fn trim_lines<'a>(lines: &'a Vec<String>) -> Vec<&str> {
  let mut trimmed_lines = Vec::new();

  for line in lines {
    match line.strip_suffix("\n") {
      Some(stripped) => {
        trimmed_lines.push(stripped);
      }
      None => {
        trimmed_lines.push(line);
      }
    }
  }

  trimmed_lines
}

pub fn group_lines_by_blanks(lines: Vec<&str>) -> Vec<Vec<&str>> {
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
