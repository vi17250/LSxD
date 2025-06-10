use std::io::BufRead;
use std::io::{self, Write};
use std::process::{Command, Output};
use std::str::from_utf8;

use clap::Parser;

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

    let lines = from_utf8(&command)
        .unwrap()
        .lines()
        .map(|line| line.split_once("\t"))
        .collect::<Vec<Option<(&str, &str)>>>();

    println!("{:?}", lines);
    Ok(())
}
