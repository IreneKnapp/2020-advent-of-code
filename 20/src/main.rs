use advent_lib::prelude::*;

use std::collections::BTreeMap;
use std::collections::BTreeSet;

#[derive(Clone,Copy,Debug,Eq,Ord,PartialOrd,PartialEq)]
pub enum Edge {
  Top,
  Bottom,
  Left,
  Right,
}

#[derive(Clone,Copy,Debug,Eq,Ord,PartialOrd,PartialEq)]
pub struct Orientation {
  edge_at_top: Edge, // rotation occurs *after* flipping
  is_flipped: bool, // flips occur around the y axis
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

  let mut tiles: BTreeMap<i64,Vec<Vec<bool>>> = BTreeMap::new();
  for tile_input in &input {
    let mut lines = tile_input.iter();

    let rest = lines.next().unwrap();
    let (_, rest) = rest.split_at(5);
    let colon_point = rest.find(':').unwrap();
    let (tile_id, _) = rest.split_at(colon_point);
    let tile_id = tile_id.parse::<i64>().unwrap();

    let mut tile = Vec::new();
    for line in lines {
      let mut row = Vec::new();
      for c in line.chars() {
        row.push(c == '#');
      }
      tile.push(row);
    }

    tiles.insert(tile_id, tile);
  }

  let mut tile_edges: BTreeMap<(i64,Edge),u64> = BTreeMap::new();
  for (tile_id, tile) in tiles.iter() {
    let top = extract_top(tile);
    let bottom = extract_bottom(tile);
    let left = extract_left(tile);
    let right = extract_right(tile);
    tile_edges.insert((*tile_id, Edge::Top), top);
    tile_edges.insert((*tile_id, Edge::Bottom), bottom);
    tile_edges.insert((*tile_id, Edge::Left), left);
    tile_edges.insert((*tile_id, Edge::Right), right);
  }

  let tile_count = tiles.keys().len() as f64;
  let grid_size = tile_count.sqrt().floor() as i64;

  let mut grid: Vec<Vec<Option<(i64,Orientation)>>> = Vec::new();
  for _ in 0 .. grid_size {
    let mut grid_row = Vec::new();
    for _ in 0 .. grid_size {
      grid_row.push(None);
    }
    grid.push(grid_row);
  }

  let mut tile_ids = BTreeSet::new();
  for tile_id in tiles.keys() {
    tile_ids.insert(*tile_id);
  }

  let grid = fill_grid(&grid, &tiles, &tile_edges, &tile_ids).unwrap();
  let summary = summarize_grid(&grid);
  println!("{}", summary);

  let image = assemble_grid(&grid, &tiles);
  let sea_serpent_count = count_sea_serpents(&image);
  println!("{}", sea_serpent_count);

  Ok(())
}


fn fill_grid(grid: &Vec<Vec<Option<(i64,Orientation)>>>,
             tiles: &BTreeMap<i64,Vec<Vec<bool>>>,
             tile_edges: &BTreeMap<(i64,Edge),u64>,
             available_tiles: &BTreeSet<i64>)
  -> Option<Vec<Vec<Option<(i64,Orientation)>>>>
{
  match find_empty_cell(grid) {
    Some((empty_x, empty_y)) => {
      for tile_id in available_tiles {
        let mut value_to_match_top = None;
        if empty_y > 0 {
          let (tile_id_above, orientation_above) =
              grid[empty_y - 1][empty_x].unwrap();
          let edge_above = edge_at(orientation_above, Edge::Bottom);
          let mut edge_value_above =
              *tile_edges.get(&(tile_id_above, edge_above)).unwrap();
          if !orientation_above.is_flipped {
            edge_value_above = reverse_edge(edge_value_above);
          }
          value_to_match_top = Some(edge_value_above);
        }

        let mut value_to_match_left = None;
        if empty_x > 0 {
          let (tile_id_left, orientation_left) =
              grid[empty_y][empty_x - 1].unwrap();
          let edge_left = edge_at(orientation_left, Edge::Right);
          let mut edge_value_left =
              *tile_edges.get(&(tile_id_left, edge_left)).unwrap();
          if !orientation_left.is_flipped {
            edge_value_left = reverse_edge(edge_value_left);
          }
          value_to_match_left = Some(edge_value_left);
        }

        for edge_at_top in &[Edge::Top, Edge::Right, Edge::Bottom, Edge::Left] {
          for is_flipped in &[false, true] {
            let orientation = Orientation {
              edge_at_top: *edge_at_top,
              is_flipped: *is_flipped,
            };

            match value_to_match_top {
              Some(to_match) => {
                let edge_top = edge_at(orientation, Edge::Top);
                let mut value_top =
                    *tile_edges.get(&(*tile_id, edge_top)).unwrap();
                if *is_flipped {
                  value_top = reverse_edge(value_top);
                }

                if value_top != to_match {
                  continue;
                }
              },
              None => { },
            }

            match value_to_match_left {
              Some(to_match) => {
                let edge_left = edge_at(orientation, Edge::Left);
                let mut value_left =
                    *tile_edges.get(&(*tile_id, edge_left)).unwrap();
                if *is_flipped {
                  value_left = reverse_edge(value_left);
                }

                if value_left != to_match {
                  continue;
                }
              },
              None => { },
            }

            let mut modified_grid = Vec::new();
            for y in 0 .. grid.len() {
              let mut modified_row = Vec::new();
              for x in 0 .. grid[y].len() {
                if y == empty_y && x == empty_x {
                  modified_row.push(Some((*tile_id, orientation)));
                } else {
                  modified_row.push(grid[y][x]);
                }
              }
              modified_grid.push(modified_row);
            }

            let mut modified_available_tiles = available_tiles.clone();
            modified_available_tiles.remove(tile_id);

            if modified_available_tiles.iter().count() == 0 {
              return Some(modified_grid);
            } else {
              let result = fill_grid(&modified_grid, tiles,
                                     tile_edges, &modified_available_tiles);
              if result.is_some() {
                return result;
              }
            }
          }
        }
      }

      None
    },
    None => {
      Some(grid.clone())
    },
  }
}


