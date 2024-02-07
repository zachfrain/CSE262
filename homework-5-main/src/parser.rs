// Here is where the various combinators are imported. You can find all the combinators here:
// If you want to use it in your parser, you need to import it here. I've already imported a couple.

use nom::{
    IResult,
    branch::alt,
    multi::{many1, many0},
    combinator::opt,
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
    MathAdd {children: Vec<Node> },
    FunctionCall { name: String, children: Vec<Node> },
    VariableDefine { children: Vec<Node> },
    Number { value: i32 },
    Bool { value: bool },
    Identifier { value: String },
    String { value: String },
    Null,
  }
  
  // Here is the grammar, for your reference:
  
  pub fn function_definition(input: &str) -> IResult<&str, Node> {
    /*  let (input, _) = tag("fn ")(input)?;
     let (input, function_name) = identifier(input)?;   
     let (input, _) = tag("(")(input)?;
     let (input, mut args) = many0(arguments)(input)?;
     let (input, _) = tag(")")(input)?;
     let (input, _) = many0(alt((space1, line_ending)))(input)?;
     let (input, _) = tag("{")(input)?;
     let (input, _) = many0(alt((space1, line_ending)))(input)?;
     let (input, mut statements) = many1(alt((statement,expression)))(input)?;
     let (input, _) = tag("}")(input)?;
     let (input, _) = many0(alt((space1, line_ending)))(input)?;
     let mut children = vec![function_name];
     children.append(&mut args);
     children.append(&mut statements);
     Ok((input, Node::FunctionDefine{ children: children }))   */ 
     let(input, _) = many0(alt((space1, line_ending)))(input)?;
     let(input, _) = tag("fn ")(input)?;
     let(input, _) = many0(alt((space1, line_ending)))(input)?;
     let(input, function_name) = identifier(input)?;
     let(input, _) = many0(alt((space1, line_ending)))(input)?;
     let(input,_) = tag("(")(input)?;
     let(input, mut args) = many0(arguments)(input)?;
     let(input, _) = tag(")")(input)?;
     let (input, _) = many0(alt((space1, line_ending)))(input)?;
     let (input, _) = tag("{")(input)?;
     let (input, _) = many0(alt((space1, line_ending)))(input)?;
     let (input, mut statements) = many1(statement)(input)?;
     let(input, _) = many0(alt((space1, line_ending)))(input)?;
     let (input, _) = tag("}")(input)?;
     let (input, _) = many0(alt((space1, line_ending)))(input)?;
     let mut children = vec![function_name];
     children.append(&mut args);
     children.append(&mut statements);
     Ok((input, Node::FunctionDefine{ children: children }))
     
   }

  pub fn identifier(input: &str) -> IResult<&str, Node> {
    let (input, result) = alphanumeric1(input)?;              // Consume at least 1 alphanumeric character. The ? automatically unwraps the result if it's okay and bails if it is an error.
    Ok((input, Node::Identifier{ value: result.to_string()})) // Return the now partially consumed input, as well as a node with the string on it.
  }
  
  pub fn number(input: &str) -> IResult<&str, Node> {
    let (input, result) = digit1(input)?;                     // Consume at least 1 digit 0-9
    let number = result.parse::<i32>().unwrap();              // Parse the string result into a usize
    Ok((input, Node::Number{ value: number}))                 // Return the now partially consumed input with a number as well
  }
  
  pub fn boolean(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((tag("true"),tag("false")))(input)?;
    let bool_value = if result == "true" {true} else {false};
    Ok((input, Node::Bool{ value: bool_value}))
  }
  
  pub fn string(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("\"")(input)?;
    let (input, string) = many1(alt((alphanumeric1,tag(" "))))(input)?;
    let (input, _) = tag("\"")(input)?;
    Ok((input, Node::String{ value: string.join("")}))
  }
  
  pub fn function_call(input: &str) -> IResult<&str, Node> {
    let (input, name) = identifier(input)?;
    let call_name = match name {
      Node::Identifier{value} => value,
      _ => String::from(""),
    };
    let(input, _) = many0(alt((space1, line_ending)))(input)?;
    let(input, _) = tag("(")(input)?;
    let(input, args) = many0(arguments)(input)?;
    let(input, _) = tag(")")(input)?;
    let(input, _) = many0(alt((space1, line_ending)))(input)?;
    Ok((input, Node::FunctionCall{name: call_name, children: args}))   
  } 

