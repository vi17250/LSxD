use crate::types::{Entity, Lines};
use regex::Regex;
use std::process::Command;
use std::str::from_utf8;

pub fn command() -> Lines {
    let program = "ls";
    let command = Command::new(program)
        .arg("-lshan")
        .output()
        .expect("WTF")
        .stdout;

    let lines = from_utf8(&command)
        .expect("WTF")
        .lines()
        .skip(1)
        .filter(|line| !line.contains("4,0K d"))
        .collect::<Vec<&str>>();

    let regex = Regex::new(
        r"^(\d+,?\d*\s*[A-z]{1})\s*(?P<permissions>-[rwx-]{9})\s*(?P<links>\d+)\s*(?P<groups>\d+\s*\d+)\s*(?P<size>\d+,?\d*\s*[A-Z]?)\s*(?P<date>[a-zA-ZÀ-Ÿ-. \d:]*\s*[\d:]{5})\s*(?P<filename>.*)$",
    );

    lines
        .iter()
        .map(|line| {
            let caps = &regex.as_ref().expect("WTF").captures(line).expect("WTF");
            let mut dst = String::new();
            caps.expand("$size<>$filename", &mut dst);
            let filename_offset = dst.find("<>").unwrap_or(dst.len());
            let size: String = dst.drain(..filename_offset).collect();
            dst.remove(0);
            dst.remove(0);

            (Entity::File, size, dst)
        })
        .collect::<Lines>()
}
