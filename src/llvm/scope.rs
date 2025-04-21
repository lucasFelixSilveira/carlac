use crate::symbols::Scope;

use super::{SymbolTable, Tunnel};

#[derive(Debug, Clone, PartialEq)]
pub enum Cause {
  Lambda(&'static str),
}

#[allow(unreachable_patterns)]
pub fn up(table: &mut SymbolTable<'_>, tunnel: &mut Tunnel<'_>, cause: Cause) {
  let data: Option<(String, usize)> = match cause {
    Cause::Lambda(_) => {
      let tmp = (tunnel.shard.clone(), *tunnel.var);
      tunnel.shard.clear();
      *tunnel.var = 0;
      Some(tmp)
    }
    _ => None
  };

  let scope: Scope = (table.symbols.clone(), data, cause);
  table.symbols.clear();
  table.scopes.push(scope);
}

pub fn down(table: &mut SymbolTable<'_>, tunnel: &mut Tunnel<'_>) {
  let Some(scope) = table.scopes.last() else { unreachable!() };

  table.symbols.clear();
  for symbol in scope.0.clone() {
    table.symbols.push(symbol);
  }

  if scope.1.is_some() {
    let content = scope.1.clone().unwrap();
    tunnel.shards.push(tunnel.shard.clone());
    tunnel.shard.clear();
    tunnel.shard.push_str(&content.0);
    *tunnel.var = content.1
  }
}