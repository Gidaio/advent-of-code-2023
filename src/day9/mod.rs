pub mod part1;
pub mod part2;

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Puzzle(Vec<Vec<isize>>);

#[derive(Debug)]
pub enum Day9Error {
    ParseIntError(ParseIntError),
    IoError(IoError),
}

impl TryFrom<File> for Puzzle {
    type Error = Day9Error;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let buf = BufReader::new(value);
        Ok(Puzzle(
            buf.lines()
                .map(|line| {
                    line?
                        .split_whitespace()
                        .map(|num| num.parse::<isize>().map_err(Day9Error::ParseIntError))
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl TryFrom<&str> for Puzzle {
    type Error = Day9Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Puzzle(
            value
                .lines()
                .map(|line| {
                    line.split_whitespace()
                        .map(|num| num.parse::<isize>().map_err(Day9Error::ParseIntError))
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

impl Display for Day9Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ParseIntError(err) => err.fmt(f),
            Self::IoError(err) => err.fmt(f),
        }
    }
}

impl Error for Day9Error {}

impl From<IoError> for Day9Error {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::Puzzle;

    #[test]
    fn example() {
        let input = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45\n";
        let expected = Puzzle(vec![
            vec![0, 3, 6, 9, 12, 15],
            vec![1, 3, 6, 10, 15, 21],
            vec![10, 13, 16, 21, 30, 45],
        ]);

        let result: Puzzle = input.try_into().unwrap();
        assert_eq!(result, expected);
    }
}
