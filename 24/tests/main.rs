use assert_cmd::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_24")?;

  command.arg("input");
  command.assert().success().stdout("230\n3565\n");

  Ok(())
}

