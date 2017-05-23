fn flush_token (tokens: &mut Vec<String>, token_buf: &Vec<char>) {
  if token_buf.len() == 0 { return }

  let s: String = token_buf.into_iter().collect();
  tokens.push(s);
}

fn tokenize (text: String) -> Vec<String> {
  let mut tokens: Vec<String> = vec![];
  let mut token_buf: Vec<char> = vec![];
  let mut newlines = 0;
  
  for c in text.chars() {
    match c {
      ' ' => {
        flush_token(&mut tokens, &token_buf);
        token_buf.clear();
      },
      '\t' => {
        flush_token(&mut tokens, &token_buf);
        token_buf.clear();
      },
      '\n' => {
        flush_token(&mut tokens, &token_buf);
        token_buf.clear();
        newlines = newlines + 1;
        if newlines >= 2 {
          tokens.push("\n".to_string());
          newlines = 0;
        }
      },
      '\\' => {
        flush_token(&mut tokens, &token_buf);
        token_buf.clear();
        tokens.push("\\".to_string());
      },
      '{' => {
        flush_token(&mut tokens, &token_buf);
        token_buf.clear();
        tokens.push("{".to_string());
      },
      '}' => {
        flush_token(&mut tokens, &token_buf);
        token_buf.clear();
        tokens.push("}".to_string());
      },
      _ => {
        token_buf.push(c);
        continue;
      }
    }
  }

  flush_token(&mut tokens, &token_buf);
  tokens
}
