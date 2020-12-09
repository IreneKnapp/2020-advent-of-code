use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut ciphertext: Vec<i64> = Vec::new();
  for line in &input {
    ciphertext.push(line.parse::<i64>().unwrap());
  }

  let mut first_key = 0;

  for i in 25 .. ciphertext.len() {
    let sum = ciphertext[i];
    let mut found_addends = false;

    for j in 0 .. 25 {
      let a = ciphertext[i - 25 + j];
      let b = sum - a;
      for k in j + 1 .. 25 {
        if ciphertext[i - 25 + k] == b {
          found_addends = true;
          break;
        }
      }
      if found_addends {
        break;
      }
    }

    if !found_addends {
      println!("{}", sum);
      first_key = sum;
      break;
    }
  }

  for run_length in 2 .. ciphertext.len() {
    let mut sum = 0;
    let found_solution = false;

    for i in 0 .. ciphertext.len() - run_length {
      sum += ciphertext[i];

      if i >= run_length {
        sum -= ciphertext[i - run_length];
      }

      if sum == first_key {
        let mut min = 0;
        let mut max = 0;

        for j in 0 .. run_length {
          let item = ciphertext[i - run_length + j + 1];
          if j == 0 || item < min {
            min = item;
          }
          if j == 0 || item > max {
            max = item;
          }
        }

        println!("{}", min + max);
        break;
      }
    }

    if found_solution {
      break;
    }
  }

  Ok(())
}

