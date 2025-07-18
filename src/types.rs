use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct Line {
    pub path: Box<Path>,
    pub size: u64,
    pub children: Option<Vec<Line>>,
}
