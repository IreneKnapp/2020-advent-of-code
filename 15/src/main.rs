use advent_lib::prelude::*;

use std::collections::BTreeMap;



fn main() -> Result<()> {
  let args = std::env::args();
  if args.len() != 1 {
    eprintln!("Usage: advent");
  }

  let starting_numbers = vec![2, 15, 0, 9, 1, 20];

  let mut history: BTreeMap<usize, usize> = BTreeMap::new();
  let mut output = 0;
  let mut next_output = 0;

  for i in 0 .. starting_numbers.len() {
    output = starting_numbers[i];
    next_output = match history.get(&output) {
      Some(previous) => i - previous,
      None => 0,
    };
    history.insert(output, i);
  }

  for i in starting_numbers.len() .. 2020 {
    output = next_output;
    next_output = match history.get(&output) {
      Some(previous) => i - previous,
      None => 0,
    };
    history.insert(output, i);
  }

  println!("{}", output);

  for i in 2020 .. 30000000 {
    output = next_output;
    next_output = match history.get(&output) {
      Some(previous) => i - previous,
      None => 0,
    };
    history.insert(output, i);
  }

  println!("{}", output);

  Ok(())
}

