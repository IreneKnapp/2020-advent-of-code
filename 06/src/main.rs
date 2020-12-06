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

  let mut yes_counts = Vec::new();
  let mut yes_answers = BTreeSet::new();

  for line in &input {
    if line.trim().len() == 0 {
      let count = yes_answers.len();
      yes_counts.push(count);

      yes_answers = BTreeSet::new();
    }

    for c in line.trim().chars() {
      yes_answers.insert(c);
    }
  }

  let count = yes_answers.len();
  yes_counts.push(count);

  let mut sum = 0;
  for count in yes_counts {
    sum += count;
  }

  println!("{}", sum);

  let mut everyone_yes_counts = Vec::new();
  let mut everyone_yes_answers = BTreeSet::new();
  let mut is_within_group = false;

  for line in &input {
    let this_line_is_blank = line.trim().len() == 0;

    if this_line_is_blank {
      let count = everyone_yes_answers.len();
      everyone_yes_counts.push(count);
      //println!("{}", count);

      everyone_yes_answers = BTreeSet::new();
      is_within_group = false;
    }

    let mut this_person_yes_answers = BTreeSet::new();

    for c in line.trim().chars() {
      this_person_yes_answers.insert(c);
    }

    //println!("{} {}", everyone_yes_answers.len(), this_person_yes_answers.len());
    if is_within_group {
      let intersection =
          everyone_yes_answers.intersection(&this_person_yes_answers);

      let mut copied = BTreeSet::new();
      for c in intersection {
        copied.insert(c.clone());
      }

      everyone_yes_answers = copied;
    } else {
      everyone_yes_answers = this_person_yes_answers;
    }

    is_within_group = !this_line_is_blank;
  }

  let count = everyone_yes_answers.len();
  everyone_yes_counts.push(count);

  let mut everyone_sum = 0;
  for count in everyone_yes_counts {
    everyone_sum += count;
  }

  println!("{}", everyone_sum);

  Ok(())
}
