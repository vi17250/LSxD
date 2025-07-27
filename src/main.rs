use clap::Parser;
use std::fs::read_dir;
use std::io;
use std::path::PathBuf;

mod types;
use crate::types::Directory;

mod output_color;

const ROOT_DIR: &str = ".";

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 0)]
    deepth: i8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let deepth = args.deepth;
    let root_dir: PathBuf = PathBuf::from(ROOT_DIR);

    let mut current_dir = Directory {
        path: root_dir.clone(),
        children: get_dirs(root_dir),
    };
    current_dir.get_children(deepth);
    dbg!(current_dir);
    Ok(())
}

pub fn get_dirs(root_dir: PathBuf) -> Vec<Directory> {
    let mut dirs: Vec<Directory> = vec![];
    if let Ok(entries) = read_dir(root_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let path = entry.path();
                        let children: Vec<Directory> = Vec::new();
                        dirs.push(Directory { path, children });
                    }
                } else {
                    println!("Couldn't get file type for {:?}", entry.path());
                }
            }
        }
    }
    dirs
}
