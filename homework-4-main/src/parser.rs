// Here is where the various combinators are imported. You can find all the combinators here:
// https://docs.rs/nom/7.1.3/nom/
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
  IResult,
  branch::alt,
  combinator::opt,
  multi::{many1, many0},
  bytes::complete::{tag},
  character::complete::{alphanumeric1, digit1, space1, line_ending},
};

// Here are the different node types. You will use these to make your parser and your grammar.
// You may add other nodes as you see fit, but these are expected by the runtime.

#[derive(Debug, Clone)]
pub enum Node {
  Program { children: Vec<Node> },
  Statement { children: Vec<Node> },
  FunctionReturn { children: Vec<Node> },
  FunctionDefine { children: Vec<Node> },
  FunctionArguments { children: Vec<Node> },
  FunctionStatements { children: Vec<Node> },
  Expression { children: Vec<Node> },
  MathExpression {name: String, children: Vec<Node> },
  FunctionCall { name: String, children: Vec<Node> },
  VariableDefine { children: Vec<Node> },
  Number { value: i32 },
  Bool { value: bool },
  Identifier { value: String },
  String { value: String },
}

// Here is the grammar, for your reference:

//identifier = alnum , {alnum} ;
pub fn identifier(input: &str) -> IResult<&str, Node> {
  let(input, result) = alphanumeric1(input)?;
  Ok((input, Node::Identifier { value: result.to_string() }))
}

// number = digit+ ;
pub fn number(input: &str) -> IResult<&str, Node> {
  let(input, result) = digit1(input)?;
  Ok((input, Node::Number { value: result.parse::<i32>().unwrap()}))
}

// boolean = "true" | "false" ;
pub fn boolean(input: &str) -> IResult<&str, Node> {
  let(input, result) = alt((tag("true"),tag("false")))(input)?;
  let temp = result.eq("true");
  Ok((input, Node::Bool { value: temp }))
}

// string = "\"" , {alnum | " "} , "\"" ;
pub fn string(input: &str) -> IResult<&str, Node> {
  let(input,_) = many0(alt((space1, line_ending)))(input)?;
  let(input,_) = tag("\"")(input)?;
  let(input, result) = many0(alt((alphanumeric1, space1)))(input)?;
  let(input,_) = tag("\"")(input)?;
  let(input,_) = many0(alt((space1,line_ending)))(input)?;
  Ok((input, Node::String { value: result.join("") }))
}

// function_call = identifier , "(" , [arguments] , ")" ;
pub fn function_call(input: &str) -> IResult<&str, Node> {
  let(input, temp) = identifier(input)?;
  let call_name = match temp {
      Node::Identifier{ value } => value,
      _ => panic!(),
   };
  let(input,_) = many0(alt((space1, line_ending)))(input)?;
  let(input,_) = tag("(")(input)?;
  let(input, children) = arguments(input)?;
  let(input, _) = tag(")")(input)?;
  let(input,_) = many0(alt((space1, line_ending)))(input)?;
  Ok((input, Node::FunctionCall { name: call_name, children: vec![children]}))
}

// value = bool | number | identifier ;
pub fn value(input: &str) -> IResult<&str, Node> {
  let(input, result) = alt((number, identifier, boolean))(input)?;
  Ok((input, result))
}

// math_expression = value , { ("+" | "-") , value } ;
pub fn math_expression(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// expression = boolean | math_expression | function_call | number | string | identifier ;
pub fn expression(input: &str) -> IResult<&str, Node> {
  let(input, result) = many0(alt((boolean, math_expression, function_call, number, identifier)))(input)?;
  Ok((input, Node::Expression { children: result }))
}

// statement = variable_define , ";" | function_return , ";" ;
pub fn statement(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// function_return = "return" , (function_call | expression | identifier) ;
pub fn function_return(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// variable_define = "let" , identifier , "=" , expression ;
pub fn variable_define(input: &str) -> IResult<&str, Node> {
  unimplemented!();  
}

// arguments = expression , { "," , expression } ;
pub fn arguments(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// Like the first argument but with a comma in front
pub fn other_arg(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// function_definition = "fn" , identifier , "(" , [arguments] , ")" , "{" , [statement+] , "}" ;
pub fn function_definition(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}

// comment = "//" , (?any-character? - newline);
pub fn comment(input: &str) -> IResult<&str, Node> {
  let(input,_) = tag("//")(input)?;
  let(input, comment) = many0(alt((alphanumeric1, space1)))(input)?;
  Ok((input, Node::String{ value: comment.join("")}))
}

// program = function_definition+ ;
pub fn program(input: &str) -> IResult<&str, Node> {
  unimplemented!();
}
