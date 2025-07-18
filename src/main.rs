use clap::Parser;
use std::{io, path::Path};
mod types;
use types::Line;

mod commands;
use commands::lines;
mod output_color;

const ROOT_DIR: &str = ".";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    deepth: u8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let deepth = args.deepth;
    let root_dir: Box<Path> = Path::new(ROOT_DIR).into();
    let mut directories: Vec<Line> = lines::command(root_dir);
    for directory in directories.iter_mut() {
        if let Some(_) = &directory.children {
            let lines = lines::command(directory.path.clone());
            directory.add(lines);
        }
    }

    dbg!(&directories);

    Ok(())
}
