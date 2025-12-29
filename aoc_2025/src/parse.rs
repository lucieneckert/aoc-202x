use std::io::{Result, BufReader};
use std::fs::File;

pub(crate) fn get_input(path: String) -> Result<BufReader<File>> {
    let input = File::open(path)?;
    let reader = BufReader::new(input);
    Ok(reader)
}

pub(crate) enum Part {
    Part1,
    Part2
}

impl From<String> for Part {
    fn from(s: String) -> Part {
        match s.as_str() {
            "1" => Part::Part1,
            "2" => Part::Part2,
            _ => panic!("invalid part received")
        }
    }
}