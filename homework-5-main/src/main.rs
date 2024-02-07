extern crate nom;
extern crate asalang;

use asalang::{program, run, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  
  let result = program(r#"fn main(){ return 1 + 1; } "#);
  match result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:#?}", tree);
      let result = run(&tree);
      println!("{:?}", result);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }

    
  Ok(())
}
