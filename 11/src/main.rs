use advent_lib::prelude::*;

use std::convert::TryFrom;


#[derive(Clone,Debug,PartialEq)]
pub enum CellState {
  Floor,
  EmptySeat,
  FullSeat,
}


pub fn debug_grid(grid: &Vec<Vec<CellState>>) {
  let mut output = String::new();

  for row in grid {
    for cell in row {
      match cell {
        CellState::Floor => {
          output.push_str(".");
        }
        CellState::EmptySeat => {
          output.push_str("L");
        }
        CellState::FullSeat => {
          output.push_str("#");
        }
      }
    }
    output.push_str("\n");
  }
  output.push_str("\n");
  println!("{}", output);
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut original_grid: Vec<Vec<CellState>> = Vec::new();

  for line in &input {
    let mut row: Vec<CellState> = Vec::new();

    for c in line.chars() {
      if c == 'L' {
        row.push(CellState::EmptySeat);
      } else {
        row.push(CellState::Floor);
      }
    }

    original_grid.push(row);
  }

  let mut grid = original_grid.clone();
  let nearby_seats = compute_nearby_seats(&original_grid);

  loop {
    let (any_changed, new_grid) = simulate_one(&grid);
    if !any_changed {
      break;
    }
    grid = new_grid;
  }

  println!("{}", count_full_seats(&grid));

  let mut grid = original_grid.clone();

  let mut n_iterations = 0;

  loop {
    let (any_changed, new_grid) = simulate_distancey_one(&grid, &nearby_seats);
    if !any_changed {
      break;
    }
    grid = new_grid;
    n_iterations += 1;
    if n_iterations % 1000 == 0 {
      println!("iteration {}", n_iterations);
      debug_grid(&grid);
    }
  }

  println!("{}", count_full_seats(&grid));

  Ok(())
}


fn simulate_one(input_grid: &Vec<Vec<CellState>>) -> (bool, Vec<Vec<CellState>>) {
  let mut output_grid: Vec<Vec<CellState>> = Vec::new();
  let mut any_changed = false;

  for y in 0 .. input_grid.len() {
    let mut output_row: Vec<CellState> = Vec::new();

    for x in 0 .. input_grid[y].len() {
      let mut adjacent_count = 0;

      let mut nearby_x_min = x;
      if nearby_x_min > 0 {
        nearby_x_min -= 1;
      }

      let mut nearby_x_max = x;
      if nearby_x_max + 1 < input_grid[y].len() {
        nearby_x_max += 1;
      }

      let mut nearby_y_min = y;
      if nearby_y_min > 0 {
        nearby_y_min -= 1;
      }

      let mut nearby_y_max = y;
      if nearby_y_max + 1 < input_grid.len() {
        nearby_y_max += 1;
      }

      for nearby_x in nearby_x_min .. nearby_x_max + 1 {
        for nearby_y in nearby_y_min .. nearby_y_max + 1 {
          if nearby_x == x && nearby_y == y {
            continue;
          }

          let nearby_cell = &input_grid[nearby_y][nearby_x];

          if *nearby_cell == CellState::FullSeat {
            adjacent_count += 1;
          }
        }
      }

      let input_cell = &input_grid[y][x];
      if *input_cell == CellState::Floor {
        output_row.push(CellState::Floor);
      } else if *input_cell == CellState::EmptySeat && adjacent_count == 0 {
        output_row.push(CellState::FullSeat);
        any_changed = true;
      } else if *input_cell == CellState::FullSeat && adjacent_count >= 4 {
        output_row.push(CellState::EmptySeat);
        any_changed = true;
      } else {
        output_row.push(input_cell.clone());
      }
    }

    output_grid.push(output_row);
  }

  (any_changed, output_grid)
}


fn simulate_distancey_one(
    input_grid: &Vec<Vec<CellState>>,
    nearby_seats: &Vec<Vec<Vec<(usize, usize)>>>)
  -> (bool, Vec<Vec<CellState>>)
{
  let mut output_grid: Vec<Vec<CellState>> = Vec::new();
  let mut any_changed = false;

  for y in 0 .. input_grid.len() {
    let mut output_row: Vec<CellState> = Vec::new();

    for x in 0 .. input_grid[y].len() {
      let mut adjacent_count = 0;

      for (nearby_x, nearby_y) in &nearby_seats[y][x] {
        let nearby_cell = &input_grid[*nearby_y][*nearby_x];

        if *nearby_cell == CellState::FullSeat {
          adjacent_count += 1;
        }
      }

      let input_cell = &input_grid[y][x];
      if *input_cell == CellState::Floor {
        output_row.push(CellState::Floor);
      } else if *input_cell == CellState::EmptySeat && adjacent_count == 0 {
        output_row.push(CellState::FullSeat);
        any_changed = true;
      } else if *input_cell == CellState::FullSeat && adjacent_count >= 5 {
        output_row.push(CellState::EmptySeat);
        any_changed = true;
      } else {
        output_row.push(input_cell.clone());
      }
    }

    output_grid.push(output_row);
  }

  (any_changed, output_grid)
}


fn compute_nearby_seats(input_grid: &Vec<Vec<CellState>>)
  -> Vec<Vec<Vec<(usize, usize)>>>
{
  let mut nearby_seats: Vec<Vec<Vec<(usize, usize)>>> = Vec::new();

  let mut max_dimension: isize = isize::try_from(input_grid.len()).unwrap();
  if isize::try_from(input_grid[0].len()).unwrap() > max_dimension {
    max_dimension = isize::try_from(input_grid[0].len()).unwrap();
  }

  for y in 0 .. input_grid.len() {
    let mut nearby_seats_row: Vec<Vec<(usize, usize)>> = Vec::new();

    for x in 0 .. input_grid[y].len() {
      let mut nearby_seats_one: Vec<(usize, usize)> = Vec::new();

      for (dx, dy) in [
        (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1) ].iter()
      {
        for distance in 1 .. max_dimension {
          let nearby_y = isize::try_from(y).unwrap() + dy * distance;
          let nearby_x = isize::try_from(x).unwrap() + dx * distance;

          if nearby_y < 0 || usize::try_from(nearby_y).unwrap() >= input_grid.len() {
            break;
          }

          if nearby_x < 0 || usize::try_from(nearby_x).unwrap()
              >= input_grid[usize::try_from(nearby_y).unwrap()].len()
          {
            break;
          }

          let nearby_cell = &input_grid[usize::try_from(nearby_y).unwrap()]
            [usize::try_from(nearby_x).unwrap()];

          if *nearby_cell == CellState::FullSeat || *nearby_cell == CellState::EmptySeat {
            nearby_seats_one.push((usize::try_from(nearby_x).unwrap(),
                usize::try_from(nearby_y).unwrap()));
            break;
          }
        }
      }

      nearby_seats_row.push(nearby_seats_one);
    }

    nearby_seats.push(nearby_seats_row);
  }

  nearby_seats
}


fn count_full_seats(grid: &Vec<Vec<CellState>>) -> i64 {
  let mut count = 0;

  for row in grid {
    for cell in row {
      if *cell == CellState::FullSeat {
        count += 1;
      }
    }
  }

  count
}

