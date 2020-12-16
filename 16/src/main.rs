use advent_lib::prelude::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone,Debug)]
struct Range {
  min: i64,
  max: i64,
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

  let mut rules: BTreeMap<String,Vec<Range>> = BTreeMap::new();

  for line in &input[0] {
    let colon_point = line.find(':').unwrap();
    let (field_name, rest) = line.split_at(colon_point);
    let (_, rest) = rest.split_at(2);

    let mut ranges: Vec<Range> = Vec::new();
    for word in rest.split(' ') {
      if word == "or" {
        continue;
      }

      let parts: Vec<&str> = word.split('-').collect();
      ranges.push(Range {
        min: parts[0].parse::<i64>().unwrap(),
        max: parts[1].parse::<i64>().unwrap(),
      });

    }
    rules.insert(field_name.to_string(), ranges);
  }

  let player_ticket = parse_comma_separated_ints(&input[1][1]);

  let mut nearby_tickets = Vec::new();
  for line in input[2].iter().skip(1) {
    nearby_tickets.push(parse_comma_separated_ints(&line));
  }

  let mut valid_tickets = Vec::new();
  valid_tickets.push(player_ticket.clone());

  let mut error_sum = 0;
  for ticket in &nearby_tickets {
    let mut is_valid = true;
    for value in ticket {
      let mut found_match = false;
      for rule in rules.values() {
        if value_obeys_rule(*value, rule) {
          found_match = true;
          break;
        }
      }

      if !found_match {
        error_sum += value;
        is_valid = false;
      }
    }

    if is_valid {
      valid_tickets.push(ticket.clone());
    }
  }

  println!("{}", error_sum);

  let mut possibilities: Vec<BTreeSet<String>> = Vec::new();
  for _ in 0 .. player_ticket.len() {
    let mut all_fields = BTreeSet::new();
    for field_name in rules.keys() {
      all_fields.insert(field_name.clone());
    }
    possibilities.push(all_fields);
  }

  for ticket in valid_tickets {
    //println!("{:?}", possibilities);
    for i in 0 .. ticket.len() {
      let mut fields_to_remove = Vec::new();

      for field_name in possibilities[i].iter() {
        let rule = rules.get(field_name).unwrap();
        if !value_obeys_rule(ticket[i], rule) {
          /*
          println!("{}: value {} does not obey rule for {}",
                   i, ticket[i], field_name);
                   */
          fields_to_remove.push(field_name.clone());
        }
      }

      for field_name in fields_to_remove {
        possibilities[i].remove(&field_name);
      }
    }
  }

  //println!("{:?}", possibilities);

  let mut finalized_fields = BTreeSet::new();
  loop {
    let mut removed_any = false;

    for i in 0 .. player_ticket.len() {
      let mut unique_field = None;
      if possibilities[i].len() == 1 {
        let field_name = possibilities[i].iter().next().unwrap().clone();
        if finalized_fields.contains(&field_name) {
          continue;
        }
        unique_field = Some((i, field_name));
      }

      match unique_field {
        Some((i_to_skip, field_name)) => {
          removed_any = true;

          for j in 0 .. possibilities.len() {
            if j == i_to_skip {
              continue;
            }

            possibilities[j].remove(&field_name);
          }

          finalized_fields.insert(field_name);
        },
        None => { },
      }
    }

    if !removed_any {
      break;
    }
  }

  let mut product = 1;
  for i in 0 .. possibilities.len() {
    let field_name = possibilities[i].iter().next().unwrap();
    if field_name.starts_with("departure ") {
      product *= player_ticket[i];
    }
  }

  println!("{}", product);

  Ok(())
}


fn parse_comma_separated_ints(line: &str) -> Vec<i64> {
  let mut result = Vec::new();

  for word in line.split(',') {
    result.push(word.parse::<i64>().unwrap());
  }

  result
}


fn value_obeys_rule(value: i64, rule: &Vec<Range>) -> bool {
  for range in rule {
    if value >= range.min && value <= range.max {
      return true;
    }
  }
  false
}

