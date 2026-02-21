use std::process::Command;

mod files;
pub mod projects;

#[tauri::command]
fn open_in_vscode(path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .arg("-a")
            .arg("Visual Studio Code")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open VS Code: {}", e))?;
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("code")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("Failed to open VS Code: {}", e))?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            files::get_readme_content,
            open_in_vscode,
            projects::scan_project_folder
        ])
        .setup(|app| {
            if cfg!(debug_assertions) {
                app.handle().plugin(
                    tauri_plugin_log::Builder::default()
                        .level(log::LevelFilter::Info)
                        .build(),
                )?;
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
