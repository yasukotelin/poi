use std::path::PathBuf;

/// Converts an absolute path if it starts with a tilde(~).
pub fn replace_tilde_to_absolute(path: &PathBuf) -> PathBuf {
    if path.starts_with("~") {
        let mut home_dir = dirs::home_dir().unwrap();
        let path = path.strip_prefix("~").unwrap();
        home_dir.push(path);
        return home_dir;
    } else {
        return path.clone();
    }
}
