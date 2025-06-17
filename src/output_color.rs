use colored::{ColoredString, Colorize};

pub trait Colored {
    fn coloring(&self) -> ColoredString;
}

impl Colored for String {
    fn coloring(&self) -> ColoredString {
        let string = match self.to_lowercase() {
            v if v.contains("k") => self.green(),
            v if v.contains("m") => self.yellow(),
            v if v.contains("g") => self.red(),
            v => v.into(),
        };
        string.bold()
    }
}

#[allow(warnings)]
mod test {

    use crate::output_color::Colored;
    use colored::Colorize;

    #[test]
    fn it_returns_value_below_1Mb() {
        assert_eq!("10K".to_string().coloring(), "10K".green().bold());
    }

    #[test]
    fn it_returns_value_above_1Mb() {
        assert_eq!("10M".to_string().coloring(), "10M".yellow().bold());
    }

    #[test]
    fn it_returns_value_above_1Gb() {
        assert_eq!("10G".to_string().coloring(), "10G".red().bold());
    }
}
