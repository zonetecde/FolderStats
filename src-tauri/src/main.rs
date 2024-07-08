// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![get_folder_size, get_subfolders_and_files])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn get_folder_size(path: String) -> Result<u64, String> {
  let path = std::path::Path::new(&path);
  let size = match std::fs::metadata(path) {
    Ok(metadata) => {
      if metadata.is_file() {
        metadata.len()
      } else {
        let mut size = 0;
        for entry in std::fs::read_dir(path).unwrap() {
          let entry = entry.unwrap();
          let path = entry.path();
          if path.is_dir() {
            size += get_folder_size(path.to_str().unwrap().to_string()).unwrap();
          } else {
            size += std::fs::metadata(path).unwrap().len();
          }
        }
        size
      }
    }
    Err(_) => 0,
  };
  Ok(size)
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
