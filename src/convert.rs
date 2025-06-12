use colored::{ColoredString, Colorize};

pub trait Colored {
    fn colored(self) -> ColoredString;
}

impl Colored for &str {
    fn colored(self) -> ColoredString {
        let string = match self.to_lowercase() {
            v if v.contains("k") => self.green(),
            v if v.contains("m") => self.yellow(),
            v if v.contains("g") => self.red(),
            v => v.into(),
        };
        string.bold()
    }
}

mod test {

    use crate::convert::Colored;
    use colored::Colorize;

    #[test]
    fn it_returns_value_below_1Mb() {
        assert_eq!("10K".colored(), "10K".green().bold());
    }

    #[test]
    fn it_returns_value_above_1Mb() {
        assert_eq!("10M".colored(), "10M".yellow().bold());
    }

    #[test]
    fn it_returns_value_above_1Gb() {
        assert_eq!("10G".colored(), "10G".red().bold());
    }
}
