use nom::{
    IResult,
    branch::alt,
    combinator::opt,
    multi::{many1, many0},
    bytes::complete::{tag},
    character::complete::{alphanumeric1, digit1},
};

#[derive(Debug, Clone)]
pub enum Node {
    Program { children: Vec<Node> },
    Statement { children: Vec<Node> },
    WhileLoop { children: Vec<Node> },
    Condition { value : i32, children: Vec<Node>},
    PrintStatement { children: Vec<Node>},
    Expression { children: Vec<Node>},
    SingleMathExpression { name: String, children: Vec<Node>},
    Type { value: String},
    StringDefine { children: Vec<Node> },
    VariableDefine { children: Vec<Node> },
    NumberVariable { name: String, value: i32},
    Number { value : i32},
    Bool { value : bool },
    Identifier { value: String},
    String { value: String},
}

//identifier= letter, [{alphanumeric}] ;
pub fn identifier(input: &str) -> IResult<&str, Node>{
    let(input, result) = alphanumeric1(input)?;
    Ok((input, Node::Identifier{ value: result.to_string()}))
}

//string = "\"", {alphanumeric | " "}, "\""
pub fn string(input: &str) -> IResult<&str, Node>{
    let(input, _) = tag("\"")(input)?;
    let(input, result) = many1(alt((alphanumeric1, tag(" "))))(input)?;
    let(input, _) = tag("\"")(input)?;
    Ok((input, Node::String{ value: result.join("")}))
}

pub fn boolean(input: &str) -> IResult<&str, Node>{
    let mut temp = true;
    let(input, result) = alt((tag("true"),tag("false")))(input)?;
    if result == "true" {temp = true} else {temp = false};
    Ok((input, Node::Bool { value : temp}))
}

pub fn number(input: &str) -> IResult<&str, Node>{
    let(input, result) = digit1(input)?;
    let temp = result.parse::<i32>().unwrap();
    Ok((input, Node::Number{ value: temp}))
}

pub fn primitive_type(input: &str) -> IResult<&str, Node>{
    let(input, result) = alt((tag("boolean"),tag("int")))(input)?;
    Ok((input, Node::Type{ value: result.to_string()}))
}

pub fn value(input: &str) -> IResult<&str, Node>{
    let(input, result) = alt((identifier, number))(input)?;
    Ok((input, result))
}

//integer | identifier ;
pub fn expression(input: &str) -> IResult<&str, Node>{
    let(input, result) = alt((number, identifier))(input)?;
    Ok((input, Node::Expression { children: vec![result] }))
}
// identifier , ("++" | "+=" | "--" | "-="), [value] ;
pub fn single_math_expression(input: &str) -> IResult<&str, Node>{
    let mut sign = "";
    let(input, id) = identifier(input)?;
    let(input,eval) = alt((tag("++"),tag("--"),tag("+="),tag("-=")))(input)?;
    match eval {
        "++" => {
            sign = "+";
            let value = Node::Number{value : 1};
            Ok((input, Node::SingleMathExpression { name: sign.to_string(), children: vec![id, value] }))
        },
        "--" => {
            sign = "-";
            let value = Node::Number{value: 1};
            Ok((input, Node::SingleMathExpression { name : sign.to_string(), children: vec![id, value]}))
        },
        "+=" => {
            sign = "+";
            let(input, a) = expression(input)?;
            Ok((input, Node::SingleMathExpression { name : sign.to_string(), children: vec![id, a]}))
        },
        "-=" => {
            sign = "-";
            let(input, a) = expression(input)?;
            Ok((input, Node::SingleMathExpression { name : sign.to_string(), children: vec![id, a]}))
        }
        _ => panic!(),
    }
}

pub fn variable_define(input: &str) -> IResult<&str, Node>{
    let(input, ptype) = primitive_type(input)?;
    let(input,_) = many0(tag(" "))(input)?;
    let(input, name) = identifier(input)?;
    let(input,_) = many0(tag(" "))(input)?;
    let(input,_) = tag("=")(input)?;
    let(input,_) = many0(tag(" "))(input)?;
    let(input, expression) = expression(input)?;
    Ok((input, Node::VariableDefine { children: vec![ptype, name, expression] }))
}

pub fn string_define(input: &str) -> IResult<&str, Node>{
    let(input,_) = tag("String ")(input)?;
    let(input, name) = identifier(input)?;
    let(input,_) = many0(tag(" "))(input)?;
    let(input,_) = tag("=")(input)?;
    let(input,_) = many0(tag(" "))(input)?;
    let(input, contents) = string(input)?;
    Ok((input, Node::StringDefine{ children: vec![name, contents]}))
}

pub fn print_statement(input: &str) -> IResult<&str, Node>{
    let(input, _) = tag("System.out.println")(input)?;
    let(input,_) = tag("(")(input)?;
    let(input, result) = alt((expression,string))(input)?;
    let(input,_) = tag(")")(input)?;
    Ok((input, Node::PrintStatement { children: vec![result] }))
}

pub fn define(input: &str) -> IResult<&str, Node>{
    let(input, result) = alt((variable_define,string_define))(input)?;
    Ok((input, result))
}

pub fn statement(input: &str) -> IResult<&str, Node>{
    let(input, _) = many0(tag(" "))(input)?;
    let(input,result) = alt((print_statement, define, single_math_expression))(input)?;
    let(input,_) = tag(";")(input)?;
    let(input,_) = many0(tag(" "))(input)?;
    Ok((input,result))
}

pub fn condition(input: &str) -> IResult<&str, Node>{
    let(input,_) = many0(tag(" "))(input)?;
    let(input, fVal) = expression(input)?;
    let(input,_) = many0(tag(" "))(input)?;
    let(input, sign) = alt((tag(">"),tag(">="),tag("=="),tag("<"),tag("<=")))(input)?;
    let(input,_) = many0(tag(" "))(input)?;
    let(input, finVal) = expression(input)?;
    let mut temp = 0;
    match sign {
        ">" => temp = 1,
        "<" => temp = -1,
        "<=" | ">=" | "==" => temp = 0,
        _  => panic!(),
    }
    Ok((input, Node::Condition{ value: temp, children: vec![fVal, finVal]}))

}
pub fn while_loop(input: &str) -> IResult<&str, Node>{
    let(input,_) = tag("while")(input)?;
    let(input,_) = many0(tag(" "))(input)?;
    let(input,_) = tag("(")(input)?;
    let(input, mut condition) = condition(input)?;
    let(input,_) = tag(")")(input)?;
    let(input,_) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let(input, _) = tag("{")(input)?;
    let(input,_) = many0(alt((tag(" "),tag("\n"))))(input)?;
    let(input, mut statements) = many1(statement)(input)?;
    let(input, _) = tag("}")(input)?;
    let(input,_) = many0(alt((tag(" "),tag("\n"))))(input)?;
    //Want to read the statements and find the single_math_expression which affects the fVal in condition.
    //Run through the statements as many times as you need to until the condition is met
    //Return statements each time they were ran
    let mut children = vec![condition];
    children.append(&mut statements);
    Ok((input, Node::WhileLoop{ children: children}))
}

pub fn program(input: &str) -> IResult<&str, Node>{
    let(input, result) = many1(alt((statement,while_loop)))(input)?;
    Ok((input, Node::Program { children: result}))
}