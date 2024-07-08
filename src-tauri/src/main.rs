// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, DirEntry};
use std::io;
use std::path::Path;
use rayon::prelude::*;
use std::process::Command;

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_folder_size, get_subfolders_and_files, open_folder_in_explorer])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}


#[tauri::command]
fn get_folder_size(path: String) -> Result<u64, String> {
    let path = Path::new(&path);
    calculate_size(path).map_err(|e| e.to_string())
}

fn calculate_size(path: &Path) -> io::Result<u64> {
    if path.is_file() {
        return Ok(path.metadata()?.len());
    }

    let entries: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .collect();

    let size: u64 = entries.par_iter()
        .map(|entry| entry_size(entry))
        .sum();

    Ok(size)
}

fn entry_size(entry: &DirEntry) -> u64 {
    let path = entry.path();
    if path.is_dir() {
        calculate_size(&path).unwrap_or(0)
    } else {
        entry.metadata().map(|m| m.len()).unwrap_or(0)
    }
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