fn find_empty_cell(grid: &Vec<Vec<Option<(i64,Orientation)>>>)
  -> Option<(usize, usize)>
{
  let mut found_empty_cell = false;
  let mut empty_y = 0;
  let mut empty_x = 0;

  for y in 0 .. grid.len() {
    for x in 0 .. grid[y].len() {
      match grid[y][x] {
        None => {
          empty_x = x;
          empty_y = y;
          found_empty_cell = true;
          break;
        },
        _ => { },
      }
    }
    if found_empty_cell {
      break;
    }
  }

  if found_empty_cell {
    Some((empty_x, empty_y))
  } else {
    None
  }
}


fn extract_top(tile: &Vec<Vec<bool>>) -> u64 {
  let mut result = 0;
  let y = 0;
  for x in 0 .. tile[y].len() {
    result *= 2;
    if tile[y][x] {
      result += 1;
    }
  }
  result
}


fn extract_bottom(tile: &Vec<Vec<bool>>) -> u64 {
  let mut result = 0;
  let y = tile.len() - 1;
  let width = tile[y].len();
  for x_reversed in 0 .. width {
    let x = width - x_reversed - 1;
    result *= 2;
    if tile[y][x] {
      result += 1;
    }
  }
  result
}


fn extract_left(tile: &Vec<Vec<bool>>) -> u64 {
  let mut result = 0;
  let x = 0;
  let height = tile.len();
  for y_reversed in 0 .. height {
    let y = height - y_reversed - 1;
    result *= 2;
    if tile[y][x] {
      result += 1;
    }
  }
  result
}


fn extract_right(tile: &Vec<Vec<bool>>) -> u64 {
  let mut result = 0;
  let x = tile[0].len() - 1;
  for y in 0 .. tile.len() {
    result *= 2;
    if tile[y][x] {
      result += 1;
    }
  }
  result
}


fn reverse_edge(input: u64) -> u64 {
  let mut input = input;
  let mut result = 0;
  for _ in 0 .. 10 {
    result *= 2;
    if input & 1 != 0 {
      result += 1;
    }
    input /= 2;
  }
  result
}


fn edge_clockwise(edge: Edge) -> Edge {
  match edge {
    Edge::Top => Edge::Right,
    Edge::Right => Edge::Bottom,
    Edge::Bottom => Edge::Left,
    Edge::Left => Edge::Top,
  }
}


fn edge_counterclockwise(edge: Edge) -> Edge {
  match edge {
    Edge::Top => Edge::Left,
    Edge::Left => Edge::Bottom,
    Edge::Bottom => Edge::Right,
    Edge::Right => Edge::Top,
  }
}


