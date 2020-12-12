use advent_lib::prelude::*;

//use std::convert::TryFrom;

#[derive(Debug)]
struct ShipState {
  x_offset: i64, // positive is east
  y_offset: i64, // positive is north
  heading: i64, // 0 is east; 90 is north
}

#[derive(Debug)]
struct NavState {
  waypoint_x_offset: i64,
  waypoint_y_offset: i64,
  ship_x_offset: i64,
  ship_y_offset: i64,
}


impl ShipState {
  fn new() -> ShipState {
    ShipState {
      x_offset: 0,
      y_offset: 0,
      heading: 0,
    }
  }
}

impl NavState {
  fn new() -> NavState {
    NavState {
      waypoint_x_offset: 10,
      waypoint_y_offset: 1,
      ship_x_offset: 0,
      ship_y_offset: 0,
    }
  }
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let mut ship = ShipState::new();
  let mut nav = NavState::new();

  for line in &input {
    let (command, parameter_str) = line.split_at(1);
    let parameter = parameter_str.parse::<i64>()?;

    match command {
      "N" => {
        ship.y_offset += parameter;
        nav.waypoint_y_offset += parameter;
      },
      "S" => {
        ship.y_offset -= parameter;
        nav.waypoint_y_offset -= parameter;
      },
      "E" => {
        ship.x_offset += parameter;
        nav.waypoint_x_offset += parameter;
      },
      "W" => {
        ship.x_offset -= parameter;
        nav.waypoint_x_offset -= parameter;
      },
      "L" => {
        ship.heading = (ship.heading + parameter) % 360;

        rotate_waypoint(&mut nav, parameter);
      },
      "R" => {
        ship.heading = (ship.heading - parameter + 360) % 360;

        rotate_waypoint(&mut nav, -parameter);
      },
      "F" => {
        while ship.heading < 0 {
          ship.heading += 360;
        }
        ship.heading = ship.heading % 360;

        if ship.heading == 0 {
          ship.x_offset += parameter;
        } else if ship.heading == 90 {
          ship.y_offset += parameter;
        } else if ship.heading == 180 {
          ship.x_offset -= parameter;
        } else if ship.heading == 270 {
          ship.y_offset -= parameter;
        }

        nav.ship_x_offset += parameter * nav.waypoint_x_offset;
        nav.ship_y_offset += parameter * nav.waypoint_y_offset;
      },
      _ => { },
    }
  }

  let ship_distance = ship.x_offset.abs() + ship.y_offset.abs();
  let nav_distance = nav.ship_x_offset.abs() + nav.ship_y_offset.abs();
  println!("{}", ship_distance);
  println!("{}", nav_distance);

  Ok(())
}


fn rotate_waypoint(nav: &mut NavState, parameter: i64) {
  let mut rotation = parameter;
  while rotation < 0 {
    rotation += 360;
  }
  rotation = rotation % 360;

  if rotation == 90 {
    // counterclockwise
    let x_tmp = nav.waypoint_x_offset;
    let y_tmp = nav.waypoint_y_offset;
    nav.waypoint_y_offset = x_tmp;
    nav.waypoint_x_offset = -y_tmp;
  } else if rotation == 270 {
    // clockwise
    let x_tmp = nav.waypoint_x_offset;
    let y_tmp = nav.waypoint_y_offset;
    nav.waypoint_y_offset = -x_tmp;
    nav.waypoint_x_offset = y_tmp;
  } else if rotation == 180 {
    let x_tmp = nav.waypoint_x_offset;
    let y_tmp = nav.waypoint_y_offset;
    nav.waypoint_x_offset = -x_tmp;
    nav.waypoint_y_offset = -y_tmp;
  }
}
