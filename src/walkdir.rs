use std::path::PathBuf;
use walkdir::DirEntry;
use walkdir::WalkDir;

const IGNORES: [&str; 6] = [
    "node_modules",
    "build",
    "out",
    ".idea",
    ".vscode",
    ".gradle",
];

pub fn walk_projects(dirs: Vec<PathBuf>) -> Vec<PathBuf> {
    dirs.iter()
        .map(|d| walk_project(d))
        .flatten()
        .collect::<Vec<PathBuf>>()
}

fn is_skip(entry: &DirEntry) -> bool {
    let path = entry.path();
    let file_name = path.file_name().unwrap();
    if path.is_dir() {
        match IGNORES.iter().find(|&&v| v == file_name) {
            Some(_) => return true,
            None => return false,
        };
    }
    return false;
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
        if is_skip(e) {
            return false;
        }
        if is_git_project(e) {
            project_paths.push(e.path().parent().unwrap().to_path_buf());
            return false;
        }
        return true;
    }) {}

    return project_paths;
}
