use advent_lib::prelude::*;

//use std::convert::TryFrom;

#[macro_use] extern crate lalrpop_util;

lalrpop_mod!(pub expression);



fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let input = advent_lib::read_lines_file(&filename)?;

  let parser = expression::ExpressionParser::new();
  let parser_pro = expression::ExpressionProParser::new();

  let mut sum = 0;
  let mut sum_pro = 0;

  for line in &input {
    let value = parser.parse(line)?;
    sum += value;

    let value_pro = parser_pro.parse(line)?;
    sum_pro += value_pro;
  }

  println!("{}", sum);
  println!("{}", sum_pro);

  Ok(())
}
