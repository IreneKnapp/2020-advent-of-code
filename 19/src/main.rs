use advent_lib::prelude::*;

use std::collections::BTreeMap;
//use std::convert::TryFrom;

#[derive(Clone,Debug)]
enum Rule {
  Alternatives(Vec<Vec<i64>>),
  Character(char),
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;
  let input = advent_lib::group_lines_by_blanks(input);

  let mut rules: BTreeMap<i64,Rule> = BTreeMap::new();
  for line in &input[0] {
    let colon_point = line.find(':').unwrap();
    let (rule_number, rest) = line.split_at(colon_point);
    let rule_number = rule_number.parse::<i64>().unwrap();
    let (_, rest) = rest.split_at(2);
    let words: Vec<&str> = rest.split(' ').collect();

    let mut chars = words[0].chars();
    if words.len() == 1 && chars.next() == Some('"') {
      let c = chars.next().unwrap();

      rules.insert(rule_number, Rule::Character(c));
    } else {
      let alternatives_inputs = words.split(|word| *word == "|");

      let mut alternatives: Vec<Vec<i64>> = Vec::new();
      for alternative_words in alternatives_inputs {
        let mut alternative: Vec<i64> = Vec::new();
        for word in alternative_words {
          alternative.push(word.parse::<i64>().unwrap());
        }
        alternatives.push(alternative);
      }

      rules.insert(rule_number, Rule::Alternatives(alternatives));
    }
  }

  let mut match_count = 0;
  for line in &input[1] {
    if is_match(&rules, 0, line) {
      match_count += 1;
    }
  }

  println!("{:?}", match_count);

  rules.insert(8, Rule::Alternatives(vec![vec![42], vec![42, 8]]));
  rules.insert(11, Rule::Alternatives(vec![vec![42, 31], vec![42, 11, 31]]));

  let mut match_count = 0;
  for line in &input[1] {
    if is_match(&rules, 0, line) {
      match_count += 1;
    }
  }

  println!("{:?}", match_count);

  Ok(())
}


fn is_match(rules: &BTreeMap<i64,Rule>, root_rule: i64, input: &str) -> bool {
  let match_results = is_match_helper(rules, root_rule, input);
  for rest in match_results {
    if rest.len() == 0 {
      return true;
    }
  }
  false
}

fn is_match_helper<'a>(rules: &BTreeMap<i64,Rule>, root_rule: i64, input: &'a str)
  -> Vec<&'a str>
{
  let mut results = Vec::new();

  match rules.get(&root_rule).unwrap() {
    Rule::Alternatives(alternatives) => {
      for alternative in alternatives {
        let mut possible_rests = Vec::new();
        possible_rests.push(input);

        for sub_rule in alternative {
          let mut new_possible_rests = Vec::new();

          for rest in possible_rests {
            let sub_matches = is_match_helper(rules, *sub_rule, rest);
            for new_rest in sub_matches {
              new_possible_rests.push(new_rest);
            }
          }

          possible_rests = new_possible_rests;
        }

        for rest in possible_rests {
          results.push(rest);
        }
      }
    },
    Rule::Character(character) => {
      if input.len() >= 1 && input.chars().next() == Some(*character) {
        let (_, rest) = input.split_at(1);
        results.push(rest);
      }
    },
  }

  results
}

