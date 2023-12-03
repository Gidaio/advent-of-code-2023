pub mod part1;
pub mod part2;
#[cfg(test)]
mod tests;

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct PuzzleState(Vec<Game>);

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    pulls: Vec<Pull>,
}

impl TryFrom<File> for PuzzleState {
    type Error = Day2Error;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let buf = BufReader::new(value);
        let games = buf
            .lines()
            .map(|line| line.map_err(|err| Day2Error::IoError(err))?.try_into())
            .collect::<Result<Vec<Game>, Day2Error>>()?;

        Ok(PuzzleState(games))
    }
}

impl TryFrom<&str> for PuzzleState {
    type Error = Day2Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let games = value
            .lines()
            .map(|line| String::from(line).try_into())
            .collect::<Result<Vec<Game>, Day2Error>>()?;

        Ok(PuzzleState(games))
    }
}

impl TryFrom<String> for Game {
    type Error = Day2Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let mut parts = value.split(": ");
        let game_id = parts.next().ok_or(Day2Error::NoGameID(value.clone()))?;

        let id: usize = (&game_id[5..])
            .parse()
            .map_err(|_| Day2Error::NoGameID(value.clone()))?;

        let pulls = parts.next().ok_or(Day2Error::NoPulls(value.clone()))?;
        let pulls = pulls
            .split("; ")
            .map(|pull| pull.try_into())
            .collect::<Result<Vec<Pull>, Day2Error>>()?;

        Ok(Game { id, pulls })
    }
}

#[derive(Debug, PartialEq)]
struct Pull {
    red: usize,
    green: usize,
    blue: usize,
}

// This is parsing the "3 blue, 4 red" part.
impl TryFrom<&str> for Pull {
    type Error = Day2Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut red: usize = 0;
        let mut green: usize = 0;
        let mut blue: usize = 0;

        value
            .split(", ")
            .map(|cube| -> Result<(), Day2Error> {
                let mut parts = cube.split(" ");
                let count = parts.next().ok_or(Day2Error::NoCount(String::from(cube)))?;
                let color = parts.next().ok_or(Day2Error::NoColor(String::from(cube)))?;

                let count: usize = count
                    .parse()
                    .map_err(|_| Day2Error::NotANumber(String::from(cube)))?;
                if color.contains("red") {
                    red = count;
                } else if color.contains("green") {
                    green = count;
                } else if color.contains("blue") {
                    blue = count;
                } else {
                    return Err(Day2Error::UnrecognizedColor(String::from(cube)));
                }

                Ok(())
            })
            .collect::<Result<Vec<()>, Day2Error>>()?;

        Ok(Self { red, green, blue })
    }
}

#[derive(Debug)]
pub enum Day2Error {
    NoCount(String),
    NoColor(String),
    NotANumber(String),
    UnrecognizedColor(String),

    NoGameID(String),
    NoPulls(String),

    IoError(io::Error),
}

impl Display for Day2Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoCount(line) => write!(f, "No count for cube {}", line),
            Self::NoColor(line) => write!(f, "No color for cube {}.", line),
            Self::NotANumber(line) => write!(f, "Count isn't a number for cube {}", line),
            Self::UnrecognizedColor(line) => write!(f, "Unrecognized color for cube {}.", line),

            Self::NoGameID(line) => write!(f, "No game ID on line {}.", line),
            Self::NoPulls(line) => write!(f, "No pulls on line {}.", line),

            Self::IoError(err) => write!(f, "{}", err),
        }
    }
}

impl<'a> Error for Day2Error {}
