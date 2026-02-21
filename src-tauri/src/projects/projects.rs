#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub root_folder_path: String,
}

fn get_folder_subdirs(path: &str) -> Vec<String> {
    // convert path to a valid file path
    let path = std::path::PathBuf::from(path);

    std::fs::read_dir(&path)
        .map(|entries| {
            entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| entry.path().is_dir())
                .map(|entry| {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    path.join(file_name).to_string_lossy().to_string()
                })
                .collect()
        })
        .unwrap_or_else(|_| vec![])
}

struct ReadMeParse {
    title: Option<String>,
    description: Option<String>,
    github_link: Option<String>,
}

fn parse_root_readme(_path: &str) -> Option<String> {
    // Get info from ReadMe HTML comment

    
}

#[tauri::command]
pub fn scan_project_folder(_path: &str) -> Vec<Project> {
    let subdirs = get_folder_subdirs(_path);
    vec![]
}

