use assert_cmd::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_10")?;

  command.arg("input");
  command.assert().success().stdout("3034\n259172170858496\n");

  Ok(())
}

