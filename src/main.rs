#[derive(Debug)]
enum Token {
  Word(String),
  Backslash,
  LeftBrace,
  RightBrace,
  Paragraph, 
  EOF
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

fn eat (tokens: &mut Vec<Token>, pattern: Token) {
  println!("eat");
  // TODO: syntax error on invalid token
  tokens.remove(0);
}

fn eat_command (tokens: &mut Vec<Token>) -> AST {
  println!("eat command");
  // eat(tokens, Token::Backslash);
  let first = tokens.remove(0);
  println!("eat: {:?}", first);
  let name = match first {
    Token::Word(w) => w,
    _ => panic!("#oops")
  };

  eat(tokens, Token::LeftBrace);
  println!("tokens before blk: {:?}", tokens);
  let internal = eat_block(tokens);
  eat(tokens, Token::RightBrace);

  println!("tokens after cmd: {:?}", tokens);
  AST::Command(name, Box::new(internal))
}

fn eat_block (tokens: &mut Vec<Token>) -> AST {
  println!("eat block");
  let mut block: Vec<AST> = vec![];
  loop {
    println!("tokens: {:?}", tokens);
    let ast = match tokens.remove(0) {
      Token::Word(w) => AST::Word(w),
      Token::Backslash => eat_command(tokens),
      x => {
        // TODO: peek at top instead of doing this
        tokens.insert(0, x);
        break;
      }
    };
    block.push(ast);
  }

  AST::Block(block)
}

fn parse (tokens: &mut Vec<Token>) -> AST {
  // TODO: multiple blocks
  let ast = eat_block(tokens);
  eat(tokens, Token::EOF);

  ast
}

fn unparse_block (s: &mut String, ast: &AST) {
  let mut internal = match ast {
    &AST::Block(ref x) => x,
    _ => panic!("that's not a block!")
  };

  for a in internal.iter() {
    match a {
      // TODO: whitespace
      &AST::Word(ref w) => s.push_str(w.as_str()),
      &AST::Command(ref cmd, ref inner_ast) => unparse_command(s, cmd, inner_ast),
      _ => panic!("what's this!")
    };
    s.push_str(" ");
  }
  s.pop();
}

fn unparse_command (s: &mut String, cmd: &String, ast: &AST) {
  s.push_str(format!("<{}>", cmd).as_str());
  unparse_block(s, ast);
  s.push_str(format!("</{}>", cmd).as_str());
}

fn unparse (ast: &AST) -> String {
  let mut s = "".to_owned();

  s.push_str("<p>");
  unparse_block(&mut s, ast);
  s.push_str("</p>");

  s
}

fn main () {
  let text: String = "a oidmaowid mad \\b{aiodmaowidm}".to_string();
  let mut tokens = tokenize(&text);
  println!("{:?}", tokens);
  let parsed = parse(&mut tokens);
  println!("{}\n{:?}", &text, &parsed);
  let unparsed = unparse(&parsed);
  println!("{}", unparsed);
}
