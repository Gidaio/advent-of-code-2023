pub mod part1;
pub mod part2;

use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{Error as IoError, Read};

#[derive(Debug, PartialEq)]
pub struct Puzzle {
    path: Vec<LR>,
    nodes: HashMap<String, Node>,
}

#[derive(Debug, PartialEq)]
enum LR {
    Left,
    Right,
}

#[derive(Debug, PartialEq)]
struct Node {
    left: String,
    right: String,
}

#[derive(Debug)]
pub enum Day8Error {
    Input,
    Direction(char),
    Node(String),
    Paths(String),
    MissingNode(String),
    IoError(IoError),
}

impl TryFrom<File> for Puzzle {
    type Error = Day8Error;

    fn try_from(mut value: File) -> Result<Self, Self::Error> {
        let mut buf = String::new();
        value.read_to_string(&mut buf)?;
        (&buf[..]).try_into()
    }
}

impl TryFrom<&str> for Puzzle {
    type Error = Day8Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (path, nodes) = value.trim().split_once("\n\n").ok_or(Day8Error::Input)?;

        let path = path
            .trim()
            .chars()
            .map(|char| char.try_into())
            .collect::<Result<Vec<LR>, _>>()?;

        let nodes = nodes
            .split('\n')
            .map(|line| {
                let (name, directions) = line
                    .split_once(" = ")
                    .ok_or(Day8Error::Node(String::from(line)))?;

                let (left, right) = directions
                    .split_once(", ")
                    .ok_or(Day8Error::Paths(String::from(directions)))?;

                Ok((
                    String::from(name),
                    Node {
                        left: String::from(&left[1..]),
                        right: String::from(&right[..right.len() - 1]),
                    },
                ))
            })
            .collect::<Result<HashMap<_, _>, Day8Error>>()?;

        Ok(Self { path, nodes })
    }
}

impl TryFrom<char> for LR {
    type Error = Day8Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'L' => Ok(Self::Left),
            'R' => Ok(Self::Right),
            char => Err(Day8Error::Direction(char)),
        }
    }
}

impl Display for Day8Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Input => write!(f, "Bad input."),
            Self::Direction(char) => write!(f, "Bad direction '{}'", char),
            Self::Node(string) => write!(f, "Bad node '{}'", string),
            Self::Paths(string) => write!(f, "Bad paths '{}'", string),
            Self::MissingNode(string) => write!(f, "Couldn't find node with name '{}'", string),
            Self::IoError(err) => err.fmt(f),
        }
    }
}

impl Error for Day8Error {}

impl From<IoError> for Day8Error {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}

#[cfg(test)]
mod tests {
    use super::{HashMap, Node, Puzzle, LR};

    #[test]
    fn example_1() {
        let input = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)\n";
        let expected = Puzzle {
            path: vec![LR::Right, LR::Left],
            nodes: HashMap::from([
                (
                    String::from("AAA"),
                    Node {
                        left: String::from("BBB"),
                        right: String::from("CCC"),
                    },
                ),
                (
                    String::from("BBB"),
                    Node {
                        left: String::from("DDD"),
                        right: String::from("EEE"),
                    },
                ),
                (
                    String::from("CCC"),
                    Node {
                        left: String::from("ZZZ"),
                        right: String::from("GGG"),
                    },
                ),
                (
                    String::from("DDD"),
                    Node {
                        left: String::from("DDD"),
                        right: String::from("DDD"),
                    },
                ),
                (
                    String::from("EEE"),
                    Node {
                        left: String::from("EEE"),
                        right: String::from("EEE"),
                    },
                ),
                (
                    String::from("GGG"),
                    Node {
                        left: String::from("GGG"),
                        right: String::from("GGG"),
                    },
                ),
                (
                    String::from("ZZZ"),
                    Node {
                        left: String::from("ZZZ"),
                        right: String::from("ZZZ"),
                    },
                ),
            ]),
        };
        let result: Puzzle = input.try_into().unwrap();

        assert_eq!(result, expected);
    }

    #[test]
    fn example_2() {
        let input = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)\n";
        let expected = Puzzle {
            path: vec![LR::Left, LR::Left, LR::Right],
            nodes: HashMap::from([
                (
                    String::from("AAA"),
                    Node {
                        left: String::from("BBB"),
                        right: String::from("BBB"),
                    },
                ),
                (
                    String::from("BBB"),
                    Node {
                        left: String::from("AAA"),
                        right: String::from("ZZZ"),
                    },
                ),
                (
                    String::from("ZZZ"),
                    Node {
                        left: String::from("ZZZ"),
                        right: String::from("ZZZ"),
                    },
                ),
            ]),
        };
        let result: Puzzle = input.try_into().unwrap();

        assert_eq!(result, expected);
    }
}
