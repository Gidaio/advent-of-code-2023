pub mod part1;
pub mod part2;

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Puzzle {
    times: Vec<isize>,
    distances: Vec<isize>,
}

#[derive(Debug)]
pub enum Day6Error {
    NoTimes,
    NoDistances,
    ParseIntError(ParseIntError),
    IoError(IoError),
}

impl TryFrom<File> for Puzzle {
    type Error = Day6Error;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let buf = BufReader::new(value);
        let mut lines = buf.lines();

        let times = lines
            .next()
            .ok_or(Day6Error::NoTimes)??
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let distances = lines
            .next()
            .ok_or(Day6Error::NoDistances)??
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Puzzle { times, distances })
    }
}

impl TryFrom<&str> for Puzzle {
    type Error = Day6Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines();

        let times = lines
            .next()
            .ok_or(Day6Error::NoTimes)?
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse())
            .collect::<Result<Vec<_>, _>>()?;

        let distances = lines
            .next()
            .ok_or(Day6Error::NoDistances)?
            .split_whitespace()
            .skip(1)
            .map(|num| num.parse())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Puzzle { times, distances })
    }
}

impl Display for Day6Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoTimes => write!(f, "No line for times!"),
            Self::NoDistances => write!(f, "No line for distances!"),
            Self::ParseIntError(err) => err.fmt(f),
            Self::IoError(err) => err.fmt(f),
        }
    }
}

impl Error for Day6Error {}

impl From<ParseIntError> for Day6Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl From<IoError> for Day6Error {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;

    #[test]
    fn parse_provided_example() {
        let expected = Puzzle {
            times: vec![7, 15, 30],
            distances: vec![9, 40, 200],
        };
        let input = "Time:      7  15   30\nDistance:  9  40  200\n";
        let puzzle: Puzzle = input.try_into().unwrap();
        assert_eq!(puzzle, expected);
    }
}
