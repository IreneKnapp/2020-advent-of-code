use assert_cmd::prelude::*;
//use predicates::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_05")?;

  command.arg("input");
  command.assert().success().stdout("996\n671\n");

  Ok(())
}

