use crate::{errors::{CarlaError, ErrorContent, TipContent}, lexer::{Lexames, Token, TokenKind}};

#[derive(Debug, Clone, PartialEq)]
pub struct Definition {
  pub identifier: String,
  pub ctype: String,
  pub hopeful: bool
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimpleValue {
  pub value: String
}

#[derive(Debug, Clone, PartialEq)]
pub enum SingleExpr {
  Numeric(String),
  Text(String),
  Identifier(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
  Return,
  Evoke(String)
}

#[derive(Debug, Clone, PartialEq)]
pub enum LambdaKind {
  Common,
  Our,
  Bound
}

#[derive(Debug, Clone, PartialEq)]
pub enum NodeKind {
  Definition(Definition),
  Macro(SimpleValue),
  Keyword(Keywords),
  Type(SimpleValue),
  Single(SingleExpr),
  Lambda(LambdaKind),
  BoundLibc(&'static str),
  Open,  Close,
  Begin, End,
  Cut
}

#[derive(Debug, Clone)]
pub struct Node {
  pub kind: NodeKind,
  pub location: (usize, usize)
}

pub type Nodes = Vec<Node>;

pub fn generate(lexames: Lexames, file: &String) -> Nodes {
  let mut nodes: Nodes = Nodes::new();

  let next = move |i: &mut usize, lexames: &Lexames| -> Option<Token> { 
    let temp: Option<Token> = lexames.get(*i).cloned();
    *i += 1; 
    temp
  }; 

  let mut syscall: bool = false;

  let mut i: usize = 0;
  while i < lexames.len() {
    let first: &Token = &lexames[i];

    match first.kind {

      /* Parse if is a definition */
      TokenKind::Type => {
        i += 1;
        let id_comma: Option<Token> = next(&mut i, &lexames);
        if let Some(id_comma_tk) = id_comma {
          match id_comma_tk.kind {

            /* If the next token is an identifier, it is a definition. */
            TokenKind::Identifier => {
              if let Some(sign) = next(&mut i, &lexames) {
                match sign.kind {

                  /* Request the '=', ',' or ';' use. */
                  TokenKind::Operator if sign.buffer == "=" || sign.buffer == "," || sign.buffer == ";" => {
                    nodes.push(Node {
                      kind: NodeKind::Definition(Definition { 
                        identifier: id_comma_tk.buffer.clone(), 
                        ctype: first.buffer.clone(), 
                        hopeful: sign.buffer == "=" 
                      }),
                      location: first.location
                    });

                    continue;
                  }

                  TokenKind::Symbol => {
                    i -= 1;
                    nodes.push(Node {
                      kind: NodeKind::Definition(Definition { 
                        identifier: id_comma_tk.buffer.clone(), 
                        ctype: first.buffer.clone(), 
                        hopeful: false
                      }),
                      location: first.location
                    });

                    continue;
                  }

                  _ => {}
                }  
              } 

              /* If don't have a next token, or it is not a valid token, emit an error */
              CarlaError::emit(CarlaError { 
                file: Some(file.clone()),
                title: "Has been expected '=', ',' or ';'".into(), 
                content: Some(
                    ErrorContent::Tip(TipContent {
                    tip: format!("Try put the ';' after {}", id_comma_tk.buffer),
                    docs: "carla-corp.github.io/carla/docs?search=definitions".into()
                  })
                ), 
                buffer: id_comma_tk.buffer,
                location: id_comma_tk.location,
              });
            }

            _ => {}
          }
        }

        i -= 1;
        nodes.push(Node {
          kind: NodeKind::Type(SimpleValue {
            value: first.buffer.clone()
          }),
          location: first.location
        });
      },

      TokenKind::Symbol if first.buffer == "(" => {
        i += 1;
        let backup: usize = i;
        loop {
          let prox: Option<Token> = next(&mut i, &lexames);
          if let Some(tk) = prox {
            if tk.buffer != ")" { continue; }

            let bracket_privacity: Option<Token> = next(&mut i, &lexames);
            if let Some(bp) = bracket_privacity {
              match bp.kind {
                TokenKind::Keyword if bp.buffer == "our" || bp.buffer == "bound" => {

                  let begin: Token = next(&mut i, &lexames).unwrap();
                  if begin.buffer == "{" {
                    nodes.push(Node {
                      kind: NodeKind::Lambda(
                        match bp.buffer.as_str() {
                          "our" => LambdaKind::Our,
                          "bound" => LambdaKind::Bound,
                          _ => LambdaKind::Common
                        }
                      ),
                      location: first.location
                    });
                    i = backup;
                    break;
                  } 

                  CarlaError::emit(CarlaError { 
                    file: Some(file.clone()),
                    title: "Has been expected '{'".into(), 
                    content: None, 
                    buffer: bp.buffer,
                    location: bp.location,
                  });

                }

                TokenKind::Symbol if bp.buffer == "{" => {
                  nodes.push(Node {
                    kind: NodeKind::Lambda(LambdaKind::Common),
                    location: first.location
                  });
                  i = backup;
                  break;
                }

                _ => {}
              }
            }
          }

          i = backup;
          nodes.push(Node {
            kind: NodeKind::Open,
            location: first.location
          });
        }
      }

      TokenKind::Symbol if first.buffer == ")" => {
        i += 1;
        nodes.push(Node {
          kind: NodeKind::Close,
          location: first.location
        });
      }

      TokenKind::Symbol if first.buffer == "{" => {
        i += 1;
        nodes.push(Node {
          kind: NodeKind::Begin,
          location: first.location
        });
      }

      TokenKind::Symbol if first.buffer == "}" => {
        i += 1;
        nodes.push(Node {
          kind: NodeKind::End,
          location: first.location
        });
      }

      TokenKind::Symbol if first.buffer == "#" => {
        i += 1;
        if let Some(preprocessor) = next(&mut i, &lexames) {
          if preprocessor.kind == TokenKind::Keyword {
            nodes.push(Node {
              kind: NodeKind::Macro(SimpleValue {
                value: preprocessor.buffer.clone()
              }),
              location: preprocessor.location
            });
          }
        }
      }

      TokenKind::Operator if first.buffer == ";" => {
        i += 1;
        nodes.push(Node {
          kind: NodeKind::Cut,
          location: first.location
        });
      }

      TokenKind::Number | TokenKind::Text | TokenKind::Identifier => {
        i += 1;
        let next: Token = next(&mut i, &lexames).unwrap();
        if next.kind == TokenKind::Operator {
          match next.buffer.as_str() {
            ";" | "," => {
              nodes.push(Node {
                location: first.location,
                kind: NodeKind::Single(
                  match first.kind {
                    TokenKind::Identifier => SingleExpr::Identifier,
                    TokenKind::Number     => SingleExpr::Numeric,
                    TokenKind::Text       => SingleExpr::Text,
                    _                     => todo!()
                  }(first.clone().buffer)
                ),
              });
            },
            _   => {}
          }
        }
      }

      TokenKind::Keyword if! [ 
        String::from("bound"),
        String::from("our"),
      ].contains(&first.buffer) => {
        i += 1;
        nodes.push(Node {
          location: first.location,
          kind: NodeKind::Keyword(
            match first.buffer.as_str() {
              "return" => Keywords::Return,
              "evoke" => {
                let colon: Token = next(&mut i, &lexames).unwrap();
                if colon.buffer != ":" {
                  CarlaError::emit(CarlaError { 
                    file: Some(file.clone()),
                    title: "Has been expected ':'".into(), 
                    content: None, 
                    buffer: colon.buffer,
                    location: colon.location,
                  });
                }

                let libc: Token = next(&mut i, &lexames).unwrap();
                Keywords::Evoke(libc.buffer.clone())
              }
              _ => todo!()
            }
          )
        })
      }

      TokenKind::Keyword if first.buffer == "bound" => {
        i += 1;
        let module: Token = next(&mut i, &lexames).unwrap();
        let temp: bool = syscall;
        syscall = syscall || module.buffer == "syscall";
        if temp != syscall { 
          nodes.push(Node {
            location: first.location,
            kind: NodeKind::BoundLibc("syscall")
          });

          continue;
        }
      }

      _ => { i += 1; }
    }
  } 

  nodes
}