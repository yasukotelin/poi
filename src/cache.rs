use dirs;
use std::fs::File;
use std::io::prelude::*;
use std::{fs, path::PathBuf};

pub struct Cache {
    path: PathBuf,
}

impl Cache {
    pub fn new() -> Cache {
        return Cache {
            path: Cache::get_path().unwrap(),
        };
    }

    fn get_path() -> Option<PathBuf> {
        match dirs::home_dir() {
            Some(mut path) => {
                path.push(".config");
                path.push("poipoi");
                path.push(".cache");
                return Some(path);
            }
            None => return None,
        }
    }

    pub fn load(&self) -> std::io::Result<Vec<PathBuf>> {
        let content = fs::read_to_string(self.path.to_str().unwrap())?;
        Ok(content.split("\n").map(|v| PathBuf::from(v)).collect::<_>())
    }

    pub fn write(&self, lines: Vec<PathBuf>) -> std::io::Result<()> {
        let mut f = File::create(self.path.to_str().unwrap())?;
        f.write_all(
            lines
                .iter()
                .map(|v| v.to_str().unwrap())
                .collect::<Vec<&str>>()
                .join("\n")
                .as_bytes(),
        )?;
        Ok(())
    }
}
