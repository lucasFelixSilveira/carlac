use std::fs::{self, File, Metadata};
use std::io::{Read, Seek, SeekFrom};
use std::process::exit;
use std::time::SystemTime;
use serde::{Deserialize, Serialize};
use sha2::{Sha256, Digest};

use crate::common::LocalPath;

#[inline]
pub fn file(name: &String) -> bool { name.ends_with(".crl") }

pub fn structurize() {
  _ = fs::create_dir("target".to_local_path());
  _ = fs::create_dir("target/out".to_local_path());
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FileCache {
  pub size: u64,
  pub modified: Option<SystemTime>,
  pub partial_hash: [u8; 32],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FilesCache {
  pub length: usize,
  pub files: Vec<FileCache>
}

impl FileCache {
  fn from_file(path: String) -> std::io::Result<Self> {
    let metadata = std::fs::metadata(&path)?;
    let modified = metadata.modified().ok();
    let size = metadata.len();
    let mut file = File::open(path)?;
    
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64];
    
    if size > 64 {
      file.read_exact(&mut buffer)?;
      hasher.update(&buffer);

      if size > 128 {
        file.seek(SeekFrom::Start(size / 2))?;
        file.read_exact(&mut buffer)?;
        hasher.update(&buffer);

        file.seek(SeekFrom::End(-64))?;
        file.read_exact(&mut buffer)?;
        hasher.update(&buffer);
      }
    } else {
      file.read_to_end(&mut buffer.to_vec())?;
      hasher.update(&buffer);
    }
    
    let partial_hash = hasher.finalize().into();
    
    Ok(Self { size, modified, partial_hash })
  }
}

pub fn check_cache(files: &Vec<String>) {
  let cache_file: String = "target/cache.json".to_local_path();
  if let Ok(info) = fs::read_to_string(&cache_file) { 
    let mut cache: FilesCache = serde_json::from_str(&info).unwrap();
    if cache.length != files.len() { return; }

    let mut caches: Vec<FileCache> = Vec::new();
    for file in files.clone() {
      caches.push(
        FileCache::from_file(
          file.to_local_path()
        ).unwrap()
      );
    }

    let mut iterable: Vec<(FileCache, FileCache)> = Vec::new();

    for i in 0..caches.len() {
      iterable.push((caches[i].clone(), cache.files[i].clone()))
    }

    cache.files.clear();

    for (x, y) in iterable {
      if x.size == y.size && x.modified == y.modified && x.partial_hash == y.partial_hash { 
        exit(0); 
      }
    }

    let ctx = FilesCache {
      length: files.len(),
      files: caches.clone()
    };
    _ = fs::write(cache_file, serde_json::to_string_pretty(&ctx).unwrap());

    caches.clear();
  } 
  else {
    let mut caches: Vec<FileCache> = Vec::new();
    for file in files.clone() {
      caches.push(
        FileCache::from_file(
          file.to_local_path()
        ).unwrap()
      );
    }

    let ctx = FilesCache {
      length: files.len(),
      files: caches.clone()
    };

    _ = fs::write(cache_file, serde_json::to_string_pretty(&ctx).unwrap());
  }

  _ = fs::remove_file("target/latest.logs.json");
}