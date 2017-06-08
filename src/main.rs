use std::env;
use std::io::Read;
use std::fs::File;

mod tokenizer;
mod unparser;
mod parser;

fn main () {
  let filename = match env::args().nth(1) {
    Some(x) => x,
    None => panic!("args[1] must be filename!")
  };

  let mut file = File::open(filename.as_str()).expect("Unable to read file");
  let mut text = String::new();

  // TODO: line-by-line for larger files
  file.read_to_string(&mut text).expect("Unable to read string");

  let mut tokens = tokenizer::tokenize(&text);
  let parsed = parser::parse(&mut tokens);
  let unparsed = unparser::unparse(&parsed);
  println!("{}", unparsed);
}
