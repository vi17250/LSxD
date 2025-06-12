use colored::{ColoredString, Colorize};

pub fn convert(size: &str) -> ColoredString {
    match size.to_lowercase() {
        v if v.contains("k") => size.green(),
        v if v.contains("m") => size.yellow(),
        v if v.contains("g") => size.red(),
        v => v.into(),
    }
}

mod test {

    use crate::convert::convert;
    use colored::Colorize;

    #[test]
    fn it_returns_value_below_1Mb() {
        assert_eq!(convert("10K"), "10K".green());
    }

    #[test]
    fn it_returns_value_above_1Mb() {
        assert_eq!(convert("10M"), "10M".yellow());
    }

    #[test]
    fn it_returns_value_above_1Gb() {
        assert_eq!(convert("10G"), "10G".red());
    }
}
