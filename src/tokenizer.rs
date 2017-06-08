#[derive(Debug,Clone,PartialEq)]
pub enum Token {
  Word(Box<String>),
  Backslash,
  LeftBrace,
  RightBrace,
  Paragraph, 
  EOF
}

fn flush_token (tokens: &mut Vec<Token>, token_buf: &Vec<char>) {
  if token_buf.len() == 0 { return }

  let s: String = token_buf.into_iter().collect();
  tokens.push(Token::Word(Box::new(s)));
}

pub fn tokenize (text: &String) -> Vec<Token> {
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
