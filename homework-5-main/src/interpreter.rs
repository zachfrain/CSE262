use crate::parser::Node;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
  String(String),
  Number(i32),
  Bool(bool),
}


struct Runtime {
  functions: HashMap<String, Vec<Node>>,
  stack: Vec<HashMap<String, Value>>,
}

impl Runtime {

  pub fn new() -> Runtime {
    Runtime {
      functions: HashMap::new(),
      stack: Vec::new(),
    }
  }

  pub fn run(&mut self, node: &Node) -> Result<Value, &'static str> {
    match node {
      Node::Program{children} => {
        for n in children {
          match n {
            Node::FunctionDefine{..} => {
              self.run(n);
            },
            Node::Expression{..} => {
              self.functions.insert("main".to_string(), vec![Node::FunctionReturn{children: vec![n.clone()]}]);
            },
            Node::Statement{..} => {
              self.functions.insert("main".to_string(), vec![n.clone()]);
            }
            _ => (),
          }
        }
        Ok(Value::Bool(true)) 
      },
      // Evaluates a mathematical expression based on the elements in the children argument. If the expression is valid, the code evaluates it and returns a new Value object with the resulting value. If the expression is not valid, the code returns an error message.
      Node::MathExpression{name, children} => {
        match(self.run(&children[0]), self.run(&children[1])){
          (Ok(Value::Number(left)), Ok(Value::Number(right))) => {
            match name.as_ref() {
              "+" => Ok(Value::Number(left + right)),
              "-" => Ok(Value::Number(left - right)),
              "*" => Ok(Value::Number(left * right)),
              "/" => Ok(Value::Number(left / right)),
              "^" => {
                let mut temp = 1;
                for _i in 0..right{
                  temp = temp * left;
                }
                Ok(Value::Number(temp))
              },
              _ => Err("Invalid Operator"),
            }
          }
          _ => Err("Invalid math operation")
        }
      },
      // Defines a function that takes some arguments and executes a program based on those arguments. The code first checks if the function exists, and if it does, it creates a new scope in which to execute the function's statements. The code then executes each statement in the function's statements list and returns the result of the function's execution.
      Node::FunctionCall{name, children} => {
        let func_args = if children.len() > 0 {
          match &children[0] {
            Node::FunctionArguments{children} => { children },
            _ => children,
          }
        } else {
          children
        };
        let mut map = HashMap::new();
        let mut result: Result<Value, &'static str> = Err("Undefined function");
        let mut runtime = Runtime::new();
        match self.functions.get(name){
          Some(statements) => {
            {
              match statements[0].clone(){
              Node::FunctionArguments{children} => {
                for(i,arg) in children.iter().enumerate(){
                    let result = runtime.run(&func_args[i])?;
                    match arg {
                      Node::Expression{ children } => {
                        match &children[0] {
                          Node::Identifier{value} => {
                            map.insert(value.clone(), result);
                          },
                          _ => (),
                        }
                      }
                      _ => (),
                    }
                }
              }
              _ => (),
            }
          }
            self.stack.push(map);
            for temp in statements.clone(){
              result = self.run(&temp);
            }
            self.stack.pop();
          },
          None => (),
        };
        result
      },
      // Defines a new function based on the elements in the children argument. The name of the function is retrieved from the first element of the children, and the statements that define the function are retrieved from rest of hte children (head/tail). A new key-value pair is then inserted into the functions field of the current runtime object. If the function was successfully defined, the code returns a Value object with a boolean value of true, otherwise an error is returned.
      Node::FunctionDefine{children} => {
        let(head, tail) = children.split_at(1);
        match &head[0] {
          Node::Identifier{value} => {
            self.functions.insert(value.to_string(), tail.to_vec());
          },
          _ => (),
        }
        Ok(Value::Bool(true))
      },
      // Calls the run method on the first element in the children argument, which recursively evaluates the AST of the program being executed and returns the resulting value or error message.
      Node::FunctionReturn{children} => {
        self.run(&children[0]) //should work, check back later if not
      },
      // Retrieves the value of a variable from the current frame on the stack. If the variable is defined in the current frame, the code returns its value. If the variable is not defined in the current frame, the code returns an error message.
      Node::Identifier{value} => {
        let size = self.stack.len() -1 ;
        match self.stack[size].get(value){
          Some(identifier_value) => Ok(identifier_value.clone()),
          None => Err("Undefined variable")
        }
      },
      // Checks the type of the first element in the children argument and deciding what to do based on that type. If the type is a VariableDefine or FunctionReturn node, the code runs the run method on that node and returns the result.
      Node::Statement{children} => {
        match children[0] {
          Node::VariableDefine{..} | Node::FunctionReturn{..} => {
            self.run(&children[0])
          },
          _ => Err("Unknown statement"),
        }
      },
      // Defines a new variable by assigning a name and a value to it. The name is retrieved from the first element of the children argument, and the value is retrieved by running the run method on the second element of the children argument. The key-value pair is then inserted into the last frame on the stack field of the current runtime object.
      Node::VariableDefine{children} => {
        //get the name of variable/identifier
        let name: String = match &children[0] {
          Node::Identifier{value} => value.clone(),
          _ => "".to_string(), //figure out later
        };

        //get the result of the expression
        let value = self.run(&children[1])?;
        let last = self.stack.len() - 1;
        self.stack[last].insert(name, value.clone());
        Ok(value)
      },
      Node::Expression{children} => { //boolean, function_call, number, string, identifier, math_expression
        match children[0] {
          Node::MathExpression{..} | Node::Number{..} | Node::FunctionCall{..} | Node::String{..} | Node::Bool{..} | Node::Identifier{..} => {
            self.run(&children[0])
          },
          _ => Err("Unknown expression"),
        }
      },
      Node::Number{value} => {
        Ok(Value::Number(*value))
      },
      Node::String{value} => {
        Ok(Value::String(value.clone()))
      },
      Node::Bool{value} => {
        Ok(Value::Bool(*value))
      },
      _ => Err("Unhandled Node")
      
    }
  }

}

pub fn run(node: &Node) -> Result<Value, &'static str> {
  let mut runtime = Runtime::new();
  runtime.run(node);
  let start_main = Node::FunctionCall{name: "main".to_string(), children: vec![]};
  runtime.run(&start_main)
}
