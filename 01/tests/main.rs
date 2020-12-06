use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_01")?;

  command.arg("input");
  command.assert().success().stdout(
      "a: 246, b: 1774, a*b: 436404\n\
      a: 448, b: 721, c: 851, a*b*c: 274879808\n");

  Ok(())
}

