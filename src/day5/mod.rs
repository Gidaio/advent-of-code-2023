pub mod part1;
pub mod part2;

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
    entries: Vec<MapEntry>,
}

impl Map {
    fn from_buf_lines<Iter: Iterator<Item = Result<String, IoError>>>(
        lines: &mut Iter,
    ) -> Result<Self, Day5Error> {
        let header = lines.next().ok_or(Day5Error::NoMapHeader)??;
        let (from, to) = header[0..header.len() - 5]
            .split_once("-to-")
            .ok_or(Day5Error::InvalidMapHeader)?;

        let mut entries = Vec::<MapEntry>::new();

        for line in lines {
            let line = line?;
            if line.is_empty() {
                break;
            }

            let nums = line
                .split(' ')
                .map(|num| num.parse::<isize>())
                .collect::<Result<Vec<_>, _>>()?;
            if nums.len() != 3 {
                return Err(Day5Error::InvalidRange);
            }

            entries.push(MapEntry {
                destination: Range::from_size(nums[0], nums[2]),
                source: Range::from_size(nums[1], nums[2]),
            });
        }

        Ok(Map {
            from: String::from(from),
            to: String::from(to),
            entries,
        })
    }

    fn from_str_lines<'a, Iter: Iterator<Item = &'a str>>(
        lines: &mut Iter,
    ) -> Result<Self, Day5Error> {
        let header = lines.next().ok_or(Day5Error::NoMapHeader)?;
        let (from, to) = header[0..header.len() - 5]
            .split_once("-to-")
            .ok_or(Day5Error::InvalidMapHeader)?;

        let mut entries = Vec::<MapEntry>::new();

        for line in lines {
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

            entries.push(MapEntry {
                destination: Range::from_size(nums[0], nums[2]),
                source: Range::from_size(nums[1], nums[2]),
            });
        }

        Ok(Map {
            from: String::from(from),
            to: String::from(to),
            entries,
        })
    }
}

#[derive(Debug, PartialEq)]
struct MapEntry {
    source: Range,
    destination: Range,
}

impl MapEntry {
    fn map_number(&self, num: isize) -> isize {
        if self.source.contains_value(num) {
            num - self.source.from + self.destination.from
        } else {
            num
        }
    }

    fn map_range(&self, range: &Range) -> (Range, Option<Range>, Option<Range>) {
        if self.source.contains_range(range) {
            // Self  |---------|   |-----|   |-----|   |-----|
            // Other    |---|      |---|       |---|   |-----|
            (
                Range {
                    from: self.map_number(range.from),
                    to: self.map_number(range.to),
                },
                None,
                None,
            )
        } else if range.contains_range(&self.source) {
            // Self     |---|      |---|       |---|
            // Other |---------|   |-----|   |-----|
            let left = if range.from < self.source.from {
                Some(Range {
                    from: range.from,
                    to: self.source.from - 1,
                })
            } else {
                None
            };

            let right = if range.to > self.source.to {
                Some(Range {
                    from: self.source.to + 1,
                    to: range.to,
                })
            } else {
                None
            };

            (
                Range {
                    from: self.destination.from,
                    to: self.destination.to,
                },
                left,
                right,
            )
        } else if self.source.from < range.from {
            // Self  |---|
            // Other   |---|
            (
                Range {
                    from: self.map_number(range.from),
                    to: self.destination.to,
                },
                None,
                Some(Range {
                    from: self.source.to + 1,
                    to: range.to,
                }),
            )
        } else {
            // Self    |---|
            // Other |---|
            (
                Range {
                    from: self.destination.from,
                    to: self.map_number(range.to),
                },
                Some(Range {
                    from: range.from,
                    to: self.source.from - 1,
                }),
                None,
            )
        }
    }
}

#[derive(Debug, PartialEq)]
struct Range {
    // Inclusive
    from: isize,
    // Also inclusive
    to: isize,
}

impl Range {
    fn from_size(start: isize, size: isize) -> Self {
        Self {
            from: start,
            to: start + size - 1,
        }
    }

    fn contains_value(&self, num: isize) -> bool {
        num >= self.from && num <= self.to
    }

    fn contains_range(&self, range: &Self) -> bool {
        self.contains_value(range.from) && self.contains_value(range.to)
    }

