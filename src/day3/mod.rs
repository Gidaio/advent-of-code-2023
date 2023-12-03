pub mod part1;
pub mod part2;

use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug)]
pub struct Puzzle {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

#[derive(Debug)]
struct Number {
    start: (usize, usize),
    width: usize,
    value: usize,
}

#[derive(Debug)]
struct Symbol {
    position: (usize, usize),
    symbol: char,
}

impl TryFrom<File> for Puzzle {
    type Error = io::Error;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let mut numbers: Vec<Number> = vec![];
        let mut symbols: Vec<Symbol> = vec![];

        let buf = BufReader::new(value);

        for (y, line) in buf.lines().enumerate() {
            let line = line?;
            let mut accumulated_number: usize = 0;
            let mut start_x: usize = 0;
            for (x, character) in line.chars().enumerate() {
                match character {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        accumulated_number *= 10;
                        accumulated_number += character.to_digit(10).unwrap() as usize;
                    }
                    non_num => {
                        if accumulated_number > 0 {
                            numbers.push(Number {
                                start: (start_x, y),
                                width: x - start_x,
                                value: accumulated_number,
                            });
                            accumulated_number = 0;
                        }
                        start_x = x + 1;

                        match non_num {
                            '.' => (),
                            symbol => symbols.push(Symbol {
                                position: (x, y),
                                symbol,
                            }),
                        }
                    }
                }
            }

            if accumulated_number > 0 {
                numbers.push(Number {
                    start: (start_x, y),
                    width: line.len() - start_x,
                    value: accumulated_number,
                });
            }
        }

        Ok(Puzzle { numbers, symbols })
    }
}

impl From<&str> for Puzzle {
    fn from(value: &str) -> Self {
        let mut numbers: Vec<Number> = vec![];
        let mut symbols: Vec<Symbol> = vec![];

        for (y, line) in value.lines().enumerate() {
            let mut accumulated_number: usize = 0;
            let mut start_x: usize = 0;
            for (x, character) in line.chars().enumerate() {
                match character {
                    '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                        accumulated_number *= 10;
                        accumulated_number += character.to_digit(10).unwrap() as usize;
                    }
                    non_num => {
                        if accumulated_number > 0 {
                            numbers.push(Number {
                                start: (start_x, y),
                                width: x - start_x,
                                value: accumulated_number,
                            });
                            accumulated_number = 0;
                        }
                        start_x = x + 1;

                        match non_num {
                            '.' => (),
                            symbol => symbols.push(Symbol {
                                position: (x, y),
                                symbol,
                            }),
                        }
                    }
                }
            }

            if accumulated_number > 0 {
                numbers.push(Number {
                    start: (start_x, y),
                    width: line.len() - start_x,
                    value: accumulated_number,
                });
            }
        }

        Puzzle { numbers, symbols }
    }
}
