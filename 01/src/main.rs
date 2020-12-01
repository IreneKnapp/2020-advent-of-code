use advent_lib::prelude::*;


fn main() -> Result<()> {
  let mut args = std::env::args();
  if args.len() != 2 {
    eprintln!("Usage: advent input");
  }
  let _ = args.next();
  let filename = args.next().unwrap();

  let mut input = advent_lib::read_int_file(&filename)?;
  input.sort();

  for i in 0 .. input.len() {
    let a = input[i];
    if a > 2020 {
      break;
    }

    for j in i+1 .. input.len() {
      let b = input[j];

      if a + b == 2020 {
        let product = a * b;
        println!("a: {:?}, b: {:?}, a*b: {:?}", a, b, product);
      }
    }
  }

  for i in 0 .. input.len() {
    let a = input[i];
    if a > 2020 {
      break;
    }

    for j in i+1 .. input.len() {
      let b = input[j];

      if a + b > 2020 {
        break;
      }

      for k in j+1 .. input.len() {
        let c = input[k];

        if a + b + c == 2020 {
          let product = a * b * c;
          println!("a: {:?}, b: {:?}, c: {:?}, a*b*c: {:?}", a, b, c, product);
        }
      }
    }
  }

  Ok(())
}
