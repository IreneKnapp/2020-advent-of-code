use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut trees: Vec<Vec<bool>> = Vec::new();

  for line in &input {
    let mut tree_line: Vec<bool> = Vec::new();

    for c in line.trim().chars() {
      if c == '#' {
        tree_line.push(true);
      } else {
        tree_line.push(false);
      }
    }

    trees.push(tree_line);
  }

  let mut product = 1;
  product *= check_slope(&trees, 1, 1);
  product *= check_slope(&trees, 3, 1);
  product *= check_slope(&trees, 5, 1);
  product *= check_slope(&trees, 7, 1);
  product *= check_slope(&trees, 1, 2);

  println!("product: {}", product);

  Ok(())
}


fn check_slope(trees: &Vec<Vec<bool>>, dx: usize, dy: usize) -> usize {
  let mut x = 0;
  let mut y = 0;
  let mut trees_hit = 0;
  loop {
    let x_adjusted = x % trees[y].len();
    let is_tree = trees[y][x_adjusted];

    if is_tree {
      trees_hit += 1;
    }

    x += dx;
    y += dy;

    if y >= trees.len() {
      break;
    }
  }

  println!("dx: {}, dy: {}, trees: {}", dx, dy, trees_hit);

  trees_hit
}

