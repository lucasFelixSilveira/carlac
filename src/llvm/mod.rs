use std::path;

use scope::Cause;
use serde_json::Number;

use crate::{parser::{Keywords, LambdaKind, Node, NodeKind, Nodes, SingleExpr}, symbols::{LambdaData, Scope, Scopes, Symbol, SymbolKind, Symbols}, utils::Constants};

pub mod scope;
mod vars;
mod utils;
mod types;
mod unicode;
mod definitions;

// enum Resolve {
//   VarDeclaration((Definition, usize)),
//   Return
// }
 
trait LikeAFile {
  fn write(&mut self, string: impl ToString);
  fn writeln_without_tab(&mut self, string: impl ToString);
  fn writeln(&mut self, string: impl ToString);
}

impl LikeAFile for String {
  fn write(&mut self, string: impl ToString) {
    self.push_str(&string.to_string());
  }

  fn writeln_without_tab(&mut self, string: impl ToString) {
    self.push_str(&string.to_string());
    self.push('\n');
  }

  fn writeln(&mut self, string: impl ToString) {
    self.push('\t');
    self.push_str(&string.to_string());
    self.push('\n');
  }
}

pub struct Tunnel<'a> {
  pub shard: &'a mut String,
  pub var: &'a mut usize,
  pub shards: &'a mut Vec<String>,
}

pub struct SymbolTable<'a> {
  pub symbols: &'a mut Vec<Symbol>,
  pub scopes: &'a mut Vec<Scope>,
}

impl LikeAFile for Tunnel<'_> {
  fn write(&mut self, string: impl ToString) {
    self.shard.write(&string.to_string());
  }

  fn writeln_without_tab(&mut self, string: impl ToString) {
    self.shard.writeln_without_tab(&string.to_string());
  }

  fn writeln(&mut self, string: impl ToString) {
    self.shard.writeln(&string.to_string());
  }
}

