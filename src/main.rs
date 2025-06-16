mod color;
mod parsers;

use clap::Parser;
use std::io;
use std::process::Command;
use std::str::from_utf8;

use color::Colored;
use parsers::files::Parse;

type Directories<'a> = Vec<Option<(&'a str, &'a str)>>;

// see "directories"

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Command recursion
    #[arg(short, long, default_value_t = 1)]
    deep: u8,
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let program = "du";
    let deep = args.deep;
    let command = Command::new(program)
        .arg("-h")
        .arg(format!("-d {deep}"))
        .output()?
        .stdout;

    let directories = from_utf8(&command)
        .unwrap()
        .lines()
        .map(|line| line.split_once("\t"))
        .collect::<Directories>();

    let program = "ls";
    let command = Command::new(program)
        .arg("-n")
        .arg("-h")
        .arg("-a")
        .output()?
        .stdout;

    let files = from_utf8(&command)
        .unwrap()
        .lines()
        .skip(1)
        .filter(|line| !line.contains("4,0K d"))
        .map(|line| line.parse_files())
        .for_each(|line| {
            let r = line.as_str().split_once(" ");
            println!("R: {:?}", r);
        });
    dbg!(files);

    display(&directories);

    Ok(())
}

fn display(lines: &Directories) {
    for line in lines {
        match line {
            Some((size, path)) => println!("{} {}", size.colored(), path),
            None => (),
        };
    }
}
