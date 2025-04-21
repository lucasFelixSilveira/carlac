use crate::{lexer::{Lexames, Token, TokenKind}, utils::Utils};

pub fn generate(lexames: Lexames) -> Lexames {
  let mut tokens: Lexames = Lexames::new();

  let next = move |i: &mut usize, lexames: &Lexames| -> Option<Token> { 
    let temp = lexames.get(*i).cloned();
    *i += 1; 
    temp
  }; 
  
  let mut i: usize = 0; 
  while i < lexames.len() {
    let first: &Token = &lexames[i];

    let mut buffer: String = String::new();

    match first.kind {
      TokenKind::Symbol => {
        let backup: usize = i;

        let mut pointers: bool = true;
        let mut is_type: bool = false;

        loop {
          let tk: Option<Token> = next(&mut i, &lexames);

          match &tk {
            None => break,
            Some(ch) => {
              match ch.buffer.as_str() {
                "&" if pointers => {
                  buffer.push('&');
                  is_type = true;
                } 
                "[" => {
                  buffer.push('[');
                  if pointers { pointers = !pointers }
                  
                  let nc: Token = next(&mut i, &lexames).unwrap();
                  match nc.kind {
                    TokenKind::Number => {
                      buffer.push_str(&nc.buffer);
                      let close: Token = next(&mut i, &lexames).unwrap();
                      if close.buffer == "]" {  buffer.push(']') }
                      /* Need error here */
                      is_type = true;
                    },

                    TokenKind::Symbol if nc.buffer == "]" => {
                      is_type = true;
                      buffer.push(']');
                    }

                    /* Need error here */
                    _ => break
                  }
                }
                _ => { i -= 1; break; }
              }
            }
          }
        }

        if! is_type {
          i = backup + 1;
          tokens.push(first.clone());
          continue;
        }

        
        let ttk: Token = next(&mut i, &lexames).unwrap();
        match ttk.kind {
          TokenKind::Identifier if Utils::is_type(&ttk.buffer) => buffer.push_str(&ttk.buffer),

          /* Need error here */
          _ => {}
        }

        tokens.push(Token {
          kind: TokenKind::Type,
          location: first.location,
          buffer: buffer
        });

      }

      TokenKind::Identifier if Utils::is_type(&first.buffer) => {
          tokens.push(Token {
            kind: TokenKind::Type,
            location: first.location,
            buffer: first.buffer.clone()
          });
        i += 1;
      },

      _ => {
        tokens.push(first.clone());
        i += 1;
      }
    }

  } 

  tokens
}