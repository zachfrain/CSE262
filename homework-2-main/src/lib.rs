#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Keyword(Vec<u8>),
  Alpha(u8),
  Digit(u8),
  LeftParen,
  RightParen,
  LeftCurly,
  RightCurly,
  Equal,
  Plus,
  Dash,
  Quote,
  WhiteSpace,
  Semicolon,
  Comma,
  Other,
  EOF,
}

pub fn lex(input: &str) -> Vec<Token> {
    let mut temp: Vec<u8> = input.chars().map(|c| c as u8).collect();
    temp = find_keyword(temp);

    let mut tokens: Vec<Token> = Vec::new();
    for i in 0..temp.len() {
        let char = temp[i];
        if temp[i].is_ascii_whitespace() {
            tokens.push(Token::WhiteSpace);
            continue;
        }
        match char {
            0xDB => tokens.push(Token::Keyword(vec![b't', b'r', b'u', b'e'])),
            0xDC => tokens.push(Token::Keyword(vec![b'f', b'a', b'l', b's', b'e'])),
            0xDD => tokens.push(Token::Keyword(vec![b'f',b'n'])),
            0xDE => tokens.push(Token::Keyword(vec![b'r', b'e', b't', b'u', b'r', b'n'])),
            0xDF => tokens.push(Token::Keyword(vec![b'l', b'e', b't'])),
            0x41..=0x5A | 0x61..=0x7A => tokens.push(Token::Alpha(char)),
            0x30..=0x39 => tokens.push(Token::Digit(char)),
            0x28 => tokens.push(Token::LeftParen),
            0x29 => tokens.push(Token::RightParen),
            0x7B => tokens.push(Token::LeftCurly),
            0x7D => tokens.push(Token::RightCurly),
            0x3D => tokens.push(Token::Equal),
            0x2B => tokens.push(Token::Plus),
            0x2D => tokens.push(Token::Dash),
            0x22 => tokens.push(Token::Quote),
            0x3B => tokens.push(Token::Semicolon),
            0x2C => tokens.push(Token::Comma),
            _ => tokens.push(Token::Other),
        }
    }

    tokens.push(Token::EOF);

    tokens
}

pub fn strip_whitespace(tokens: &Vec<Token>) -> Vec<Token> {
  let w_t:Token = Token::WhiteSpace;
  let mut temp = tokens.clone();
  temp.retain(|token| *token != w_t);
  temp
}

pub fn find_keyword(mut input: Vec<u8>) -> Vec<u8>{
    loop {
        let temp = input.clone();
        input = kw_tr(input);
        input = kw_fa(input);
        input = kw_fn(input);
        input = kw_re(input);
        input = kw_le(input);
        if temp == input {
            break;
        }
    }
    input
}

pub fn kw_tr(mut input: Vec<u8>) -> Vec<u8> {  
  for i in 3..input.len()+1 {
        if input[i - 3] == 0x74 {
            if input[i - 2] == 0x72 {
                if input[i - 1] == 0x75 {
                    if input[i] == 0x65 {
                        input[i - 3] = 0xDB;
                        input.drain(i-2..i+1);
                        break;
                    }
                }
            }
        }
    }
  input
}

pub fn kw_fa(mut input: Vec<u8>) -> Vec<u8> { //0x66, 0x61, 0x6C, 0x73, 0x65
    for i in 4..input.len()+1 {
        if input[i - 4] == 0x66 {
            if input[i - 3] == 0x61 {
                if input[i - 2] == 0x6C {
                    if input[i - 1] == 0x73 {
                        if input[i] == 0x65 {
                            input[i - 4] = 0xDC;
                            input.drain(i-3..i+1);
                            break;
                        }
                    }
                }
            }
        }
    }
    input
}

pub fn kw_fn(mut input: Vec<u8>) -> Vec<u8> {
    for i in 1..input.len()+1 {
        if input[i - 1] == 0x66 {
            if input[i] == 0x6E {
                input[i - 1] = 0xDD;
                input.drain(i..i+1);
                break;
            }
        }
    }
    input
}

pub fn kw_re(mut input: Vec<u8>) -> Vec<u8> {
    for i in 5..input.len()+1 {
        if input[i - 5] == 0x72 {
            if input[i - 4] == 0x65 {
                if input[i - 3] == 0x74 {
                    if input[i - 2] == 0x75 {
                        if input[i - 1] == 0x72 {
                            if input[i] == 0x6E {
                                input[i - 5] = 0xDE;
                                input.drain(i-4..i+1);
                                break;
                            }
                        }
                    }
                }
            }
        }
    }
    input
}

pub fn kw_le(mut input: Vec<u8>) -> Vec<u8> {
    for i in 2..input.len()+1 {
        if input[i - 2] == 0x6C {
            if input[i - 1] == 0x65 {
                if input[i] == 0x74 {
                    input[i - 2] = 0xDF;
                    input.drain(i-1..i+1);
                    break;
                }
            }
        }
    }
    input
}