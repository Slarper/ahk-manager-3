// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{sync::Mutex, process::{Child, Command}, path::Path, collections::HashMap};

use tauri::{State, Manager};
use walkdir::WalkDir;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    tauri::Builder::default()
        .on_window_event(|event|{
            match event.event() {
                tauri::WindowEvent::CloseRequested {..} => {
                    //阻止默认关闭
                    //api.prevent_close();

                    

                     let window = event.window().clone();

                     let handler_map = window.state::<ScriptHandlerMap>();
                    clear_script_handlers(handler_map).unwrap();
                    // window.close();
                }
                _ => {} //todo
            }
        })
        .manage(ScriptHandlerMap(Default::default()))
        .invoke_handler(tauri::generate_handler![
            greet, 
            scan_folder, 
            run_script, 
            shutdowm_script,
            vscode_script
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


struct ScriptHandlerMap(Mutex<HashMap<String, Child>>);

#[tauri::command]
fn scan_folder(folder_path: String) -> Vec<String> {
    let path = Path::new(&folder_path);

    let mut scripts = Vec::new();

    // Use WalkDir to iterate through the directory and its subdirectories
    for entry in WalkDir::new(path).max_depth(1).into_iter().filter_map(|e| e.ok()) {
        // Check if the entry is a file and ends with ".ahk"
        if entry.file_type().is_file() {
            if let Some(extension) = entry.path().extension() {
                if extension == "ahk" {
                    if let Some(file_path) = entry.path().to_str() {
                        scripts.push(file_path.to_string())

                    }
                }
            }
        }
    }

    println!("Scan Foler");
    // self.instances = ahk_files;
    scripts

    

    
}


#[tauri::command]
fn run_script(file_path: String, handler_map: State<ScriptHandlerMap>) {
    let mut handlers = handler_map.0.lock().unwrap();
    let child: Child = Command::new("D:/Program Files/AutoHotkey/AutoHotkeyU64.exe")
        .arg(&file_path)
        .spawn()
        .expect("Failed to start the process");
    handlers.insert(file_path, child);
    // handlers.push(ScriptHandler{file_path:file_path, process:child})
}


#[tauri::command]
fn shutdowm_script(file_path: String, handler_map: State<ScriptHandlerMap>) -> Result<(), String> {
    let mut handlers = handler_map.0.lock().unwrap();
    if let Some(mut value) = handlers.remove(&file_path) {
        let r = value.kill();

        if let Err(e) = r {
            return Err(e.to_string())
        }
    }
    Ok(())
}

fn clear_script_handlers(handler_map: State<ScriptHandlerMap>) -> Result<(), String> {
    let mut handlers = handler_map.0.lock().unwrap();
    for child in handlers.values_mut(){
        let r = child.kill();

        if let Err(e) = r {
            return Err(e.to_string())
        }
    } // Assuming `clear` is a method to reset or clear your map

    handlers.clear();
    println!("Script handlers cleared.");
    Ok(())
}
#[tauri::command]
fn vscode_script(file_path: String) {
    Command::new("D:\\Microsoft VS Code\\Code.exe")
        .arg(&file_path)
        .spawn()
        .expect(&("Failed to start the process".to_owned() + &file_path));
}