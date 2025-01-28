// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self};
use std::io;
use std::path::Path;
use rayon::prelude::*;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_folder_size, get_subfolders_and_files, open_folder_in_explorer, delete_from_disk])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn get_folder_size(path: String) -> Result<u64, String> {
    let path = Path::new(&path);
    calculate_size(path).map_err(|e| e.to_string())
}

fn calculate_size(path: &Path) -> io::Result<u64> {
  WalkDir::new(path)
      .into_iter()
      .par_bridge()
      .try_fold(
          || 0u64,
          |acc, entry| {
              let entry = match entry {
                  Ok(entry) => entry,
                  Err(_) => return Ok(acc),
              };

              let metadata = entry.metadata()?;
              Ok(acc + metadata.len())
          },
      )
      .try_reduce(|| 0, |a, b| Ok(a + b))
}

#[tauri::command]
fn get_subfolders_and_files(path: String) -> Result<Vec<String>, String> {
  let path = std::path::Path::new(&path);
  let subfolders_and_files = match std::fs::metadata(path) {
    Ok(metadata) => {
      if metadata.is_dir() {
        let mut subfolders_and_files = vec![];
        for entry in std::fs::read_dir(path).unwrap() {
          let entry = entry.unwrap();
          let path = entry.path();
          if path.is_dir() {
            subfolders_and_files.push(path.to_str().unwrap().to_string());
          } else {
            subfolders_and_files.push(path.to_str().unwrap().to_string());
          }
        }
        subfolders_and_files
      } else {
        vec![]
      }
    }
    Err(_) => vec![],
  };
  Ok(subfolders_and_files)
}

#[tauri::command]
fn open_folder_in_explorer(path: String) -> Result<(), String> {

  let result = Command::new("explorer")
    .args(["/select,", &path])
    .spawn();

  match result {
    Ok(_) => Ok(()),
    Err(_) => Err("Failed to open folder in explorer".to_string()),
  }
}

#[tauri::command]
fn delete_from_disk(_path: String) -> Result<(), String> {
  let path = Path::new(&_path);
  if path.is_file() {
    fs::remove_file(path).map_err(|e| e.to_string())
  } else {
    fs::remove_dir_all(path).map_err(|e| e.to_string())
  }
}