use assert_cmd::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_16")?;

  command.arg("input");
  command.assert().success().stdout("25788\n3902565915559\n");

  Ok(())
}

