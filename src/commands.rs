use std::process::Command;
use std::str::from_utf8;

type Directories = Vec<(String, String)>;

pub fn get_directories(deepth: u8) -> Directories {
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
        .collect::<Directories>();
    lines
}
