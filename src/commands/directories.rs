use crate::types::Directory;
use std::process::Command;
use std::str::from_utf8;

pub fn command(deepth: u8) -> Vec<Directory> {
    let program = "du";
    let command = Command::new(program)
        .arg("-h")
        .arg(format!("-d {deepth}"))
        .output()
        .expect("WTF")
        .stdout;

    let lines = from_utf8(&command)
        .expect("WTF")
        .lines()
        .map(|line| line.split_once("\t"))
        .map(|line| match line {
            Some((size, path)) => Directory {
                path: path.to_string(),
                size: size.to_string(),
                files: vec![],
            },
            None => Directory {
                path: String::new(),
                size: String::new(),
                files: vec![],
            },
        })
        .collect::<Vec<Directory>>();
    lines
}