    fn overlaps(&self, range: &Self) -> bool {
        self.contains_value(range.from) || self.contains_value(range.to)
    }
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
        let seeds = seeds[7..]
            .split(' ')
            .map(|num| num.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;

        // Just gotta skip the empty line.
        let _ = lines.next();

        let mut maps = Vec::<Map>::new();

        while lines.peek().is_some() {
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
        let seeds = seeds[7..]
            .split(' ')
            .map(|num| num.parse::<isize>())
            .collect::<Result<Vec<_>, _>>()?;

        // Just gotta skip the empty line.
        let _ = lines.next();

        let mut maps = Vec::<Map>::new();

        while lines.peek().is_some() {
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
    use super::{Map, MapEntry, Puzzle, Range};

    const MAP: MapEntry = MapEntry {
        source: Range { from: 2, to: 8 },
        destination: Range { from: 22, to: 28 },
    };

    #[test]
    fn parses_example() {
        let expected = Puzzle {
            seeds: vec![79, 14, 55, 13],

            maps: vec![
                Map {
                    from: String::from("seed"),
                    to: String::from("soil"),
                    entries: vec![
                        MapEntry {
                            destination: Range::from_size(50, 2),
                            source: Range::from_size(98, 2),
                        },
                        MapEntry {
                            destination: Range::from_size(52, 48),
                            source: Range::from_size(50, 48),
                        },
                    ],
                },
                Map {
                    from: String::from("soil"),
                    to: String::from("fertilizer"),
                    entries: vec![
                        MapEntry {
                            destination: Range::from_size(0, 37),
                            source: Range::from_size(15, 37),
                        },
                        MapEntry {
                            destination: Range::from_size(37, 2),
                            source: Range::from_size(52, 2),
                        },
                        MapEntry {
                            destination: Range::from_size(39, 15),
                            source: Range::from_size(0, 15),
                        },
                    ],
                },
                Map {
                    from: String::from("fertilizer"),
                    to: String::from("water"),
                    entries: vec![
                        MapEntry {
                            destination: Range::from_size(49, 8),
                            source: Range::from_size(53, 8),
                        },
                        MapEntry {
                            destination: Range::from_size(0, 42),
                            source: Range::from_size(11, 42),
                        },
                        MapEntry {
                            destination: Range::from_size(42, 7),
                            source: Range::from_size(0, 7),
                        },
                        MapEntry {
                            destination: Range::from_size(57, 4),
                            source: Range::from_size(7, 4),
                        },
                    ],
                },
                Map {
                    from: String::from("water"),
                    to: String::from("light"),
                    entries: vec![
                        MapEntry {
                            destination: Range::from_size(88, 7),
                            source: Range::from_size(18, 7),
                        },
                        MapEntry {
                            destination: Range::from_size(18, 70),
                            source: Range::from_size(25, 70),
                        },
                    ],
                },
                Map {
                    from: String::from("light"),
                    to: String::from("temperature"),
                    entries: vec![
                        MapEntry {
                            destination: Range::from_size(45, 23),
                            source: Range::from_size(77, 23),
                        },
                        MapEntry {
                            destination: Range::from_size(81, 19),
                            source: Range::from_size(45, 19),
                        },
                        MapEntry {
                            destination: Range::from_size(68, 13),
                            source: Range::from_size(64, 13),
                        },
                    ],
                },
                Map {
                    from: String::from("temperature"),
                    to: String::from("humidity"),
                    entries: vec![
                        MapEntry {
                            destination: Range::from_size(0, 1),
                            source: Range::from_size(69, 1),
                        },
                        MapEntry {
                            destination: Range::from_size(1, 69),
                            source: Range::from_size(0, 69),
                        },
                    ],
                },
                Map {
                    from: String::from("humidity"),
                    to: String::from("location"),
                    entries: vec![
                        MapEntry {
                            destination: Range::from_size(60, 37),
                            source: Range::from_size(56, 37),
                        },
                        MapEntry {
                            destination: Range::from_size(56, 4),
                            source: Range::from_size(93, 4),
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

    #[test]
    // 0123456789X
    //   |-----|
    //     |-----|
    fn map_range_greater_greater() {
        let range = Range { from: 4, to: 10 };
        let result = MAP.map_range(&range);
        assert_eq!(
            result,
            (
                Range { from: 24, to: 28 },
                None,
                Some(Range { from: 9, to: 10 })
            )
        );
    }

    #[test]
    // 0123456789X
    //   |-----|
    //     |---|
    fn map_range_greater_equal() {
        let range = Range { from: 4, to: 8 };
        let result = MAP.map_range(&range);
        assert_eq!(result, (Range { from: 24, to: 28 }, None, None,));
    }

    #[test]
    // 0123456789X
    //   |-----|
    //     |-|
    fn map_range_greater_less() {
        let range = Range { from: 4, to: 6 };
        let result = MAP.map_range(&range);
        assert_eq!(result, (Range { from: 24, to: 26 }, None, None,));
    }

    #[test]
    // 0123456789X
    //   |-----|
    //   |-------|
    fn map_range_equal_greater() {
        let range = Range { from: 2, to: 10 };
        let result = MAP.map_range(&range);
        assert_eq!(
            result,
            (
                Range { from: 22, to: 28 },
                None,
                Some(Range { from: 9, to: 10 }),
            )
        );
    }

    #[test]
    // 0123456789X
    //   |-----|
    //   |-----|
    fn map_range_equal_equal() {
        let range = Range { from: 2, to: 8 };
        let result = MAP.map_range(&range);
        assert_eq!(result, (Range { from: 22, to: 28 }, None, None,));
    }

    #[test]
    // 0123456789X
    //   |-----|
    //   |---|
    fn map_range_equal_less() {
        let range = Range { from: 2, to: 6 };
        let result = MAP.map_range(&range);
        assert_eq!(result, (Range { from: 22, to: 26 }, None, None,));
    }

    #[test]
    // 0123456789X
    //   |-----|
    // |---------|
    fn map_range_less_greater() {
        let range = Range { from: 0, to: 10 };
        let result = MAP.map_range(&range);
        assert_eq!(
            result,
            (
                Range { from: 22, to: 28 },
                Some(Range { from: 0, to: 1 }),
                Some(Range { from: 9, to: 10 }),
            )
        );
    }

    #[test]
    // 0123456789X
    //   |-----|
    // |-------|
    fn map_range_less_equal() {
        let range = Range { from: 0, to: 8 };
        let result = MAP.map_range(&range);
        assert_eq!(
            result,
            (
                Range { from: 22, to: 28 },
                Some(Range { from: 0, to: 1 }),
                None,
            )
        );
    }

    #[test]
    // 0123456789X
    //   |-----|
    // |-----|
    fn map_range_less_less() {
        let range = Range { from: 0, to: 6 };
        let result = MAP.map_range(&range);
        assert_eq!(
            result,
            (
                Range { from: 22, to: 26 },
                Some(Range { from: 0, to: 1 }),
                None,
            )
        );
    }
}
