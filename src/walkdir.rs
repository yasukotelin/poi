use super::hashmap;
use indicatif::ProgressBar;
use indicatif::ProgressStyle;
use lazy_static::lazy_static;
use std::path::Path;
use std::{collections::HashMap, path::PathBuf};
use walkdir::WalkDir;

lazy_static! {
    static ref IGNORES: HashMap<&'static str, i32> =
        hashmap!["node_modules" => 0, "build" => 0, "target" => 0, "out" => 0, "bin" => 0];
}

pub fn walk_projects(dirs: Vec<PathBuf>) -> Vec<PathBuf> {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_chars("/|\\- ")
            .template("[Searching...] {spinner:.dim.bold} {wide_msg}"),
    );
    pb.enable_steady_tick(80);

    let paths = dirs
        .iter()
        .map(|d| walk_project(d))
        .flatten()
        .collect::<Vec<PathBuf>>();

    pb.finish_and_clear();

    paths
}

// Skip when directory is finded ignores or "." directory.
fn is_skip(path: &Path) -> bool {
    if path.is_dir() {
        let file_name = path.file_name().unwrap().to_str().unwrap();
        // check started with "." hidden directory.
        if is_hidden_file(file_name) {
            return false;
        }
        // check ignores
        match IGNORES.get(file_name) {
            Some(_) => return true,
            None => return false,
        };
    }
    return false;
}

fn is_hidden_file(file_name: &str) -> bool {
    file_name.starts_with(".")
}

fn is_git_project(path: &Path) -> bool {
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
        let path = e.path();
        if is_git_project(&path) {
            project_paths.push(path.parent().unwrap().to_path_buf());
            return false;
        }
        if is_skip(&path) {
            return false;
        }

        return true;
    }) {}

    return project_paths;
}
