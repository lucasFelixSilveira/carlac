use std::{fs, process::exit};

use serde::{Deserialize, Serialize};

use crate::common::LocalPath;

#[derive(Clone, Serialize, Deserialize)]
pub struct CarlaError {
  pub file: Option<String>,
  pub title: String,
  pub buffer: String,
  pub content: Option<ErrorContent>,
  pub location: (usize, usize)
}

#[derive(Clone, Serialize, Deserialize)]
pub enum ErrorContent {
  Tip(TipContent)
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TipContent {
  pub tip: String,
  pub docs: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Definition {
  pub tip: String,
  pub docs: String,
  pub location: (usize, usize),
}

impl CarlaError {
  pub fn emit(data: CarlaError) {
    let path: String = "target/latest.logs.json".to_local_path();
    _ = fs::write(path, serde_json::to_string_pretty(&data).unwrap());
    exit(0);
  }
}