pub mod part1;
pub mod part2;

use std::collections::HashSet;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use std::num::ParseIntError;

#[derive(Debug)]
pub struct Puzzle(Vec<Card>);

#[derive(Debug, PartialEq)]
struct Card {
    id: usize,
    winning_numbers: HashSet<usize>,
    scratched_numbers: HashSet<usize>,
}

#[derive(Debug)]
pub enum Day4Error {
    NoColonSeparator(String),
    NoPipe(String),
    ParseIntError(ParseIntError),
    IoError(IoError),
}

impl TryFrom<File> for Puzzle {
    type Error = Day4Error;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let value = BufReader::new(value);
        Ok(Puzzle(
            value
                .lines()
                .map(|line| (&line?[..]).try_into())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl TryFrom<&str> for Puzzle {
    type Error = Day4Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(Puzzle(
            value
                .lines()
                .map(|line| line.try_into())
                .collect::<Result<Vec<_>, _>>()?,
        ))
    }
}

impl TryFrom<&str> for Card {
    type Error = Day4Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (card_id, numbers) = value
            .split_once(": ")
            .ok_or(Day4Error::NoColonSeparator(String::from(value)))?;
        let card_id: usize = card_id[5..].trim().parse()?;

        let (winning_numbers, scratched_numbers) = numbers
            .split_once(" | ")
            .ok_or(Day4Error::NoPipe(String::from(value)))?;
        let winning_numbers = winning_numbers
            .split(' ')
            .filter_map(|number| {
                if number.is_empty() {
                    None
                } else {
                    Some(number.parse::<usize>())
                }
            })
            .collect::<Result<HashSet<_>, _>>()?;
        let scratched_numbers = scratched_numbers
            .split(' ')
            .filter_map(|number| {
                if number.is_empty() {
                    None
                } else {
                    Some(number.parse::<usize>())
                }
            })
            .collect::<Result<HashSet<_>, _>>()?;

        Ok(Card {
            id: card_id,
            winning_numbers,
            scratched_numbers,
        })
    }
}

impl Display for Day4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoColonSeparator(line) => write!(f, "No colon separator on line \"{}\"", line),
            Self::NoPipe(line) => write!(f, "No pipe separator on line \"{}\"", line),
            Self::ParseIntError(err) => err.fmt(f),
            Self::IoError(err) => err.fmt(f),
        }
    }
}

impl Error for Day4Error {}

impl From<ParseIntError> for Day4Error {
    fn from(value: ParseIntError) -> Self {
        Day4Error::ParseIntError(value)
    }
}

impl From<IoError> for Day4Error {
    fn from(value: IoError) -> Self {
        Day4Error::IoError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Day4Error};
    use std::collections::HashSet;

    #[test]
    fn parses_a_card() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected = Card {
            id: 1,
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            scratched_numbers: HashSet::from([83, 86, 6, 31, 17, 9, 48, 53]),
        };

        let result: Result<Card, Day4Error> = input.try_into();

        match result {
            Ok(card) => assert_eq!(card, expected),
            Err(err) => panic!("Got error {}", err),
        }
    }
}
