use advent_lib::prelude::*;

use std::collections::BTreeSet;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let mut input = advent_lib::read_int_file(&filename)?;

  input.sort();

  let mut input_set = BTreeSet::new();
  for item in &input {
    input_set.insert(item);
  }

  for i in 0 .. input.len() {
    let a = input[i];
    if a > 2020 {
      break;
    }

    let b = 2020 - a;
    if input_set.contains(&b) {
      let product = a * b;
      println!("a: {:?}, b: {:?}, a*b: {:?}", a, b, product);
      break;
    }
  }

  let mut done = false;
  for i in 0 .. input.len() {
    if done {
      break;
    }

    let a = input[i];
    if a > 2020 {
      break;
    }

    for j in i+1 .. input.len() {
      let b = input[j];

      if a + b > 2020 {
        break;
      }

      let c = 2020 - a - b;
      if input_set.contains(&c) {
        let product = a * b * c;
        println!("a: {:?}, b: {:?}, c: {:?}, a*b*c: {:?}", a, b, c, product);

        done = true;
        break;
      }
    }
  }

  Ok(())
}

