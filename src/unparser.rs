use parser::AST;

fn unparse_block (s: &mut String, ast: &AST) {
  let internal = match ast {
    &AST::Block(ref x) => x,
    _ => panic!("that's not a block!")
  };

  for a in internal.iter() {
    match a {
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

pub fn unparse (block_list: &Vec<AST>) -> String {
  let mut s = "".to_owned();

  for ast in block_list {
    s.push_str("<p>");
    unparse_block(&mut s, ast);
    s.push_str("</p>");
  }

  s
}
