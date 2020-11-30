pub mod error;
pub mod prelude;

pub use crate::prelude::Result;


pub fn greeting() -> Result<()> {
  println!("Hello, Irenes!");

  Ok(())
}
