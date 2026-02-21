use app_lib::projects::get_folder_subdirs;

fn test_get_folder_subdirs(path: &str) {
    let subdirs = get_folder_subdirs(path);
    println!("Subdirectories: {:?}", subdirs);
}

fn main() {
    test_get_folder_subdirs("/Users/bzar/Documents/Production Projects");
}