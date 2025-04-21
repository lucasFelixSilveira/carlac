mod carla;
mod symbols;
mod errors;
mod common;
mod parser;
mod lexer;
mod types;
mod utils;
mod llvm;

use std::{env, fs};
use common::LocalPath;
use lexer::Lexames;
use parser::Nodes;
use utils::Ir;
// use symbols::Files;

fn main() {
  /* Collect all files entries on the process call */
  let files: Vec<String> = env::args().skip(1).collect(); 
  
  /* Create the ./target folder if it not exist */
  carla::structurize();

  /* Check if the file is different than the past compilation */
  // carla::check_cache(&files);

  // let mut symbols: Files = Files::new();

  /* Make the same with all the files */
  for file in files {
    /* Checks if the given file is a Carla file */
    if! carla::file(&file) { continue; }

    /* Collect the file content if it exist */
    let file_path: String = file.clone().to_local_path();
    if let Ok(content) = fs::read_to_string(&file_path) {
      let lexer:  Lexames = lexer::tokenize(content);

      let mut constant_strings: Vec<(usize, String)> = vec![];
      for lexame in &lexer {
        if lexame.kind == lexer::TokenKind::Text {
          constant_strings.push( 
            (lexame.buffer.ir_len(), lexame.buffer.clone()) 
          );
        }
      }

      let types: Lexames = types::generate(lexer); 
      let parser: Nodes  = parser::generate(types, &file_path); 
      println!("nodes: {parser:#?}");

      // let llvm_path: String = ["target/out/", file.rsplit_once('.').unwrap().0, ".ll"].concat();
      llvm::generate(parser, file_path, constant_strings);
    }
  }
}