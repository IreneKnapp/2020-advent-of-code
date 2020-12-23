use assert_cmd::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_23")?;

  command.arg("input");
  command.assert().success().stdout("24798635\n12757828710\n");

  Ok(())
}

