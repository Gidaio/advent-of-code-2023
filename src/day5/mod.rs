pub mod part1;

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::io::{BufRead, BufReader, Error as IoError};
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Puzzle {
    seeds: Vec<isize>,
    maps: Vec<Map>,
}

#[derive(Debug, PartialEq)]
struct Map {
    from: String,
    to: String,
    ranges: Vec<Range>,
}

impl Map {
    fn from_buf_lines<Iter: Iterator<Item = Result<String, IoError>>>(
        lines: &mut Iter,
    ) -> Result<Self, Day5Error> {
        let header = lines.next().ok_or(Day5Error::NoMapHeader)??;
        let (from, to) = (&header[0..header.len() - 5])
            .split_once("-to-")
            .ok_or(Day5Error::InvalidMapHeader)?;

        let mut ranges = Vec::<Range>::new();

        while let Some(line) = lines.next() {
            let line = line?;
            if line == "" {
                break;
            }

            let nums = line
                .split(' ')
                .map(|num| num.parse::<isize>())
                .collect::<Result<Vec<_>, _>>()?;
            if nums.len() != 3 {
                return Err(Day5Error::InvalidRange);
            }

            ranges.push(Range {
                to_start: nums[0],
                from_start: nums[1],
                size: nums[2],
            });
        }

        Ok(Map {
            from: String::from(from),
            to: String::from(to),
            ranges,
        })
    }

    fn from_str_lines<'a, Iter: Iterator<Item = &'a str>>(
        lines: &mut Iter,
    ) -> Result<Self, Day5Error> {
        let header = lines.next().ok_or(Day5Error::NoMapHeader)?;
        let (from, to) = (&header[0..header.len() - 5])
            .split_once("-to-")
            .ok_or(Day5Error::InvalidMapHeader)?;

        let mut ranges = Vec::<Range>::new();

        while let Some(line) = lines.next() {
            if line.trim().is_empty() {
                break;
            }

            let nums = line
                .split(' ')
                .map(|num| num.parse::<isize>())
                .collect::<Result<Vec<_>, _>>()?;
            if nums.len() != 3 {
                return Err(Day5Error::InvalidRange);
            }

            ranges.push(Range {
                to_start: nums[0],
                from_start: nums[1],
                size: nums[2],
            });
        }

        Ok(Map {
            from: String::from(from),
            to: String::from(to),
            ranges,
        })
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    from_start: isize,
    to_start: isize,
    size: isize,
}

#[derive(Debug)]
pub enum Day5Error {
    NoSeeds,
    NoMapHeader,
    InvalidMapHeader,
    InvalidRange,
    IoError(IoError),
    ParseIntError(ParseIntError),
}

impl TryFrom<File> for Puzzle {
    type Error = Day5Error;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let buf = BufReader::new(value);
        let mut lines = buf.lines().peekable();

        let seeds = lines.next().ok_or(Day5Error::NoSeeds)??;
        if !seeds.starts_with("seeds: ") {
            return Err(Day5Error::NoSeeds);
        }
        let seeds = (&seeds[7..])
            .split(' ')
            .map(|num| num.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;

        // Just gotta skip the empty line.
        let _ = lines.next();

        let mut maps = Vec::<Map>::new();

        while let Some(_) = lines.peek() {
            maps.push(Map::from_buf_lines(&mut lines)?);
        }

        Ok(Puzzle { seeds, maps })
    }
}

impl TryFrom<&str> for Puzzle {
    type Error = Day5Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut lines = value.lines().peekable();

