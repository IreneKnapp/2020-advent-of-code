use advent_lib::prelude::*;

//use std::collections::BTreeSet;
//use std::convert::TryFrom;



fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let card_public = input[0].parse::<i64>().unwrap();
  let door_public = input[1].parse::<i64>().unwrap();
  let subject: i64 = 7;

  let mut i = 0;
  let mut value = 1;
  let mut card_loop_size = None;
  let mut door_loop_size = None;
  loop {
    value = iterate(value, subject);
    i += 1;

    if card_loop_size.is_none() {
      if value == card_public {
        card_loop_size = Some(i);
      }
    }

    if door_loop_size.is_none() {
      if value == door_public {
        door_loop_size = Some(i);
      }
    }

    if card_loop_size.is_some() && door_loop_size.is_some() {
      break;
    }
  }

  let key = transform(door_public, card_loop_size.unwrap());

  println!("{}", key);

  Ok(())
}


fn iterate(input: i64, subject: i64) -> i64 {
  let mut output = input * subject;
  output %= 20201227;
  output
}


fn transform(subject: i64, loop_count: usize) -> i64 {
  let mut value = 1;
  for _ in 0 .. loop_count {
    value = iterate(value, subject);
  }
  value
}

