use advent_lib::prelude::*;

use regex::Regex;
use std::convert::TryInto;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let regex = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]*)$").unwrap();

  let mut valid_passwords = 0;

  for line in &input {
    let captures = regex.captures(line).unwrap();
    let min = captures[1].parse::<i64>()?;
    let max = captures[2].parse::<i64>()?;
    let required_char = captures[3].chars().nth(0);
    let password = &captures[4];

    let mut occurrences = 0;
    for c in password.chars() {
      if Some(c) == required_char {
        occurrences += 1;
      }
    }

    if occurrences >= min && occurrences <= max {
      valid_passwords += 1;
    }
  }

  println!("{}", valid_passwords);

  let mut valid_passwords_2 = 0;

  for line in &input {
    let captures = regex.captures(line).unwrap();
    let index_a = captures[1].parse::<i64>()?;
    let index_b = captures[2].parse::<i64>()?;
    let required_char = captures[3].chars().nth(0);
    let password = &captures[4];

    let a_is_match =
            password.len() >= index_a.try_into().unwrap()
            && required_char
               == password.chars().nth((index_a - 1).try_into().unwrap());
    let b_is_match =
            password.len() >= index_b.try_into().unwrap()
            && required_char
               == password.chars().nth((index_b - 1).try_into().unwrap());
    let mut occurrences = 0;
    if a_is_match {
      occurrences += 1;
    }
    if b_is_match {
      occurrences += 1;
    }

    if occurrences == 1 {
      valid_passwords_2 += 1;
    }
  }

  println!("{}", valid_passwords_2);

  Ok(())
}
