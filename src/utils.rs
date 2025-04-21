pub struct Utils;

impl Utils {
  pub const KEYWORDS: [&str; 11] = [
    "const",  "if",    "else", "return",
    "struct", "enum",  "when", "case" , 
    "our",    "bound", "evoke"
  ];

  pub const MACRO: [&str; 1] = [
    "include"
  ];

  pub const TYPES: [&str; 17] = [
    "byte",  "char",    "ascii",  "void",   "usize",
    "int8",  "int16",   "int32",  "int64",  "int128",  "int256",
    "uint8", "uint16",  "uint32", "uint64", "uint128", "uint256",
  ];

  pub fn is_identifier(buffer: &String) -> bool {
    if buffer.len() > 64 { return false; }
    
    let chars: Vec<char> = buffer.chars().collect();
    let first: &char = chars.first().unwrap();

    if! first.is_alphabetic() { return false; }

    if buffer.len() > 1 {
      for ch in chars.iter().skip(1) {
        if! (ch.is_alphanumeric() || ch == &'_') {
          return false;
        }
      }
    }

    true
  }
  pub fn is_number(buffer: &String) -> bool {
    let chars: Vec<char> = buffer.chars().collect();
    for ch in chars {
      if! ch.is_numeric() { return false } 
    }
    true
  }

  pub fn is_keyword(buffer: &String) -> bool {
    for keyword in Self::KEYWORDS {
      if keyword == buffer { return true } 
    }
    false
  }

  pub fn is_type(buffer: &String) -> bool {
    for carla_type in Self::TYPES {
      if carla_type == buffer { return true } 
    }
    false
  }
}


pub trait Ir {
  fn ir_len(&self) -> usize;
}

impl Ir for String {
  fn ir_len(&self) -> usize {
    let mut base: usize = self.len() + 1;
    let chars: Vec<char> = self.chars().collect();

    let mut i: usize = 0;
    while i < chars.len() {
      let ch: char = chars[i];

      if ch == '\\' {
        base -= 1;
        if chars[i + 1] == '\\' {
          i += 1;
        }
      }

      i += 1;
    }

    base
  }
}

pub trait Constants {
  fn find_in(&self, ctx: impl ToString) -> Option<usize>;
}

impl Constants for Vec<(usize, String)> {
  fn find_in(&self, ctx: impl ToString) -> Option<usize> {
    for (i, x) in self.iter().enumerate() {
      if x.1 == ctx.to_string() {
        return Some(i);
      }
    }
    None
  }
}