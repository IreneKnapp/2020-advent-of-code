use advent_lib::prelude::*;

use std::collections::BTreeSet;
//use std::convert::TryFrom;

#[derive(Clone,Copy)]
enum Dir {
  East,
  West,
  Northeast,
  Northwest,
  Southeast,
  Southwest,
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut tile_flips: BTreeSet<(isize,isize)> = BTreeSet::new();
  for line in &input {
    let mut x: isize = 0;
    let mut y: isize = 0;
    let mut c_iter = line.chars();
    loop {
      match c_iter.next() {
        Some('e') => {
          let (new_x, new_y) = adjacent_cell(x, y, Dir::East);
          x = new_x;
          y = new_y
          //x += 2;
        },
        Some('w') => {
          let (new_x, new_y) = adjacent_cell(x, y, Dir::West);
          x = new_x;
          y = new_y
          //x -= 2;
        },
        Some('n') => {
          match c_iter.next() {
            Some('e') => {
              let (new_x, new_y) = adjacent_cell(x, y, Dir::Northeast);
              x = new_x;
              y = new_y
              /*
              if x % 2 != 0 {
                y += 1;
              }
              x += 1;
              */
            },
            Some('w') => {
              let (new_x, new_y) = adjacent_cell(x, y, Dir::Northwest);
              x = new_x;
              y = new_y
              /*
              if x % 2 != 0 {
                y += 1;
              }
              x -= 1;
              */
            },
            _ => { },
          }
        },
        Some('s') => {
          match c_iter.next() {
            Some('e') => {
              let (new_x, new_y) = adjacent_cell(x, y, Dir::Southeast);
              x = new_x;
              y = new_y
              /*
              if x % 2 == 0 {
                y -= 1;
              }
              x += 1;
              */
            },
            Some('w') => {
              let (new_x, new_y) = adjacent_cell(x, y, Dir::Southwest);
              x = new_x;
              y = new_y
              /*
              if x % 2 == 0 {
                y -= 1;
              }
              x -= 1;
              */
            },
            _ => { },
          }
        },
        None => break,
        _ => { },
      }
    }

    if !tile_flips.contains(&(x, y)) {
      tile_flips.insert((x, y));
    } else {
      tile_flips.remove(&(x, y));
    }
  }

  println!("{}", tile_flips.len());

  let mut state = tile_flips.clone();
  for _ in 0 .. 100 {
    state = iterate(state);
  }

  println!("{}", state.len());

  Ok(())
}


fn iterate(input_state: BTreeSet<(isize, isize)>) -> BTreeSet<(isize, isize)> {
  let mut min_x = 0;
  let mut min_y = 0;
  let mut max_x = 0;
  let mut max_y = 0;

  for (x, y) in &input_state {
    if *x > max_x {
      max_x = *x;
    }
    if *x < min_x {
      min_x = *x;
    }
    if *y > max_y {
      max_y = *y;
    }
    if *y < min_y {
      min_y = *y;
    }
  }

  max_x += 2;
  min_x -= 2;
  max_y += 1;
  min_y -= 1;

  let mut output_state: BTreeSet<(isize,isize)> = BTreeSet::new();

  for y in min_y .. max_y + 1 {
    for x in min_x .. max_x + 1 {
      let mut adjacent_count = 0;
      for direction in &[Dir::East, Dir::West, Dir::Northeast, Dir::Northwest,
                         Dir::Southeast, Dir::Southwest]
      {
        let adjacent_coordinates = adjacent_cell(x, y, *direction);
        if input_state.contains(&adjacent_coordinates) {
          adjacent_count += 1;
        }
      }

      if input_state.contains(&(x, y)) {
        if adjacent_count >= 1 && adjacent_count <= 2 {
          output_state.insert((x, y));
        }
      } else {
        if adjacent_count == 2 {
          output_state.insert((x, y));
        }
      }
    }
  }

  output_state
}


fn adjacent_cell(x: isize, y: isize, direction: Dir) -> (isize, isize) {
  match direction {
    Dir::East => (x + 2, y),
    Dir::West => (x - 2, y),
    Dir::Northeast => {
      if x % 2 != 0 {
        (x + 1, y + 1)
      } else {
        (x + 1, y)
      }
    },
    Dir::Northwest => {
      if x % 2 != 0 {
        (x - 1, y + 1)
      } else {
        (x - 1, y)
      }
    },
    Dir::Southeast => {
      if x % 2 == 0 {
        (x + 1, y - 1)
      } else {
        (x + 1, y)
      }
    },
    Dir::Southwest => {
      if x % 2 == 0 {
        (x - 1, y - 1)
      } else {
        (x - 1, y)
      }
    },
  }
}
