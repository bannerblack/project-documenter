use std::fs;

#[tauri::command]
pub fn get_readme_content(path: String) -> Result<String, String> {
    // convert path to a valid file path
    let path = std::path::PathBuf::from(path);

    fs::read_to_string(path).map_err(|e| e.to_string())
}