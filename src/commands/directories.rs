use std::process::Command;
use std::str::from_utf8;

type Lines = Vec<(String, String)>;

pub fn command(deepth: u8) -> Lines {
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
            Some((size, file)) => (size.to_string(), file.to_string()),
            None => (String::from(""), String::from("")),
        })
        .collect::<Lines>();
    lines
}
