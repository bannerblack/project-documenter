use app_lib::projects::scan_project_folder;

fn test_get_folder_subdirs(path: &str) {
    let projects = scan_project_folder(path);
    println!("Projects: {:?}", projects);
}

fn main() {
    test_get_folder_subdirs(r"C:\Users\kotsu\Desktop\Production");
}
