use lexer::*;

#[test]
fn test_01() {
  assert_eq!(lex("123"),vec![Token::Digit(b'1'), Token::Digit(b'2'), Token::Digit(b'3'), Token::EOF]);
}

#[test]
fn test_02() {
  assert_eq!(lex("abc"),vec![Token::Alpha(b'a'), Token::Alpha(b'b'), Token::Alpha(b'c'), Token::EOF]);
}

#[test]
fn test_03() {
  assert_eq!(lex("hello world"),vec![Token::Alpha(b'h'), Token::Alpha(b'e'), Token::Alpha(b'l'), Token::Alpha(b'l'), Token::Alpha(b'o'), Token::WhiteSpace, Token::Alpha(b'w'), Token::Alpha(b'o'), Token::Alpha(b'r'), Token::Alpha(b'l'), Token::Alpha(b'd'), Token::EOF]);
}

#[test]
fn test_04() {
  assert_eq!(lex("true"),vec![Token::Keyword(vec![b't', b'r', b'u', b'e']), Token::EOF]);
}

#[test]
fn test_05() {
  assert_eq!(lex("false"),vec![Token::Keyword(vec![b'f', b'a', b'l', b's', b'e']), Token::EOF]);
}

#[test]
fn test_06() {
  assert_eq!(lex("let x = 123;"),vec![
    Token::Keyword(vec![b'l', b'e', b't']), 
    Token::WhiteSpace, 
    Token::Alpha(b'x'), 
    Token::WhiteSpace,
    Token::Equal,
    Token::WhiteSpace,
    Token::Digit(b'1'),
    Token::Digit(b'2'),
    Token::Digit(b'3'),
    Token::Semicolon,
    Token::EOF,
  ]);
}

#[test]
fn test_07() {
  assert_eq!(lex(r#"let x = 123;let y="abc";"#),vec![
    Token::Keyword(vec![b'l', b'e', b't']), 
    Token::WhiteSpace, 
    Token::Alpha(b'x'), 
    Token::WhiteSpace,
    Token::Equal,
    Token::WhiteSpace,
    Token::Digit(b'1'),
    Token::Digit(b'2'),
    Token::Digit(b'3'),
    Token::Semicolon,
    Token::Keyword(vec![b'l', b'e', b't']), 
    Token::WhiteSpace, 
    Token::Alpha(b'y'), 
    Token::Equal,
    Token::Quote,
    Token::Alpha(b'a'), 
    Token::Alpha(b'b'), 
    Token::Alpha(b'c'), 
    Token::Quote,
    Token::Semicolon,
    Token::EOF,
  ]);
}

#[test]
fn test_08() {
  assert_eq!(lex(r#"fn main() {}"#),vec![
    Token::Keyword(vec![b'f', b'n']), 
    Token::WhiteSpace, 
    Token::Alpha(b'm'), 
    Token::Alpha(b'a'),
    Token::Alpha(b'i'),
    Token::Alpha(b'n'),
    Token::LeftParen,
    Token::RightParen,
    Token::WhiteSpace,
    Token::LeftCurly,
    Token::RightCurly,
    Token::EOF,
  ]);
}


#[test]
fn test_09() {
  assert_eq!(lex(r#"fn foo(a,b,c) {
  let x=a+1;
	let y=bar(c-b);
  return x*y;
}"#),vec![
    Token::Keyword(vec![b'f', b'n']), 
    Token::WhiteSpace, 
    Token::Alpha(b'f'), 
    Token::Alpha(b'o'),
    Token::Alpha(b'o'),
    Token::LeftParen,
    Token::Alpha(b'a'),
    Token::Comma,
    Token::Alpha(b'b'),
    Token::Comma,
    Token::Alpha(b'c'),
    Token::RightParen,
    Token::WhiteSpace,
    Token::LeftCurly,
    Token::WhiteSpace,
    Token::WhiteSpace,
    Token::WhiteSpace,
    Token::Keyword(vec![b'l', b'e', b't']), 
    Token::WhiteSpace, 
    Token::Alpha(b'x'),
    Token::Equal,
    Token::Alpha(b'a'),
    Token::Plus,
    Token::Digit(b'1'),
    Token::Semicolon,
    Token::WhiteSpace, 
    Token::WhiteSpace, 
    Token::Keyword(vec![b'l', b'e', b't']),  //let y=bar(c-b); return x*y;
    Token::WhiteSpace,
    Token::Alpha(b'y'),
    Token::Equal,
    Token::Alpha(b'b'),
    Token::Alpha(b'a'),
    Token::Alpha(b'r'),
    Token::LeftParen,
    Token::Alpha(b'c'),
    Token::Dash,
    Token::Alpha(b'b'),
    Token::RightParen,
    Token::Semicolon,
    Token::WhiteSpace,
    Token::WhiteSpace,
    Token::WhiteSpace,
    Token::Keyword(vec![b'r', b'e', b't', b'u', b'r', b'n']), 
    Token::WhiteSpace,
    Token::Alpha(b'x'),
    Token::Other,
    Token::Alpha(b'y'),
    Token::Semicolon,
    Token::WhiteSpace,
    Token::RightCurly,
    Token::EOF,
  ]);
}

#[test]
fn test_10() {
  assert_eq!(strip_whitespace(&lex(r#"fn foo(a,b,c) {
  let x=a+1;
	let y=bar(c-b);
  return x+y;
}"#)),vec![
    Token::Keyword(vec![b'f', b'n']), 
    Token::Alpha(b'f'), 
    Token::Alpha(b'o'),
    Token::Alpha(b'o'),
    Token::LeftParen,
    Token::Alpha(b'a'),
    Token::Comma,
    Token::Alpha(b'b'),
    Token::Comma,
    Token::Alpha(b'c'),
    Token::RightParen,
    Token::LeftCurly,
    Token::Keyword(vec![b'l', b'e', b't']), //let x=a+1;
    Token::Alpha(b'x'),
    Token::Equal,
    Token::Alpha(b'a'),
    Token::Plus,
    Token::Digit(b'1'),
    Token::Semicolon,
    Token::Keyword(vec![b'l', b'e', b't']), //let y=bar(c-b);
    Token::Alpha(b'y'),
    Token::Equal,
    Token::Alpha(b'b'),
    Token::Alpha(b'a'),
    Token::Alpha(b'r'),
    Token::LeftParen,
    Token::Alpha(b'c'),
    Token::Dash,
    Token::Alpha(b'b'),
    Token::RightParen,
    Token::Semicolon,
    Token::Keyword(vec![b'r', b'e', b't', b'u', b'r', b'n']), //return x+y;
    Token::Alpha(b'x'),
    Token::Plus,
    Token::Alpha(b'y'),
    Token::Semicolon,
    Token::RightCurly,
    Token::EOF,
  ]);
}
