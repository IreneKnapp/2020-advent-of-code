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

  let timestamp = input[0].parse::<i64>()?;
  let mut busses: Vec<Option<i64>> = Vec::new();

  for word in input[1].split(',') {
    if word == "x" {
      busses.push(None);
    } else {
      busses.push(Some(word.parse::<i64>().unwrap()));
    }
  }

  let mut best_bus = 0;
  let mut best_delay = timestamp;

  for bus in &busses {
    match bus {
      Some(frequency) => {
        let delay = frequency - (timestamp % frequency);
        if delay < best_delay {
          best_bus = *frequency;
          best_delay = delay;
        }
      },
      None => { },
    }
  }

  println!("{}", best_bus * best_delay);

  let mut frequency_so_far = 0;
  let mut offset_so_far = 0;
  let mut skip_from_previous_bus = 0;

  for i in 0 .. busses.len() {
    match busses[i] {
      Some(frequency) => {
        if i == 0 {
          frequency_so_far = frequency;
          offset_so_far = 0;
        } else {
          // frequency_so_far * a + skip_from_previous_bus = frequency * b
          let mut proposed_offset = offset_so_far + skip_from_previous_bus;
          loop {
            if proposed_offset % frequency == 0 {
              offset_so_far = proposed_offset;
              frequency_so_far *= frequency;
              break;
            } else {
              proposed_offset += frequency_so_far;
            }
          }
        }

        skip_from_previous_bus = 1;
      },
      None => {
        skip_from_previous_bus += 1;
      },
    }
  }
  offset_so_far -= i64::try_from(busses.len()).unwrap() - 1;

  println!("{}", offset_so_far);

  Ok(())
}

