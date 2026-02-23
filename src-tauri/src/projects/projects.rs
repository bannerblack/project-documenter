#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct Project {
    pub name: String,
    pub parent: String,
    pub root_folder_path: String,
    pub description: Option<String>,
    pub github_link: Option<String>,
    pub read_me_link: Option<String>,
    pub tags: Vec<String>,
    pub priority: Option<u32>,
}

/// Get all subdirectories of a folder and return their paths as strings
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
    read_me_link: Option<String>,
    tags: Vec<String>,
    priority: Option<u32>,
}

/// Parse the project root readme and return the content and the path to the readme if it exists
fn parse_root_readme(_path: &str) -> Option<(Option<String>, Option<String>)> {
    // Get info from ReadMe HTML comment
    let readme_path = std::fs::read_dir(_path)
        .ok()?
        .filter_map(|entry| entry.ok())
        .find(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .eq_ignore_ascii_case("readme.md")
        })
        .map(|entry| entry.path())?;

    let path_str = readme_path.to_string_lossy().to_string();
    let content = std::fs::read_to_string(&readme_path).ok();

    Some((content, Some(path_str)))
}

fn find_config_in_readme(_path: &str) -> ReadMeParse {
    let (content, read_me_link) = parse_root_readme(_path).unwrap_or((None, None));

    let mut result = ReadMeParse {
        title: None,
        description: None,
        github_link: None,
        read_me_link,
        tags: Vec::new(),
        priority: None,
    };

    for line in content.unwrap_or_default().lines() {
        let trimmed = line.trim();
        if let Some(inner) = trimmed
            .strip_prefix("<!--")
            .and_then(|s| s.strip_suffix("-->"))
        {
            let inner = inner.trim();
            let inner_lower = inner.to_ascii_lowercase();

            let parse_value =
                |prefix_len: usize| -> String { inner[prefix_len..].trim().to_string() };

            if inner_lower.starts_with("title:") {
                result.title = Some(parse_value("title:".len()));
            } else if inner_lower.starts_with("description:") {
                result.description = Some(parse_value("description:".len()));
            } else if inner_lower.starts_with("github:") {
                result.github_link = Some(parse_value("github:".len()));
            } else if inner_lower.starts_with("tags:") {
                result.tags = parse_value("tags:".len())
                    .split(',')
                    .map(|t| t.trim().to_string())
                    .filter(|t| !t.is_empty())
                    .collect();
            } else if inner_lower.starts_with("priority:") {
                result.priority = parse_value("priority:".len()).parse::<u32>().ok();
            }
        }
    }

    result
}

#[tauri::command]
/// Scan multiple folders for subdirectories and return a flat list of projects with info from their readme files
pub fn scan_project_folders(paths: Vec<&str>) -> Vec<Project> {
    let mut projects = Vec::new();

    for parent_path in paths {
        let subdirs = get_folder_subdirs(parent_path);

        for subdir in subdirs {
            let readme_info = find_config_in_readme(&subdir);
            if let Some(title) = readme_info.title {
                projects.push(Project {
                    name: title.clone(),
                    parent: parent_path.to_string(),
                    root_folder_path: subdir,
                    description: readme_info.description,
                    github_link: readme_info.github_link,
                    read_me_link: readme_info.read_me_link,
                    tags: readme_info.tags,
                    priority: readme_info.priority,
                });
            }
        }
    }

    projects
}
