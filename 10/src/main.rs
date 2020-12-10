use advent_lib::prelude::*;

use std::convert::TryFrom;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut adaptors: Vec<i64> = Vec::new();
  for line in &input {
    adaptors.push(line.parse::<i64>().unwrap());
  }
  adaptors.sort();

  let mut buckets = Vec::new();
  buckets.push(0);
  buckets.push(0);
  buckets.push(0);
  buckets.push(0);

  for i in 0 .. adaptors.len() {
    let mut prev_joltage = 0;
    if i > 0 {
      prev_joltage = adaptors[i - 1];
    }
    let difference = adaptors[i] - prev_joltage;
    buckets[usize::try_from(difference).unwrap()] += 1;
  }

  buckets[3] += 1;

  println!("{}", buckets[1]*buckets[3]);

  let mut reversed_adaptors = Vec::new();
  for i in 0 .. adaptors.len() {
    reversed_adaptors.push(adaptors[adaptors.len() - i - 1]);
  }
  reversed_adaptors.push(0);

  let mut memoized_counts: Vec<i64> = Vec::new();
  memoized_counts.push(1);

  for i in 1 .. reversed_adaptors.len() {
    let mut count = 0;
    for j in 1 .. i+1 {
      let difference = reversed_adaptors[i - j] - reversed_adaptors[i];
      if difference <= 3 {
        count += memoized_counts[i - j];
      } else {
        break;
      }
    }
    memoized_counts.push(count);
  }

  println!("{}", memoized_counts[memoized_counts.len()-1]);

  Ok(())
}

