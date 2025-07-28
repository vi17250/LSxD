use clap::Parser;
use std::io;
use std::path::PathBuf;

mod types;
use crate::types::Directory;

mod commands;
use crate::commands::get_dirs;

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
    let root_dir: PathBuf = PathBuf::from(ROOT_DIR);

    let mut current_dir: Directory = Directory::new(root_dir.clone());
    current_dir.get_children(deepth);
    Ok(())
}