        let seeds = lines.next().ok_or(Day5Error::NoSeeds)?;
        if !seeds.starts_with("seeds: ") {
            return Err(Day5Error::NoSeeds);
        }
        let seeds = (&seeds[7..])
            .split(' ')
            .map(|num| num.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;

        // Just gotta skip the empty line.
        let _ = lines.next();

        let mut maps = Vec::<Map>::new();

        while let Some(_) = lines.peek() {
            maps.push(Map::from_str_lines(&mut lines)?);
        }

        Ok(Puzzle { seeds, maps })
    }
}

impl From<IoError> for Day5Error {
    fn from(value: IoError) -> Self {
        Self::IoError(value)
    }
}

impl From<ParseIntError> for Day5Error {
    fn from(value: ParseIntError) -> Self {
        Self::ParseIntError(value)
    }
}

impl Display for Day5Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSeeds => write!(f, "No seeds!"),
            Self::NoMapHeader => write!(f, "No map header!"),
            Self::InvalidMapHeader => write!(f, "Invalid map header!"),
            Self::InvalidRange => write!(f, "Invalid range!"),
            Self::IoError(err) => err.fmt(f),
            Self::ParseIntError(err) => err.fmt(f),
        }
    }
}

impl Error for Day5Error {}

#[cfg(test)]
mod tests {
    use super::{Map, Puzzle, Range};

    #[test]
    fn parses_example() {
        let expected = Puzzle {
            seeds: vec![79, 14, 55, 13],

            maps: vec![
                Map {
                    from: String::from("seed"),
                    to: String::from("soil"),
                    ranges: vec![
                        Range {
                            to_start: 50,
                            from_start: 98,
                            size: 2,
                        },
                        Range {
                            to_start: 52,
                            from_start: 50,
                            size: 48,
                        },
                    ],
                },
                Map {
                    from: String::from("soil"),
                    to: String::from("fertilizer"),
                    ranges: vec![
                        Range {
                            to_start: 0,
                            from_start: 15,
                            size: 37,
                        },
                        Range {
                            to_start: 37,
                            from_start: 52,
                            size: 2,
                        },
                        Range {
                            to_start: 39,
                            from_start: 0,
                            size: 15,
                        },
                    ],
                },
                Map {
                    from: String::from("fertilizer"),
                    to: String::from("water"),
                    ranges: vec![
                        Range {
                            to_start: 49,
                            from_start: 53,
                            size: 8,
                        },
                        Range {
                            to_start: 0,
                            from_start: 11,
                            size: 42,
                        },
                        Range {
                            to_start: 42,
                            from_start: 0,
                            size: 7,
                        },
                        Range {
                            to_start: 57,
                            from_start: 7,
                            size: 4,
                        },
                    ],
                },
                Map {
                    from: String::from("water"),
                    to: String::from("light"),
                    ranges: vec![
                        Range {
                            to_start: 88,
                            from_start: 18,
                            size: 7,
                        },
                        Range {
                            to_start: 18,
                            from_start: 25,
                            size: 70,
                        },
                    ],
                },
                Map {
                    from: String::from("light"),
                    to: String::from("temperature"),
                    ranges: vec![
                        Range {
                            to_start: 45,
                            from_start: 77,
                            size: 23,
                        },
                        Range {
                            to_start: 81,
                            from_start: 45,
                            size: 19,
                        },
                        Range {
                            to_start: 68,
                            from_start: 64,
                            size: 13,
                        },
                    ],
                },
                Map {
                    from: String::from("temperature"),
                    to: String::from("humidity"),
                    ranges: vec![
                        Range {
                            to_start: 0,
                            from_start: 69,
                            size: 1,
                        },
                        Range {
                            to_start: 1,
                            from_start: 0,
                            size: 69,
                        },
                    ],
                },
                Map {
                    from: String::from("humidity"),
                    to: String::from("location"),
                    ranges: vec![
                        Range {
                            to_start: 60,
                            from_start: 56,
                            size: 37,
                        },
                        Range {
                            to_start: 56,
                            from_start: 93,
                            size: 4,
                        },
                    ],
                },
            ],
        };
        let input = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n";
        let result: Result<Puzzle, _> = input.try_into();
        match result {
            Ok(puzzle) => assert_eq!(puzzle, expected),
            Err(err) => panic!("Got error {}", err),
        }
    }
}
