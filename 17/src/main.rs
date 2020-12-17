use advent_lib::prelude::*;

use std::convert::TryFrom;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;
  let input = advent_lib::group_lines_by_blanks(input);

  let mut initial_state: Vec<Vec<bool>> = Vec::new();

  for line in &input[0] {
    let mut row = Vec::new();
    for c in line.chars() {
      if c == '#' {
        row.push(true);
      } else {
        row.push(false);
      }
    }
    initial_state.push(row);
  }

  do_4d(&initial_state, false);
  do_4d(&initial_state, true);

  Ok(())
}


fn do_4d(initial_state: &Vec<Vec<bool>>, use_w: bool) {
  let max_iterations = 6;

  let mut hyperspace = init_4d(initial_state, max_iterations, use_w);

  for _ in 0 .. max_iterations {
    hyperspace = step_4d(&hyperspace, use_w);
  }

  let active_count = count_active_4d(&hyperspace);
  println!("{}", active_count);
}


fn init_4d(initial_state: &Vec<Vec<bool>>, max_iterations: usize, use_w: bool)
  -> Vec<Vec<Vec<Vec<bool>>>>
{
  let initial_height = initial_state.len();
  let initial_width = initial_state[0].len();

  let mut hyperspace: Vec<Vec<Vec<Vec<bool>>>> = Vec::new();
  for w in 0 .. max_iterations * 2 + 1 {
    if w > 0 && !use_w {
      continue;
    }

    let mut space = Vec::new();
    for z in 0 .. max_iterations * 2 + 1 {
      let mut plane = Vec::new();
      for y in 0 .. initial_height + max_iterations * 2 + 1 {
        let y_shifted = isize::try_from(y).unwrap()
          - isize::try_from(max_iterations).unwrap();

        let mut row = Vec::new();
        for x in 0 .. initial_width + max_iterations * 2 + 1 {
          let x_shifted = isize::try_from(x).unwrap()
            - isize::try_from(max_iterations).unwrap();

          if ((use_w && w == max_iterations) || (!use_w && w == 0))
            && z == max_iterations
            && y_shifted >= 0 && y_shifted < isize::try_from(initial_height).unwrap()
            && x_shifted >= 0 && x_shifted < isize::try_from(initial_width).unwrap()
          {
            row.push(initial_state[usize::try_from(y_shifted).unwrap()]
                                  [usize::try_from(x_shifted).unwrap()]);
          } else {
            row.push(false);
          }
        }

        plane.push(row);
      }

      space.push(plane);
    }

    hyperspace.push(space);
  }

  hyperspace
}


fn step_4d(input_hyperspace: &Vec<Vec<Vec<Vec<bool>>>>, use_w: bool)
  -> Vec<Vec<Vec<Vec<bool>>>>
{
  let mut output_hyperspace = Vec::new();

  for w in 0 .. input_hyperspace.len() {
    if w > 0 && !use_w {
      continue;
    }

    let input_space = &input_hyperspace[w];
    let mut output_space = Vec::new();

    for z in 0 .. input_space.len() {
      let input_plane = &input_space[z];
      let mut output_plane = Vec::new();

      for y in 0 .. input_plane.len() {
        let input_row = &input_plane[y];
        let mut output_row = Vec::new();

        for x in 0 .. input_row.len() {
          let mut adjacent_count = 0;

          for dw in [-1, 0, 1].iter() {
            for dz in [-1, 0, 1].iter() {
              for dy in [-1, 0, 1].iter() {
                for dx in [-1, 0, 1].iter() {
                  if *dw == 0 && *dz == 0 && *dy == 0 && *dx == 0 {
                   continue;
                  }

                  let shifted_w = isize::try_from(w).unwrap() + dw;
                  let shifted_z = isize::try_from(z).unwrap() + dz;
                  let shifted_y = isize::try_from(y).unwrap() + dy;
                  let shifted_x = isize::try_from(x).unwrap() + dx;

                  if shifted_w < 0
                    || shifted_w >= isize::try_from(input_hyperspace.len()).unwrap()
                  {
                    continue;
                  }
                  if shifted_z < 0
                    || shifted_z >= isize::try_from(input_space.len()).unwrap()
                  {
                    continue;
                  }
                  if shifted_y < 0
                    || shifted_y >= isize::try_from(input_plane.len()).unwrap()
                  {
                    continue;
                  }
                  if shifted_x < 0
                    || shifted_x >= isize::try_from(input_row.len()).unwrap()
                  {
                    continue;
                  }

                  if input_hyperspace[usize::try_from(shifted_w).unwrap()]
                                [usize::try_from(shifted_z).unwrap()]
                                [usize::try_from(shifted_y).unwrap()]
                                [usize::try_from(shifted_x).unwrap()]
                  {
                    adjacent_count += 1;
                  }
                }
              }
            }
          }

          let mut output_cell = false;

          if input_row[x] {
            if adjacent_count >= 2 && adjacent_count <= 3 {
              output_cell = true;
            }
          } else {
            if adjacent_count == 3 {
              output_cell = true;
            }
          }

          output_row.push(output_cell);
        }

        output_plane.push(output_row);
      }

      output_space.push(output_plane);
    }

    output_hyperspace.push(output_space);
  }

  output_hyperspace
}


fn count_active_4d(hyperspace: &Vec<Vec<Vec<Vec<bool>>>>) -> usize {
  let mut count = 0;

  for space in hyperspace {
    for plane in space {
      for row in plane {
        for cell in row {
          if *cell {
            count += 1;
          }
        }
      }
    }
  }

  count
}


pub fn debug_4d(hyperspace: &Vec<Vec<Vec<Vec<bool>>>>, use_w: bool) {
  let mut output = String::new();

  for w in 0 .. hyperspace.len() {
    if w > 0 && !use_w {
      continue;
    }

    let space = &hyperspace[w];
    for z in 0 .. space.len() {
      let plane = &space[z];

      if w > 0 || z > 0 {
        output.push_str("\n");
      }
      output.push_str(&format!("w: {}, z: {}\n", w, z));

      for row in plane {
        for cell in row {
          if *cell {
            output.push_str("#");
          } else {
            output.push_str(".");
          }
        }
        output.push_str("\n");
      }
    }
  }
  println!("{}", output);
}

