use advent_lib::prelude::*;

use std::collections::BTreeMap;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut memory = BTreeMap::new();
  let mut memory_2 = BTreeMap::new();
  let mut mask_select: i64 = 0;
  let mut mask_write: i64 = 0;

  for line in &input {
    if line.starts_with("mask = ") {
      mask_select = 0;
      mask_write = 0;
      for c in line.chars() {
        mask_select *= 2;
        mask_write *= 2;

        match c {
          'X' => { },
          '0' => {
            mask_select += 1;
          },
          '1' => {
            mask_select += 1;
            mask_write += 1;
          },
          _ => { },
        }
      }
    } else if line.starts_with("mem[") {
      let rest = line.strip_prefix("mem[").unwrap();
      let (address_string, rest) = rest.split_at(rest.find(']').unwrap());
      let address = address_string.parse::<i64>().unwrap();
      let rest = rest.strip_prefix("] = ").unwrap();
      let data = rest.parse::<i64>().unwrap();

      let masked_data = apply_mask(mask_select, mask_write, data);
      memory.insert(address, masked_data);

      let mut expanded_addresses = Vec::new();
      expanded_addresses.push(address);

      for i in 0..36 {
        if mask_select & (1 << i) == 0 {
          let mut new_addresses = Vec::new();
          for expanded_address in expanded_addresses {
            new_addresses.push(expanded_address | (1 << i));
            new_addresses.push(expanded_address & !(1 << i));
          }
          expanded_addresses = new_addresses;
        } else if mask_write & (1 << i) != 0 {
          let mut new_addresses = Vec::new();
          for expanded_address in expanded_addresses {
            new_addresses.push(expanded_address | (1 << i));
          }
          expanded_addresses = new_addresses;
        }
      }

      for expanded_address in expanded_addresses {
        memory_2.insert(expanded_address, data);
      }
    }
  }

  let mut sum: i64 = 0;
  for data in memory.values() {
    sum += data;
  }

  println!("{}", sum);

  let mut sum: i64 = 0;
  for data in memory_2.values() {
    sum += data;
  }

  println!("{}", sum);

  Ok(())
}

fn apply_mask(mask_select: i64, mask_write: i64, data: i64) -> i64 {
  let masked_data = (data & !mask_select) | mask_write;
  masked_data
}


