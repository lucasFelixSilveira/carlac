use std::{env, path::{self, PathBuf}};

pub trait LocalPath {
  fn to_local_path(self) -> String;
}

impl LocalPath for String {
  fn to_local_path(self) -> String {
    let project: PathBuf = env::current_dir().unwrap();
    format!("{}{}{}", project.display(), path::MAIN_SEPARATOR, self)    
  } 
}

impl LocalPath for &str {
  fn to_local_path(self) -> String {
    let project: PathBuf = env::current_dir().unwrap();
    format!("{}{}{}", project.display(), path::MAIN_SEPARATOR, self)    
  } 
}