extern crate nom;
extern crate midterm;

use midterm::{program, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  let result = program(r#"int i = 15; while(i>20){ System.out.println("h"); i/=3;}"#);
  match result {
    Ok((unparsed,tree)) => {
      println!("Unparsed Text: {:?}", unparsed);
      println!("Parse Tree:\n {:?}", tree);
    }
    Err(error) => {
      println!("ERROR {:?}", error);
    }
  }
    
  Ok(())
}