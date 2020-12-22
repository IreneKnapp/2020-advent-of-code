use assert_cmd::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_19")?;

  command.arg("input");
  command.assert().success().stdout("176\n352\n");

  Ok(())
}

