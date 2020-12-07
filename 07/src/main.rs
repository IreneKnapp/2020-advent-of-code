use advent_lib::prelude::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut rules: BTreeMap<String, BTreeSet<(i64, String)>> =
      BTreeMap::new();

  for line in &input {
    let mut words = Vec::new();
    for word in line.split(' ') {
      words.push(word);
    }

    let key = two_words(words[0], words[1]);
    let mut value = BTreeSet::new();

    if words[4] != "no" {
      let mut i = 4;
      while i + 2 < words.len() {
        let quantity = words[i].parse::<i64>()?;
        let color = two_words(words[i+1], words[i+2]);
        value.insert((quantity, color));
        i += 4;
      }
    }

    rules.insert(key, value);
  }

  let mut closure: BTreeSet<String> = BTreeSet::new();

  for (container, rhs) in rules.iter() {
    for (_, containee) in rhs.iter() {
      if containee == "shiny gold" {
        closure.insert(container.to_string());
      }
    }
  }

  loop {
    let mut added_any = false;
    for (container, rhs) in rules.iter() {
      if closure.contains(container) {
        continue;
      }

      for (_, containee) in rhs.iter() {
        if closure.contains(containee) {
          closure.insert(container.to_string());
          added_any = true;
        }
      }
    }

    if !added_any {
      break;
    }
  }

  println!("{}", closure.len());

  println!("{}", recursive_count(&rules, "shiny gold".to_string()) - 1);

  Ok(())
}


fn two_words(a: &str, b: &str) -> String {
  let mut result = String::new();
  result.push_str(a);
  result.push_str(" ");
  result.push_str(b);
  result
}


fn recursive_count(rules: &BTreeMap<String, BTreeSet<(i64, String)>>, container: String) -> i64 {
  let mut sum = 1;

  let rhs: &BTreeSet<(i64, String)> = rules.get(&container).unwrap();
  for (quantity, containee) in rhs.iter() {
    let count_below = recursive_count(rules, containee.to_string());
    sum += quantity * count_below;
  }

  sum
}
