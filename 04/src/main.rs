use advent_lib::prelude::*;

use regex::Regex;
use std::collections::BTreeSet;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let overall_regex = Regex::new(r"([a-z][a-z][a-z]):([^ \n]*)").unwrap();
  let year_regex = Regex::new(r"^[0-9][0-9][0-9][0-9]$").unwrap();
  let height_regex = Regex::new(r"^([0-9]+)(cm|in)$").unwrap();
  let rgb_regex = Regex::new(r"^#[0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f][0-9a-f]$").unwrap();
  let eye_regex = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
  let id_regex = Regex::new(r"^[0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9][0-9]$").unwrap();

  let mut n_valid: i64 = 0;
  let mut current_fields: BTreeSet<String> = BTreeSet::new();
  let mut current_is_valid = true;

  for line in &input {
    if line.trim().len() == 0 {
      //println!("{:?}", current_fields);

      if current_is_valid
        && current_fields.contains("byr")
        && current_fields.contains("iyr")
        && current_fields.contains("eyr")
        && current_fields.contains("hgt")
        && current_fields.contains("hcl")
        && current_fields.contains("ecl")
        && current_fields.contains("pid") {
        n_valid += 1;
      }

      current_fields = BTreeSet::new();
      current_is_valid = true;
    }

    for captures in overall_regex.captures_iter(line) {
      let mut field_name = String::new();
      field_name.push_str(&captures[1]);

      let mut field_value = String::new();
      field_value.push_str(&captures[2]);

      current_fields.insert(field_name);

      match &captures[1] {
        "byr" => {
          if year_regex.is_match(&field_value) {
            let year = field_value.parse::<i64>()?;
            if year < 1920 || year > 2002 {
              current_is_valid = false;
            }
          } else {
            current_is_valid = false;
          }
        },
        "iyr" => {
          if year_regex.is_match(&field_value) {
            let year = field_value.parse::<i64>()?;
            if year < 2010 || year > 2020 {
              current_is_valid = false;
            }
          } else {
            current_is_valid = false;
          }
        },
        "eyr" => {
          if year_regex.is_match(&field_value) {
            let year = field_value.parse::<i64>()?;
            if year < 2020 || year > 2030 {
              current_is_valid = false;
            }
          } else {
            current_is_valid = false;
          }
        },
        "hgt" => {
          if height_regex.is_match(&field_value) {
            let captures = height_regex.captures(&field_value).unwrap();
            let number = captures[1].parse::<i64>()?;

            match &captures[2] {
              "cm" => {
                if number < 150 || number > 193 {
                  current_is_valid = false;
                }
              },
              "in" => {
                if number < 59 || number > 76 {
                  current_is_valid = false;
                }
              },
              _ => { },
            }
          } else {
            current_is_valid = false;
          }
        },
        "hcl" => {
          if !rgb_regex.is_match(&field_value) {
            current_is_valid = false;
          }
        },
        "ecl" => {
          if !eye_regex.is_match(&field_value) {
            current_is_valid = false;
          }
        },
        "pid" => {
          if !id_regex.is_match(&field_value) {
            current_is_valid = false;
          }
        },
        _ => {
        },
      };
    }
    //println!("end of line");
  }

  println!("{}", n_valid);

  Ok(())
}
