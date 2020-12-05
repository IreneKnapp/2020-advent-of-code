use advent_lib::prelude::*;

use std::collections::BTreeSet;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut max_id = 0;
  let mut found_ids = BTreeSet::new();

  for line in &input {
    let (row, column) = decode_seat(&line);
    let id = seat_id(row, column);

    found_ids.insert(id);

    if id > max_id {
      max_id = id;
    }

  }

  println!("{}", max_id);

  let mut is_started = false;

  for i in 0 .. max_id {
    if !is_started {
      if found_ids.contains(&i) {
        is_started = true;
      }
    } else {
      if !found_ids.contains(&i) {
        println!("{}", i);
        break;
      }
    }
  }

  Ok(())
}

fn decode_seat(seat_spec: &str) -> (i64, i64) {
  let mut row: i64 = 0;
  let mut column: i64 = 0;

  if seat_spec.chars().nth(0) == Some('B') {
    row += 64;
  }
  if seat_spec.chars().nth(1) == Some('B') {
    row += 32;
  }
  if seat_spec.chars().nth(2) == Some('B') {
    row += 16;
  }
  if seat_spec.chars().nth(3) == Some('B') {
    row += 8;
  }
  if seat_spec.chars().nth(4) == Some('B') {
    row += 4;
  }
  if seat_spec.chars().nth(5) == Some('B') {
    row += 2;
  }
  if seat_spec.chars().nth(6) == Some('B') {
    row += 1;
  }
  if seat_spec.chars().nth(7) == Some('R') {
    column += 4;
  }
  if seat_spec.chars().nth(8) == Some('R') {
    column += 2;
  }
  if seat_spec.chars().nth(9) == Some('R') {
    column += 1;
  }

  (row, column)
}


fn seat_id(row: i64, column: i64) -> i64 {
  row * 8 + column
}
