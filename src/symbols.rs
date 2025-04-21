use std::iter::Map;

use crate::llvm::scope::Cause;

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolKind {
  Lambda(LambdaData),
  Struct,
  Enum
}

#[derive(Debug, Clone, PartialEq)]
pub struct LambdaData {
  pub ctype: String,
  pub internal: bool,
  pub bound: bool,
}

#[derive(Debug, Clone)]
pub struct Symbol {
  pub public: bool,
  pub identifier: String,
  pub kind: SymbolKind
}

pub type Content = String;
pub type Symbols = Vec<Symbol>;
pub type Shard   = (Symbols, Content); 
pub type Scope   = (Symbols, Option<(String, usize)>, Cause); 
// ^- Símbolos do escopo anterior
// ^- Conteúdo completo
// ^- LLVM ID
// ^- Causa da quebra de escopo
pub type Scopes  = Vec<Scope>;