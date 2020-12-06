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
  let groups = advent_lib::group_lines_by_blanks(input);

  let mut unioned_sum = 0;

  for group in &groups {
    let mut yes_answers = BTreeSet::new();

    for line in group {
      for c in line.trim().chars() {
        yes_answers.insert(c);
      }
    }

    unioned_sum += yes_answers.len();
  }

  println!("{}", unioned_sum);

  let mut intersected_sum = 0;

  for group in &groups {
    let mut group_answers = BTreeSet::new();
    let mut is_first_person = true;

    for line in group {
      let mut person_answers = BTreeSet::new();

      for c in line.trim().chars() {
        person_answers.insert(c);
      }

      if is_first_person {
        group_answers = person_answers;
        is_first_person = false;
      } else {
        let mut intersected_answers = BTreeSet::new();
        for c in group_answers.intersection(&person_answers) {
          intersected_answers.insert(c.clone());
        }
        group_answers = intersected_answers;
      }
    }

    intersected_sum += group_answers.len();
  }

  println!("{}", intersected_sum);

  Ok(())
}