pub fn generate(nodes: Nodes, file: String, constant_strings: Vec<(usize, String)>) {
  let mut output: String = String::new();

  let consume = move |i: &mut usize, lexames: &Nodes| -> Option<Node> { 
    let temp: Option<Node> = lexames.get(*i).cloned();
    *i += 1; 
    temp
  }; 

  let filtred_filename: String = 
    file
      .rsplit_once(path::MAIN_SEPARATOR).unwrap().1
      .split_once('.').unwrap().0
      .to_string(); 

  let mut symbols: Vec<Symbol> = Symbols::new();
  let mut scopes: Vec<Scope>   = Scopes::new();
  let mut shard: String        = String::new(); 
  let mut shards: Vec<String>  = Vec::new();

  let mut scope: usize = 0;
  let mut i: usize = 0;
  let mut var: usize = 0;

  let mut tunnel: Tunnel<'_> = Tunnel {
    shard: &mut shard,
    shards: &mut shards,
    var: &mut var,
  };

  let mut table: SymbolTable<'_> = SymbolTable {
    symbols: &mut symbols,
    scopes: &mut scopes,
  };

  let mut main_return: String = String::new();

  while i < nodes.len() {
    let first: Node = consume(&mut i, &nodes).unwrap();

    match first.kind {

      NodeKind::Definition(def) => {
        let next: Node = consume(&mut i, &nodes).unwrap();
        match next.kind {
          
          NodeKind::Lambda(lkind) if scope == 0 => {
            if let LambdaKind::Bound = lkind {
              // Need Error here
              // Error cause: On the first scope, u cannot create a Bounded lambda
            }

            scope::up(&mut table, &mut tunnel, Cause::Lambda("main"));
            scope += 1;

            table.symbols.push(Symbol { 
              public: lkind == LambdaKind::Our,
              identifier: def.identifier.clone(),
              kind: SymbolKind::Lambda(LambdaData {
                ctype:    def.ctype.clone(),
                bound:    false,
                internal: false
              })
            });

            main_return = def.ctype.clone();
          
            tunnel.write(
              &format!("define {} @{}(",
                types::to_llvm::convert(&def.ctype),
                if def.identifier == "main" { def.identifier } else { 
                  format!("{}.{}", filtred_filename, def.identifier)
                }
              )
            );

            let mut len: usize = 0;
            let mut arguments: Vec<NodeKind> = Vec::new();
            loop {
              let argument: Node = consume(&mut i, &nodes).unwrap();
              if let NodeKind::Definition(arg) = argument.kind.clone() { 
                arguments.push(argument.kind);
                tunnel.write(
                  &format!("{}{} %{}",
                    if len > 0 { ", " } else { "" },
                    types::to_llvm::convert(&arg.ctype),
                    tunnel.var
                  )
                );
                len += 1;
                *tunnel.var += 1;
              } 
              else { break; }
            }

            tunnel.writeln_without_tab(") {");
            *tunnel.var += 1;

            let old: usize = *tunnel.var;
            for argument in &arguments {
              let NodeKind::Definition(def) = argument else { unreachable!() };
              definitions::alloca::create(&mut tunnel, &def);
            }

            for (index, argument) in arguments.iter().enumerate() {
              let NodeKind::Definition(def) = argument else { unreachable!() };
              vars::store::var(&mut tunnel, (def.ctype.clone(), [ index, old + index ]));
            }
          }

          NodeKind::Single(expr) => {
            let variable: usize = *tunnel.var;
            definitions::alloca::create(&mut tunnel, &def);

            if let SingleExpr::Numeric(number) = &expr {
              vars::store::numeric_literal(&mut tunnel, (def.ctype.clone(), [
                number.parse::<usize>().unwrap(),
                variable
              ]));
            }
          }
          
          _ => {}
        } 
      }

      NodeKind::Keyword(keyword) => {
        match keyword {
          Keywords::Evoke(func) => {
            let sys_numeric: String = types::to_llvm::convert(&"usize".to_string());
            let mut waiting: String = String::new();
            waiting.write(
              format!("call {} {} @{}(", 
                sys_numeric,

                match func.as_str() {
                  "syscall" => "(i64, ...)",
                  _ => ""
                },

                if cfg!(target_os="windows") {
                  ".carla.windows.syscall"
                } else { &func }
              )
            );

            let mut first: bool = true;
            loop {
              let next: Node = consume(&mut i, &nodes).unwrap();
              let prefix: &str = if first { "" } else { ", " };
              first = false;
              if let NodeKind::Single(expr) = next.kind { 
                match expr {
                  SingleExpr::Numeric(number) => {
                    waiting.write(
                      format!("{prefix}{sys_numeric} {number}")
                    );
                  },

                  SingleExpr::Text(string) => {
                    waiting.write(
                      format!("{prefix}ptr @.carla.cstr_{}", constant_strings.find_in(string).unwrap())
                    );
                  }

                  _ => todo!()
                }
              } else { 
                waiting.write(")");
                i -= 1;
                break; 
              }
            }
            tunnel.shard.write(
              format!("\t%{} = ", *tunnel.var)
            );
            tunnel.shard.writeln_without_tab(waiting);
            *tunnel.var += 1;
          }

          Keywords::Return => {}
        }
      }

      NodeKind::End => {
        let data = table.scopes.last().unwrap();
        let (_, _, cause) = data;

        scope -= 1;
        match cause {
          Cause::Lambda(name) => {
            if *name == "main" {
              tunnel.shard.writeln(
                format!("ret {} 0", 
                  types::to_llvm::convert(&main_return.clone())
                )
              );
            }

            tunnel.shard.writeln_without_tab("}");
            scope::down(&mut table, &mut tunnel);
          }
        }

      }

      _ => {}
    }
  }

  for shard in tunnel.shards.clone() {
    output.writeln_without_tab(shard);
  }

  output.write('\n');

  /* Quanto o multi-arquivos estiver em desenvolvimento, isso PRECISA ser modificado. */
  for (index, (size, string)) in constant_strings.iter().enumerate() {
    output.writeln_without_tab(
      format!("@.carla.cstr_{index} = constant [{size} x i8] c\"{string}\\00\", align 1")
    );
  }

  if! cfg!(target_os="windows") {
    let bits: usize = std::mem::size_of::<usize>() * 8;
    output.writeln_without_tab(
      format!("declare i{bits} @syscall(i{bits}, ...) nounwind")
    );
  }

  println!("output: \n{output}");
}