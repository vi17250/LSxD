use clap::Parser;
use std::env::current_dir;
use std::io;
use std::path::PathBuf;

mod types;
use crate::types::Directory;

mod commands;
mod display;
mod output_color;

mod compare;

const ROOT_DIR: &str = ".";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    deepth: u8,
    #[arg(short, long, default_value_t = false)]
    sort: bool,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let deepth = args.deepth;
    let sort = args.sort;
    let root_dir: PathBuf = PathBuf::from(ROOT_DIR);
    let mut current_dir: Directory = Directory::new(root_dir.clone());

    current_dir.set_children(deepth, sort);

    current_dir.display(0);

    Ok(())
}
