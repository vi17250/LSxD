use fs_extra::dir::get_size;
use human_bytes::human_bytes;
use std::path::PathBuf;
use std::process::Command;
use std::str::from_utf8;

pub fn file(path: PathBuf) -> (usize, String) {
    let size = get_size(path)
        .expect("Failed to get size")
        .try_into()
        .expect("Failed to parse size");
    (size, human_bytes(size as f64))
}

pub fn directory(path: PathBuf) -> (usize, String) {
    let program = "du";
    let command = Command::new(program)
        .arg(path)
        .arg("-d 0")
        .arg("--block-size=1")
        .output()
        .expect("WTF")
        .stdout;

    let line = from_utf8(&command)
        .expect("Failed to read command")
        .split_once("\t")
        .expect("Failes to read size");

    let size = line.0.parse::<usize>().expect("Failed to parse size");
    (size, human_bytes(size as f64))
}
