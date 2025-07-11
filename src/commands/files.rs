use crate::types::File;
use regex::Regex;
use std::process::Command;
use std::str::from_utf8;

pub fn command(directory: String) -> Vec<File> {
    let program = "ls";
    let command = Command::new(program)
        .arg(directory)
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
    parse_lines(&lines)
}

fn parse_lines(lines: &Vec<&str>) -> Vec<File> {
    let regex = Regex::new(
        r"^\s*(?P<link_size>\d+,?\d*\s*[A-Z]{0,1})\s*(?P<permissions>-[rwx-]{9})\s*(?P<links>\d+)\s*(?P<groups>\d+\s*\d+)\s*(?P<size>\d+,?\d*\s*[A-Z]?)\s*(?P<date>[a-zA-ZÀ-Ÿ-. \d:]*\s*[\d:]{4,5})\s*(?P<filename>.*)$",
    );
    let res = lines
        .iter()
        .map(|line| {
            let caps = &regex.as_ref().expect("WTF").captures(line).expect("WTF");
            let mut dst = String::new();
            caps.expand("$size<>$filename", &mut dst);
            let filename_offset = dst.find("<>").unwrap_or(dst.len());
            let mut size: String = dst.drain(..filename_offset).collect();
            dst.remove(0);
            dst.remove(0);
            size.retain(|c| c != ' ');
            dst.retain(|c| c != '\'');
            File { size, name: dst }
            // (size, dst);
        })
        .collect::<Vec<File>>();

    res
}

#[allow(warnings)]
mod test {

    use crate::{files::parse_lines, types::File};

    #[test]
    fn it_returns_value_with_size_in_byte() {
        let input = vec!["4,0K -rw-r--r-- 1 1000 1000 58 8 déc. 2024 .gitconfig"];
        assert_eq!(
            parse_lines(&input),
            vec![File {
                size: String::from("58"),
                name: String::from(".gitconfig")
            }]
        );
    }

    #[test]
    fn it_returns_value_with_size_in_Kbyte() {
        let input = vec!["4,0K -rw-r--r-- 1 1000 1000  5,88K 8 déc. 2024 .gitconfig"];
        assert_eq!(
            parse_lines(&input),
            vec![File {
                size: String::from("5,88K"),
                name: String::from(".gitconfig")
            }]
        );
    }

    #[test]
    fn it_returns_value_with_size_in_Mbyte() {
        let input = vec!["4,0K -rw-r--r--  1 1000 1000 5M  8 déc.  2024 .gitconfig"];
        assert_eq!(
            parse_lines(&input),
            vec![File {
                size: String::from("5M"),
                name: String::from(".gitconfig")
            }]
        );
    }

    #[test]
    fn it_returns_value_with_size_in_Gbyte() {
        let input = vec!["4,0K -rw-r--r--  1 1000 1000   5,54G 8 déc.  2024 a_big_file.json"];
        assert_eq!(
            parse_lines(&input),
            vec![File {
                size: String::from("5,54G"),
                name: String::from("a_big_file.json")
            }]
        );
    }

    #[test]
    fn it_returns_value_with_date_and_hour() {
        let input = vec!["4,0K -rw-r--r-- 1 1000 1000 3,2K 21 juin  09:40 CHANGELOG.md"];
        assert_eq!(
            parse_lines(&input),
            vec![File {
                size: String::from("3,2K"),
                name: String::from("CHANGELOG.md")
            }]
        );
    }

    #[test]
    fn it_returns_value_with_space_on_filename() {
        let input = vec![" 76K -rw-r--r--  1 1000 1000 73K 12 mai  15:40 'A4 - 8.pdf'"];
        assert_eq!(
            parse_lines(&input),
            vec![File {
                size: String::from("73K"),
                name: String::from("A4 - 8.pdf")
            }]
        );
    }

    #[test]
    fn it_bugs() {
        let input = vec!["0 -rw-r--r--  1 1000 1000 0 10 juil.  10:27 '.cargo-lock'"];
        assert_eq!(
            parse_lines(&input),
            vec![File {
                size: String::from("0"),
                name: String::from(".cargo-lock")
            }]
        );
    }
}
