use clap::App;
use clap::{Arg, ArgMatches, SubCommand};
use poipoi::config::ConfigLoader;
use poipoi::skim;
use poipoi::{cache::Cache, walkdir};
use std::{io::Error, path::PathBuf, process};

fn main() {
    let matches = App::new(clap::crate_name!())
        .version(clap::crate_version!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .arg(
            Arg::with_name("color")
                .long("color")
                .takes_value(true)
                .help("sets skim(fuzzy finder) color")
                .required(false),
        )
        .arg(
            Arg::with_name("height")
                .long("height")
                .takes_value(true)
                .default_value("30%")
                .help("sets skim(fuzzy finder) height")
                .required(false),
        )
        .arg(
            Arg::with_name("noskim")
                .long("noskim")
                .help("sets no using skim(fuzzy finder)")
                .takes_value(false)
                .required(false),
        )
        .subcommand(
            SubCommand::with_name("cache")
                .about("Cache the result that search dirs for fast launch."),
        )
        .get_matches();

    if let Some(_) = matches.subcommand_matches("cache") {
        match execute_poipoi_cache() {
            Ok(_) => println!("Success write cache file."),
            Err(e) => eprintln!("Failed to write cache file: {}", e),
        }
    } else {
        execute_poipoi(&matches);
    }
}

fn execute_poipoi(matches: &ArgMatches) {
    let project_paths = scan_projects_paths_with_cache();

    let is_use_skim = !matches.is_present("noskim");
    if is_use_skim {
        display_with_skim(&matches, project_paths);
    } else {
        display_with_println(project_paths);
    }
}

fn execute_poipoi_cache() -> Result<(), Error> {
    let project_paths = scan_projects_paths();
    Cache::new().write(project_paths)?;
    Ok(())
}

fn scan_projects_paths() -> Vec<PathBuf> {
    let config = ConfigLoader::new().load().unwrap_or_else(|e| {
        eprintln!("failed to load config file: {}", e);
        eprintln!("should exist in $HOME_DIR/.config/poipoi/poipoi.yml.");
        process::exit(1);
    });

    let mut project_paths = walkdir::walk_projects(config.projects);
    let mut other_paths = config.others;

    let mut paths = Vec::new();
    paths.append(&mut project_paths);
    paths.append(&mut other_paths);

    paths
}

fn scan_projects_paths_with_cache() -> Vec<PathBuf> {
    match Cache::new().load() {
        Ok(paths) => paths,
        Err(_) => scan_projects_paths(),
    }
}

fn display_with_skim(matches: &ArgMatches, project_paths: Vec<PathBuf>) {
    let options = skim::Options {
        color: matches.value_of("color"),
        height: matches.value_of("height"),
    };
    if let Some(selected_path) = skim::select_path(project_paths, &options) {
        println!("{}", selected_path.to_str().unwrap());
    }
}

fn display_with_println(project_paths: Vec<PathBuf>) {
    for path in project_paths.iter() {
        println!("{}", path.display());
    }
}
