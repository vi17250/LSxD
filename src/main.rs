use std::io::BufRead;
use std::io::{self};
use std::process::Command;

fn main() -> io::Result<()> {
    let program = "du";
    let deep = 1;
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