//l4 -> parentheses
pub fn l4_infix(input: &str) -> IResult<&str, Node>{
  let(input, _) = many0(alt((space1, line_ending)))(input)?;
  let(input, _) = tag("(")(input)?;
  let(input, _) = many0(alt((space1, line_ending)))(input)?;
  let(input, expr) = expression(input)?;
  let(input, _) = many0(alt((space1, line_ending)))(input)?;
  let(input, _) = tag(")")(input)?;
  Ok((input, expr))
}
pub fn l4(input: &str) -> IResult<&str, Node>{
  alt((l4_infix, number, identifier, function_call))(input)
}
//l3 -> exponent
pub fn l3_infix(input: &str) -> IResult<&str, Node>{
  let(input, _) = many0(alt((space1, line_ending)))(input)?;
  let(input, op) = tag("^")(input)?;
  let(input, _) = many0(alt((space1, line_ending)))(input)?;
  let(input, args) = l4(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
pub fn l3(input: &str) -> IResult<&str, Node>{
  let(input, mut head) = l4(input)?;
  let(input, tail) = many0(l3_infix)(input)?;
  for n in tail{
    match n{
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => ()
    };
  }
  Ok((input,head))
}
//l2 -> multiplication & division
pub fn l2_infix(input: &str) -> IResult<&str, Node>{
  let(input, _) = many0(alt((space1, line_ending)))(input)?;
  let(input, op) = alt((tag("*"),tag("/")))(input)?;
  let(input, _) = many0(alt((space1, line_ending)))(input)?;
  let(input, args) = l3(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
pub fn l2(input: &str) -> IResult<&str, Node>{
  let(input, mut head) = l3(input)?;
  let(input, tail) = many0(l2_infix)(input)?;
  for n in tail{
    match n{
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => ()
    };
  }
  Ok((input,head))
}
//l1 -> addition & subtraction
pub fn l1_infix(input: &str) -> IResult<&str, Node>{
  let(input, _) = many0(alt((space1, line_ending)))(input)?;
  let(input, op) = alt((tag("+"),tag("-")))(input)?;
  let(input, _) = many0(alt((space1, line_ending)))(input)?;
  let(input, args) = l2(input)?;
  Ok((input, Node::MathExpression{name: op.to_string(), children: vec![args]}))
}
pub fn l1(input: &str) -> IResult<&str, Node>{
  let(input, mut head) = l2(input)?;
  let(input, tail) = many0(l1_infix)(input)?;
  for n in tail {
    match n {
      Node::MathExpression{name, mut children} => {
        let mut new_children = vec![head.clone()];
        new_children.append(&mut children);
        head = Node::MathExpression{name, children: new_children};
      }
      _ => ()
    };
  }
  Ok((input,head))
}

  // math_expression = value , { ("+" | "-") , value } ;
  pub fn math_expression(input: &str) -> IResult<&str, Node> {
    let(input, _) = many0(alt((space1, line_ending)))(input)?;
    l1(input)
  }
  pub fn statement(input: &str) -> IResult<&str, Node> {
    let (input, _) = many0(alt((space1, line_ending)))(input)?;
    let (input, result) = alt((function_return,variable_define))(input)?; 
    let (input, _) = many0(alt((space1, line_ending)))(input)?;
    Ok((input, Node::Statement{children: vec![result]}))
  }

  

  pub fn expression(input: &str) -> IResult<&str, Node> {
    let (input, result) = alt((function_call, boolean,  math_expression, number, string, identifier))(input)?;
    Ok((input, Node::Expression{ children: vec![result]}))   
  }
  
  pub fn function_return(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("return ")(input)?;
    let (input, return_value) = alt((function_call, expression, identifier))(input)?;
    Ok((input, Node::FunctionReturn{ children: vec![return_value]}))
  }
  
  pub fn variable_define(input: &str) -> IResult<&str, Node> {
    let (input, _) = tag("let ")(input)?;
    let (input, variable) = identifier(input)?;
    let (input, _) = many0(alt((space1, line_ending)))(input)?;
    let (input, _) = tag("=")(input)?;
    let (input, _) = many0(alt((space1, line_ending)))(input)?;
    let (input, expression) = expression(input)?;
    let(input, _) = tag(";")(input)?;
    Ok((input, Node::VariableDefine{ children: vec![variable, expression]}))   
  }
  
  pub fn arguments(input: &str) -> IResult<&str, Node> {
    let(input, _) = many0(alt((space1, line_ending)))(input)?;
    let (input, arg) = expression(input)?;
    let(input, _) = many0(alt((space1, line_ending)))(input)?;
    let (input, mut others) = many0(other_arg)(input)?;
    let mut args = vec![arg];
    args.append(&mut others);
    Ok((input, Node::FunctionArguments{children: args}))
  }
  
  pub fn other_arg(input: &str) -> IResult<&str, Node> {
    let(input, _) = many0(alt((space1, line_ending)))(input)?;
    let (input, _) = tag(",")(input)?;
    let(input, _) = many0(alt((space1, line_ending)))(input)?;
    expression(input)
  }
  
  
  // program = function_definition+ ;
  pub fn program(input: &str) -> IResult<&str, Node> {
    let(input, result) = many1(alt((function_definition, statement, expression)))(input)?;
    Ok((input, Node::Program{ children: result}))
  }
  