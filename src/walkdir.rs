use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

const IGNORES: [&str; 5] = [
    "node_modules",
    "build",
    "target",
    "out",
    "bin"
];

pub fn walk_projects(dirs: Vec<PathBuf>) -> Vec<PathBuf> {
    dirs.iter()
        .map(|d| walk_project(d))
        .flatten()
        .collect::<Vec<PathBuf>>()
}

// Skip when directory is finded ignores or "." directory.
fn is_skip(entry: &DirEntry) -> bool {
    let path = entry.path();
    let file_name = path.file_name().unwrap().to_str().unwrap();
    if path.is_dir() {
        // check started with "." hidden directory.
        if is_hidden_file(file_name) {
            return false;
        }
        // check ignores
        match IGNORES.iter().find(|&&v| v == file_name) {
            Some(_) => return true,
            None => return false,
        };
    }
    return false;
}

fn is_hidden_file(file_name: &str) -> bool {
    file_name.starts_with(".")
}

fn is_git_project(entry: &DirEntry) -> bool {
    let path = entry.path();
    if path.is_dir() && path.file_name().unwrap() == ".git" {
        true
    } else {
        false
    }
}

fn walk_project(dir: &PathBuf) -> Vec<PathBuf> {
    let mut project_paths: Vec<PathBuf> = Vec::new();
    let walker = WalkDir::new(dir).into_iter();
    for _ in walker.filter_entry(|e| {
        if is_git_project(e) {
            project_paths.push(e.path().parent().unwrap().to_path_buf());
            return false;
        }
        if is_skip(e) {
            return false;
        }

        return true;
    }) {}

    return project_paths;
}
