use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_03")?;

  command.arg("input");
  command.assert().success().stdout(
    "dx: 1, dy: 1, trees: 60\n\
     dx: 3, dy: 1, trees: 225\n\
     dx: 5, dy: 1, trees: 57\n\
     dx: 7, dy: 1, trees: 58\n\
     dx: 1, dy: 2, trees: 25\n\
     product: 1115775000\n");

  Ok(())
}

