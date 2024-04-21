// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_dialog::DialogExt;
use tauri_plugin_fs::FsExt;

use std::io::{Error as IOError, Read};
use std::{fs::File, io::Write};

fn read_file(file_path: &str) -> Result<Vec<u8>, IOError> {
    let mut file = File::open(file_path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)
}

#[tauri::command]
fn encrypt_file(path: &str, password: &str) {
    if let Ok(file_data) = read_file(path) {
        dbg!(password);
        dbg!(file_data);
        println!("------ file data ^^ ------");
    }
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .setup(|app| {
            #[cfg(debug_assertions)] // only include this code on debug builds
            {
                let w = app.get_webview_window("main").unwrap();
                w.open_devtools();
                w.close_devtools();

                // let scope = app.fs_scope();

                // scope.allow_directory("/path/to/directory", false);
                // dbg!(scope.allowed());
            }
            Ok(())
        })
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![encrypt_file])
        // .invoke_handler(tauri::generate_handler![send_password])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
