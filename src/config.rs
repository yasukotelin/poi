use super::utils::path_util;
use dirs;
use std::fmt::Debug;
use std::fs;
use std::path::PathBuf;
use yaml_rust::{Yaml, YamlLoader};

pub struct ConfigLoader {
    path: PathBuf,
}

const YAML_KEY_PROJECTS: &str = "projects";
const YAML_KEY_OTHERS: &str = "others";

impl ConfigLoader {
    pub fn new() -> ConfigLoader {
        return ConfigLoader {
            path: ConfigLoader::get_path().unwrap(),
        };
    }

    fn get_path() -> Option<PathBuf> {
        match dirs::home_dir() {
            Some(mut path) => {
                path.push(".config");
                path.push("poi");
                path.push("poi.yml");
                return Some(path);
            }
            None => return None,
        }
    }

    pub fn load(&self) -> Result<Config, String> {
        let yaml = self.load_yaml()?;

        let projects = self.convert_paths(&yaml, YAML_KEY_PROJECTS);
        let others = self.convert_paths(&yaml, YAML_KEY_OTHERS);

        return Ok(Config {
            projects: projects,
            others: others,
        });
    }

    fn load_yaml(&self) -> Result<Yaml, String> {
        let data = fs::read_to_string(&self.path).map_err(|e| e.to_string())?;
        let yaml = YamlLoader::load_from_str(&data).map_err(|e| e.to_string())?;
        return Ok(yaml[0].clone());
    }

    fn convert_paths(&self, yaml: &Yaml, key: &str) -> Vec<PathBuf> {
        let target = yaml[key].clone();
        if target.is_array() {
            target
                .into_iter()
                .map(|v| path_util::replace_tilde_to_absolute(&PathBuf::from(v.as_str().unwrap())))
                .collect::<Vec<PathBuf>>()
        } else {
            Vec::new()
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub projects: Vec<PathBuf>,
    pub others: Vec<PathBuf>,
}

impl Config {}