fn edge_at(orientation: Orientation, edge_as_placed: Edge) -> Edge {
  if !orientation.is_flipped {
    let n_clockwise_from_top = match edge_as_placed {
      Edge::Top => 0,
      Edge::Right => 1,
      Edge::Bottom => 2,
      Edge::Left => 3,
    };

    let mut rotated_edge = orientation.edge_at_top;
    for _ in 0 .. n_clockwise_from_top {
      rotated_edge = edge_clockwise(rotated_edge);
    }

    rotated_edge
  } else {
    let n_counterclockwise_from_top = match edge_as_placed {
      Edge::Top => 0,
      Edge::Right => 1,
      Edge::Bottom => 2,
      Edge::Left => 3,
    };

    let mut rotated_edge = orientation.edge_at_top;
    for _ in 0 .. n_counterclockwise_from_top {
      rotated_edge = edge_counterclockwise(rotated_edge);
    }

    rotated_edge
  }
}


fn summarize_grid(grid: &Vec<Vec<Option<(i64,Orientation)>>>) -> i64 {
  let mut summary = 1;
  let height = grid.len();
  let width = grid[0].len();
  summary *= match grid[0][0] {
    Some((tile_id, _)) => { tile_id },
    _ => 0,
  };
  summary *= match grid[height - 1][0] {
    Some((tile_id, _)) => { tile_id },
    _ => 0,
  };
  summary *= match grid[0][width - 1] {
    Some((tile_id, _)) => { tile_id },
    _ => 0,
  };
  summary *= match grid[height - 1][width - 1] {
    Some((tile_id, _)) => { tile_id },
    _ => 0,
  };
  summary
}


fn assemble_grid(grid: &Vec<Vec<Option<(i64,Orientation)>>>,
                 tiles: &BTreeMap<i64,Vec<Vec<bool>>>)
  -> Vec<Vec<bool>>
{
  let mut image: Vec<Vec<bool>> = Vec::new();

  let grid_size = grid.len();
  let (sample_tile_id, _) = grid[0][0].unwrap();
  let sample_tile = tiles.get(&sample_tile_id).unwrap();
  let tile_size = sample_tile.len();

  for outer_y in 0 .. grid_size {
    for inner_y_shifted in 0 .. tile_size - 2 {
      let mut image_row = Vec::new();

      let inner_y = inner_y_shifted + 1;

      for outer_x in 0 .. grid_size {
        let (tile_id, orientation) = grid[outer_y][outer_x].unwrap();
        let tile = tiles.get(&tile_id).unwrap();

        for inner_x_shifted in 0 .. tile_size - 2 {
          let inner_x = inner_x_shifted + 1;
          let (tile_x, tile_y) = transform_placed_coordinates_to_tile(
              orientation, tile_size, inner_x, inner_y);
          let cell = tile[tile_y][tile_x];
          image_row.push(cell);
        }
      }

      image.push(image_row);
    }
  }

  image
}


pub fn debug_grid(grid: &Vec<Vec<Option<(i64,Orientation)>>>,
              tiles: &BTreeMap<i64,Vec<Vec<bool>>>)
{
  let mut debug_output = String::new();

  let grid_size = grid.len();
  let (sample_tile_id, _) = grid[0][0].unwrap();
  let sample_tile = tiles.get(&sample_tile_id).unwrap();
  let tile_size = sample_tile.len();

  for outer_y in 0 .. grid_size {
    if outer_y > 0 {
      debug_output.push_str("\n");
    }

    for inner_y in 0 .. tile_size {
      for outer_x in 0 .. grid_size {
        debug_output.push_str("  ");

        let (tile_id, orientation) = grid[outer_y][outer_x].unwrap();
        let tile = tiles.get(&tile_id).unwrap();

        for inner_x in 0 .. tile_size {
          let (tile_x, tile_y) = transform_placed_coordinates_to_tile(
              orientation, tile_size, inner_x, inner_y);
          let cell = tile[tile_y][tile_x];

          if cell {
            debug_output.push_str("#");
          } else {
            debug_output.push_str(".");
          }
        }
      }

      debug_output.push_str("\n");
    }
  }

  println!("{}", debug_output);
}


pub fn debug_image(image: &Vec<Vec<bool>>) {
  let mut output = String::new();

  for row in image {
    for cell in row {
      if *cell {
        output.push_str("#");
      } else {
        output.push_str(".");
      }
    }
    output.push_str("\n");
  }
  println!("{}", output);
}


pub fn debug_map(image: &Vec<Vec<bool>>, map: &Vec<Vec<bool>>) {
  let mut output = String::new();
  let image_size = image.len();

  for y in 0 .. image_size {
    for x in 0 .. image_size {
      if map[y][x] {
        if image[y][x] {
          output.push_str("O");
        } else {
          output.push_str("@");
        }
      } else {
        if image[y][x] {
          output.push_str("#");
        } else {
          output.push_str(".");
        }
      }
    }
    output.push_str("\n");
  }
  println!("{}", output);
}


