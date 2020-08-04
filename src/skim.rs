use skim::prelude::Skim;
use skim::prelude::SkimItemReader;
use skim::prelude::SkimOptionsBuilder;
use std::io::Cursor;
use std::path::PathBuf;

pub struct Options<'a> {
    pub color: Option<&'a str>,
    pub height: Option<&'a str>,
}

pub fn select_path(paths: Vec<PathBuf>, options: &Options) -> Option<PathBuf> {
    let options = SkimOptionsBuilder::default()
        .height(options.height)
        .color(options.color)
        .multi(false)
        .build()
        .unwrap();

    let input = paths
        .iter()
        .map(|p| p.to_str().unwrap())
        .collect::<Vec<_>>()
        .join("\n");

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(input));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)?;

    for item in selected_items.iter() {
        return Some(PathBuf::from(item.output().into_owned()));
    }

    return None;
}
