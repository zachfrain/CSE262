extern crate nom;
extern crate asalang_parser;

use asalang_parser::{program, Node};

fn main() -> Result<(), nom::Err<(&'static str, nom::error::ErrorKind)>> {
  let result = program(r#"fn main(){return foo(1,2,3);} fn foo(a,b,c){return a+b;}"#);
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