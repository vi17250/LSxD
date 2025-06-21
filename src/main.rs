use clap::Parser;
use std::io;

mod commands;
use commands::{directories, files};
mod output_color;
use output_color::Colored;
mod types;
use types::Lines;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    deepth: u8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let deepth = args.deepth;
    let mut directories = directories::command(deepth);

    let files = files::command();
    directories.extend(files);
    display(&directories);
    Ok(())
}

fn display(lines: &Lines) {
    for (_entity, size, path) in lines {
        println!("{} {}", size.coloring(), path);
    }
}
