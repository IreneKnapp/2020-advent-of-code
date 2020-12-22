use assert_cmd::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_18")?;

  command.arg("input");
  command.assert().success().stdout("510009915468\n321176691637769\n");

  Ok(())
}

