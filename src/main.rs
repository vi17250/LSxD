use clap::Parser;
use std::io;
mod types;
use types::{Directory, File};

mod commands;
use commands::{directories, files};
mod output_color;
use output_color::Colored;
// use types::Lines;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    deepth: u8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let deepth = args.deepth;
    let mut directories: Vec<Directory> = directories::command(deepth);

    directories.iter_mut().for_each(|dir| {
        let files: Vec<File> = files::command(dir.path.clone());
        dir.files = files;
    });

    display(&directories);
    Ok(())
}

fn display(directories: &Vec<Directory>) {
    for directory in directories {
        println!("{} {}", directory.size.coloring(), directory.path);
        for file in &directory.files {
            println!("\t{} {}", file.size.coloring(), file.name);
        }
    }
}
