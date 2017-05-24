use std::iter;

#[derive(Debug)]
enum Token {
  Backslash,
  LeftBrace,
  RightBrace,
  Paragraph, 
  EOF,
  Word(String)
}

#[derive(Debug)]
enum AST {
  Block(Vec<AST>),
  Word(String),
  Command(String, Box<AST>)
}

fn flush_token (tokens: &mut Vec<Token>, token_buf: &Vec<char>) {
  if token_buf.len() == 0 { return }

  let s: String = token_buf.into_iter().collect();
  tokens.push(Token::Word(s));
}

fn tokenize (text: &String) -> Vec<Token> {
  let mut tokens: Vec<Token> = vec![];
  let mut token_buf: Vec<char> = vec![];
  let mut newlines = 0;
  
  for c in text.chars() {
    if [' ', '\t', '\n', '\\', '{', '}'].contains(&c) {
      flush_token(&mut tokens, &token_buf);
      token_buf.clear();
    }

    match c {
      ' ' => {},
      '\t' => {},
      '\n' => {
        newlines = newlines + 1;
        if newlines >= 2 {
          tokens.push(Token::Paragraph);
          newlines = 0;
        }
      },
      '\\' => tokens.push(Token::Backslash),
      '{' => tokens.push(Token::LeftBrace),
      '}' => tokens.push(Token::RightBrace),
      _ => token_buf.push(c)
    }
  }

  flush_token(&mut tokens, &token_buf);
  tokens.push(Token::EOF);

  tokens
}

fn eat (tokens: &mut Iterator<Item=Token>, pattern: Token) {
  let next = match tokens.next() {
    Some(w) => w,
    None => panic!("#oops")
  };

  if next != pattern { panic!("#oops") } // Throw
}

fn eat_command (tokens: &mut Iterator<Item=Token>) -> AST {
  eat(&mut tokens, Token::Backslash);
  let name = match tokens.next() {
    Some(Token::Word(w)) => w,
    _ => panic!("#oops")
  };

  eat(&mut tokens, Token::LeftBrace);
  let internal = eat_block(&mut tokens);
  eat(&mut tokens, Token::RightBrace);

  AST::Command(name, Box::new(internal))
}

fn eat_block (tokens: &mut Iterator<Item=Token>) -> AST {
  let block: Vec<AST> = vec![];
  loop {
    let ast = match tokens.next() {
      Some(Token::Word(w)) => AST::Word(w),
      Some(Token::Backslash) => eat_command(&mut tokens),
      Some(_) => break,
      None => panic!("#oops")
    };
    block.push(ast);
  }

  AST::Block(block)
}

fn parse (tokens: &mut Iterator<Item=Token>) -> AST {
  eat_block(&mut tokens)
}

fn main () {
  let text: String = "a oidmaowid mad \\b{aiodmaowidm}".to_string();
  let tokens = tokenize(&text).iter();
  let parsed = parse(&mut tokens);
  
  println!("{}\n{:?}", &text, &parsed);
}
