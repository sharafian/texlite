use tokenizer::Token;

#[derive(Debug)]
pub enum AST {
  Block(Vec<AST>),
  Word(String),
  Command(String, Box<AST>)
}

fn eat (tokens: &mut Vec<Token>, pattern: Token) {
  // println!("eat {:?} from {:?}", &pattern, tokens);
  match (&tokens[0], pattern) {
    (&Token::Backslash, Token::Backslash) => {},
    (&Token::LeftBrace, Token::LeftBrace) => {},
    (&Token::RightBrace, Token::RightBrace) => {},
    (&Token::Paragraph, Token::Paragraph) => {},
    (&Token::EOF, Token::EOF) => {},
    _ => panic!("doesn't match! could not eat token")
  }

  tokens.remove(0);
}

fn eat_command (tokens: &mut Vec<Token>) -> AST {
  // println!("eat command");
  // eat(tokens, Token::Backslash);
  let first = tokens.remove(0);
  // println!("eat: {:?}", first);
  let name = match first {
    Token::Word(w) => w,
    _ => panic!("#oops")
  };

  eat(tokens, Token::LeftBrace);
  // println!("tokens before blk: {:?}", tokens);
  let internal = eat_block(tokens);
  eat(tokens, Token::RightBrace);

  // println!("tokens after cmd: {:?}", tokens);
  AST::Command(*name, Box::new(internal))
}

fn eat_block (tokens: &mut Vec<Token>) -> AST {
  // println!("eat block");
  let mut block: Vec<AST> = vec![];
  loop {
    // println!("tokens: {:?}", tokens);
    let ast = match tokens.remove(0) {
      Token::Word(w) => AST::Word(*w),
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

pub fn parse (tokens: &mut Vec<Token>) -> Vec<AST> {
  let mut block_list = vec![];

  loop {
    block_list.push(eat_block(tokens));
    match &tokens[0] {
      &Token::EOF => break,
      &Token::Paragraph => eat(tokens, Token::Paragraph),
      _ => panic!("expected paragraph break or EOF")
    };
  }

  block_list
}
