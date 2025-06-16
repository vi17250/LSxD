use regex::Regex;

pub trait Parse {
    fn parse_files(&self) -> String;
}

impl Parse for &str {
    fn parse_files(&self) -> String {
        let re = Regex::new(r"^([drwx-]{10})\s*(\d+)\s*(\d+\s*\d+)\s*(?P<size>\d*,?\d*\w?)\s*(\d{1,2}\s*([a-zA-ZÀ-Ÿ-. ]*)\s*\d{2}:?\d{2})\s*(?P<file>.*)").unwrap();
        let caps = re.captures(self).unwrap();
        let mut dst = String::new();
        caps.expand("$size $file", &mut dst);
        dst
    }
}

mod test {
    use crate::*;

    #[test]
    fn it_returns_size_in_bytes_and_filename() {
        let input = "-rw-------  1 1000 1000   64  4 déc.   2024 .histfile";
        assert_eq!(input.parse_files(), String::from("64 .histfile"));
    }

    #[test]
    fn it_returns_size_in_kb_and_filename() {
        let input = "-rw-------  1 1000 1000 64,7K  4 déc.   2024 .histfile";
        let t = input.parse_files();
        assert_eq!(input.parse_files(), String::from("64,7K .histfile"));
    }

    #[test]
    fn it_returns_size_in_kb_and_filename_with_spaces() {
        let input = "-rw-------  1 1000 1000 6,7K  4 déc.   2024 my file.txt";
        let t = input.parse_files();
        assert_eq!(input.parse_files(), String::from("6,7K my file.txt"));
    }
}
