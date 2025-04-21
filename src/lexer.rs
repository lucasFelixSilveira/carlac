use std::str::Chars;
use std::iter::Peekable;

use crate::utils::Utils;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
  Keyword,
  Operator,
  Identifier,
  Number,
  Type,
  Text,
  Symbol
}

#[derive(Debug, Clone)]
pub struct Token {
  pub buffer: String,
  pub kind: TokenKind,
  pub location: (usize, usize),
}

pub type Lexames = Vec<Token>;

pub struct Lexer<'a> {
  chars: Peekable<Chars<'a>>,
  buffer: Option<char>,
}

impl<'a> Lexer<'a> {
  pub fn new(input: &'a str) -> Self {
    Lexer {
      chars: input.chars().peekable(),
      buffer: None,
    }
  }

  pub fn getc(&mut self) -> Option<char> {
    if let Some(ch) = self.buffer.take() {
      Some(ch)
    } else {
      self.chars.next()
    }
  }

  pub fn ungetc(&mut self, ch: char) {
    self.buffer = Some(ch);
  }
}

#[allow(unused)]
trait AddWithoutDeclaration {
  fn add(&mut self, a: String, b: TokenKind, c: (usize, usize));
}

impl AddWithoutDeclaration for Lexames {
  fn add(&mut self, a: String, b: TokenKind, c: (usize, usize)) {
    self.push(Token {
      buffer: a,
      kind: b,
      location: c
    });
  }
}

pub fn check(lexames: &mut Lexames, buff: &mut String, location: &mut (usize, usize)) {
  if Utils::is_keyword(buff)         { lexames.add(buff.clone(), TokenKind::Keyword,    *location); }
  else if Utils::is_identifier(buff) { lexames.add(buff.clone(), TokenKind::Identifier, *location); }
  else if Utils::is_number(buff)     { lexames.add(buff.clone(), TokenKind::Number,     *location); }
  else { lexames.add(buff.clone(), TokenKind::Symbol, *location); }

  location.1 += buff.len();
  buff.clear();
}

pub fn tokenize(ctx: String) -> Lexames {

  let double_operators = [
    "::", ":.", 
    "==", ">=", "!=", "<=", 
    ">>", "<<", 
    ".."
  ];

  let single_operators = [
    ':', '=', 
    '*', '/', '+', '-', '%',
    '>', '<', '!',
    '.', ';', ','
  ];

  let mut lexames: Lexames = Vec::new();
  let mut lexer = Lexer::new(&ctx);
  let mut location: (usize, usize) = (1,1);

  let mut buff: String = String::new();
  while let Some(ch) = lexer.getc() {

    if ch.is_alphanumeric() {
      buff.push(ch);
      continue;
    }

    if buff.len() > 0 {
      check(&mut lexames, &mut buff, &mut location);    
    }

    if ch == '"' {
      while let Some(nch) = lexer.getc() {
        if nch == '"' { break; }
        buff.push(nch);
      }
      lexames.add(
        buff.clone(), 
        TokenKind::Text,
        location
      );
      location.1 += buff.len() + 2;
      buff.clear();
      continue;
    }

    if ch == ' ' {
      location.1 += 1;
    }

    if ch == '\n' {
      location.0 += 1;
      location.1 =  1;
      continue;
    }

    if ch == '-' {
      if let Some(ch2) = lexer.getc() {
        if ch2 == '-' {
          /* If it is a single line comment, ignore everything after them. */
          while let Some(ln) = lexer.getc() {
            if ln == '\n' { break; } 
          }
          lexer.ungetc('\n');
          continue;
        }     
        lexer.ungetc(ch2);
      }
    }

    /* 
      If the character is an single operator, we need check if it 
      is a double operator before put in the `lexames` vector 
    */
    if single_operators.contains(&ch) {
      if let Some(ch2) = lexer.getc() {
        let mut temp: String = String::from(ch);
        temp.push(ch2);
        if double_operators.contains(&temp.as_str()) {
          lexames.add(temp.clone(), TokenKind::Operator, location);
          location.1 += 2;
          continue;
        }
        lexer.ungetc(ch2);
      }
      lexames.add(String::from(ch), TokenKind::Operator, location);
      location.1 += 1;
      continue;
    }


    /* If it is not a white space, like 0x0d or others, put the character in the buffer */
    if! ch.is_whitespace() {
      lexames.add(String::from(ch), TokenKind::Symbol, location);
      location.1 += 1;
      continue;
    }
  }

  if buff.len() > 0 {
    check(&mut lexames, &mut buff, &mut location);    
  }

  lexames
}
