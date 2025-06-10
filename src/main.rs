use std::io::BufRead;
use std::io::{self};
use std::process::Command;

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
        .stdout
        .lines()
        .map(|line| line.unwrap().replace("\t", " "))
        .collect::<Vec<String>>();
    println!("{:?}", command);
    Ok(())
}
