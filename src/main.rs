use clap::Parser;
use std::io;
use std::process::Command;
use std::str::from_utf8;

mod output_color;
use output_color::Colored;

type Directories<'a> = Vec<Option<(&'a str, &'a str)>>;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = 1)]
    deep: u8,
}

fn get_directories() {
    let args = Args::parse();
    let program = "du";
    let deep = args.deep;
    let command = Command::new(program)
        .arg("-h")
        .arg(format!("-d {deep}"))
        .output()
        .expect("WTF")
        .stdout;

    let lines = from_utf8(&command)
        .unwrap()
        .lines()
        .map(|line| line.split_once("\t"))
        .collect::<Directories>();
    dbg!(lines);
}

fn main() -> io::Result<()> {
    let _lines = get_directories();
    // display(&lines);

    Ok(())
}

fn display(lines: &Directories) {
    for line in lines {
        match line {
            Some((size, path)) => println!("{} {}", size.coloring(), path),
            None => (),
        };
    }
}
