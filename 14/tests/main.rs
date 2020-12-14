use assert_cmd::prelude::*;
use std::process::Command;


#[test]
fn personal_input() -> Result<(), Box<dyn std::error::Error>> {
  let mut command = Command::cargo_bin("advent_14")?;

  command.arg("input");
  command.assert().success().stdout("12408060320841\n4466434626828\n");

  Ok(())
}

