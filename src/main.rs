use clap::Parser;
use std::io;

mod commands;
use commands::get_directories;
mod output_color;
use output_color::Colored;

type Directories = Vec<(String, String)>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    deepth: u8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let deepth = args.deepth;
    let lines = get_directories(deepth);
    display(&lines);

    Ok(())
}

fn display(lines: &Directories) {
    for (size, path) in lines {
        println!("{} {}", size.coloring(), path);
    }
}
