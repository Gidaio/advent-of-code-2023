pub mod part1;
pub mod part2;

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub struct State(Vec<String>);

#[derive(Debug, PartialEq)]
pub enum Day1Error {
    NoDigits,
}

impl Display for Day1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::NoDigits => write!(f, "No digits on a line!"),
        }
    }
}

impl Error for Day1Error {}

impl TryFrom<File> for State {
    type Error = io::Error;

    fn try_from(value: File) -> io::Result<Self> {
        let buf = BufReader::new(value);
        let lines = buf.lines().collect::<io::Result<Vec<String>>>()?;
        Ok(State(lines))
    }
}

impl From<&str> for State {
    fn from(value: &str) -> Self {
        State(value.split('\n').map(String::from).collect())
    }
}