fn transform_placed_coordinates_to_tile(
    orientation: Orientation, tile_size: usize, placed_x: usize, placed_y: usize)
  -> (usize, usize)
{
  /*
   * at top    TOP     RIGHT      BOTTOM     LEFT
   *
   * normal    A.      .2         2+         +A
   *           +2      A+         .A         2.
   *
   * flipped   .A      2.         +2         A+
   *           2+      +A         A.         .2
   */

  if !orientation.is_flipped {
    match orientation.edge_at_top {
      Edge::Top => { (placed_x, placed_y) },
      Edge::Left => { (placed_y, tile_size - placed_x - 1) },
      Edge::Bottom => { (tile_size - placed_x - 1, tile_size - placed_y - 1) },
      Edge::Right => { (tile_size - placed_y - 1, placed_x) },
    }
  } else {
    match orientation.edge_at_top {
      Edge::Top => { (tile_size - placed_x - 1, placed_y) },
      Edge::Left => { (placed_y, placed_x) },
      Edge::Bottom => { (placed_x, tile_size - placed_y - 1) },
      Edge::Right => { (tile_size - placed_y - 1, tile_size - placed_x - 1) },
    }
  }
}


fn transform_image(input_image: &Vec<Vec<bool>>, orientation: Orientation)
  -> Vec<Vec<bool>>
{
  let image_size = input_image.len();

  let mut output_image = Vec::new();
  for y_as_placed in 0 .. image_size {
    let mut output_row = Vec::new();
    for x_as_placed in 0 .. image_size {
      let (x_in_input, y_in_input) = transform_placed_coordinates_to_tile(
        orientation, image_size, x_as_placed, y_as_placed);
      if input_image[y_in_input][x_in_input] {
        output_row.push(true);
      } else {
        output_row.push(false);
      }
    }
    output_image.push(output_row);
  }

  output_image
}


fn count_sea_serpents(image: &Vec<Vec<bool>>) -> usize {
  let mut sea_serpent: Vec<Vec<bool>> = Vec::new();
  for line in &["                  # ",
                "#    ##    ##    ###",
                " #  #  #  #  #  #   "]
  {
    let mut sea_serpent_row = Vec::new();
    for c in line.chars() {
      sea_serpent_row.push(c == '#');
    }
    sea_serpent.push(sea_serpent_row);
  }
  let sea_serpent_height = sea_serpent.len();
  let sea_serpent_width = sea_serpent[0].len();
  let image_size = image.len();

  let mut sea_serpent_map: Vec<Vec<bool>> = Vec::new();
  for _ in 0 .. image_size {
    let mut map_row = Vec::new();
    for _ in 0 .. image_size {
      map_row.push(false);
    }
    sea_serpent_map.push(map_row);
  }

  for edge_at_top in &[Edge::Top, Edge::Right, Edge::Bottom, Edge::Left] {
    for is_flipped in &[false, true] {
      let orientation = Orientation {
        edge_at_top: *edge_at_top,
        is_flipped: *is_flipped,
      };

      let transformed_image = transform_image(image, orientation);
      let mut sea_serpent_count = 0;

      for image_y in 0 .. image_size - sea_serpent_height + 1 {
        for image_x in 0 .. image_size - sea_serpent_width + 1 {
          let mut possible_sea_serpent = true;

          for sea_serpent_y in 0 .. sea_serpent_height {
            for sea_serpent_x in 0 .. sea_serpent_width {
              if sea_serpent[sea_serpent_y][sea_serpent_x] {
                let cell_x = image_x + sea_serpent_x;
                let cell_y = image_y + sea_serpent_y;
                let cell = transformed_image[cell_y][cell_x];
                if !cell {
                  possible_sea_serpent = false;
                  break;
                }
              }
            }
            if !possible_sea_serpent {
              break;
            }
          }

          if possible_sea_serpent {
            sea_serpent_count += 1;

            for sea_serpent_y in 0 .. sea_serpent_height {
              for sea_serpent_x in 0 .. sea_serpent_width {
                if sea_serpent[sea_serpent_y][sea_serpent_x] {
                  let map_x = image_x + sea_serpent_x;
                  let map_y = image_y + sea_serpent_y;
                  sea_serpent_map[map_y][map_x] = true;
                }
              }
            }
          }
        }
      }

      if sea_serpent_count > 0 {
        let mut roughness = 0;
        for y in 0 .. image_size {
          for x in 0 .. image_size {
            if transformed_image[y][x] && !sea_serpent_map[y][x] {
              roughness += 1;
            }
          }
        }

        return roughness;
      }
    }
  }

  0
}

