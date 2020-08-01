use clap::App;
use clap::Arg;
use poipoi::config::ConfigLoader;
use poipoi::skim;
use poipoi::walkdir;
use std::process;

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
            Arg::with_name("noskim")
                .long("noskim")
                .help("sets no using skim(fuzzy finder)")
                .takes_value(false)
                .required(false),
        )
        .get_matches();

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

    let is_use_skim = !matches.is_present("noskim");
    if is_use_skim {
        let color = matches.value_of("color");
        let options = skim::Options { color: color };
        if let Some(selected_path) = skim::select_path(paths, &options) {
            println!("{}", selected_path.to_str().unwrap());
        }
    } else {
        // Displays paths to standard output.
        for path in paths.iter() {
            println!("{}", path.display());
        }
    }
}
