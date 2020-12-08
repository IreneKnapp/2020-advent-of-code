use advent_lib::prelude::*;

use std::convert::TryFrom;
use std::collections::BTreeSet;

#[derive(Clone, Debug)]
enum Operation {
  Accumulator(isize),
  Jump(isize),
  NoOp(isize),
}

#[derive(Clone, Debug)]
struct MachineState {
  accumulator: isize,
  program_counter: usize,
}

impl MachineState {
  fn new() -> MachineState {
    MachineState {
      accumulator: 0,
      program_counter: 0,
    }
  }
}


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let program = parse_program(&input)?;

  find_first_loop(&program);

  find_patch_to_terminate(&program);

  Ok(())
}


fn parse_program(input: &Vec<String>) -> Result<Vec<Operation>> {
  let mut program: Vec<Operation> = Vec::new();

  for line in input {
    let mut words = Vec::new();
    for word in line.split(' ') {
      words.push(word);
    }

    let operand = words[1].parse::<isize>()?;
    match words[0] {
      "acc" => program.push(Operation::Accumulator(operand)),
      "jmp" => program.push(Operation::Jump(operand)),
      "nop" => program.push(Operation::NoOp(operand)),
      _ => panic!()
    }
  }

  Ok(program)
}


fn simulate_one(program: &Vec<Operation>, state: &MachineState) -> MachineState {
  let mut new_state = state.clone();

  match program[new_state.program_counter] {
    Operation::Accumulator(operand) => {
      new_state.accumulator += operand;
      new_state.program_counter += 1;
    },
    Operation::Jump(operand) => {
      new_state.program_counter = usize::try_from(
        isize::try_from(new_state.program_counter).unwrap()
        + operand).unwrap();
    },
    Operation::NoOp(_) => {
      new_state.program_counter += 1;
    },
  }

  new_state
}


fn find_first_loop(program: &Vec<Operation>) -> () {
  let mut visited_lines: BTreeSet<usize> = BTreeSet::new();
  let mut state = MachineState::new();

  loop {
    if visited_lines.contains(&state.program_counter) {
      println!("{}", state.accumulator);
      break;
    }

    visited_lines.insert(state.program_counter);
    state = simulate_one(&program, &state);
  }
}


fn find_patch_to_terminate(program: &Vec<Operation>) -> () {
  for i in 0..program.len() {
    let mut patched_program = program.clone();

    match patched_program[i] {
      Operation::Accumulator(_) => {
        continue;
      },
      Operation::Jump(operand) => {
        patched_program[i] = Operation::NoOp(operand);
      },
      Operation::NoOp(operand) => {
        patched_program[i] = Operation::Jump(operand);
      },
    }

    let mut visited_lines: BTreeSet<usize> = BTreeSet::new();
    let mut state = MachineState::new();
    let mut normal_exit = false;

    loop {
      if visited_lines.contains(&state.program_counter) {
        break;
      }

      if state.program_counter == patched_program.len() {
        println!("normal exit {}", state.accumulator);
        normal_exit = true;
        break;
      }

      visited_lines.insert(state.program_counter);
      state = simulate_one(&patched_program, &state);
    }

    if normal_exit {
      break;
    }
  }
}

