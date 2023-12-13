pub mod part1;

use std::cmp::Ordering;
use std::collections::HashSet;
use std::error::Error;
use std::io::{BufRead, BufReader, Error as IoError};
use std::fmt::Display;
use std::fs::File;
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub struct Puzzle(Vec<Hand>);

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bet: usize,
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum Card {
    Ace = 14,
    King = 13,
    Queen = 12,
    Jack = 11,
    Ten = 10,
    Nine = 9,
    Eight = 8,
    Seven = 7,
    Six = 6,
    Five = 5,
    Four = 4,
    Three = 3,
    Two = 2,
}

#[derive(Debug)]
pub enum Day7Error {
    UnrecognizedCharacter(char),
    UnparsableHand(String),
    ParseIntError(ParseIntError),
    IoError(IoError),
}

impl TryFrom<File> for Puzzle {
    type Error = Day7Error;

    fn try_from(value: File) -> Result<Self, Self::Error> {
        let buf = BufReader::new(value);
        let lines = buf.lines();
        let hands = lines
            .map(|line| (&line?[..]).try_into())
            .collect::<Result<Vec<Hand>, _>>()?;
        Ok(Self(hands))
    }
}

impl TryFrom<&str> for Puzzle {
    type Error = Day7Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let lines = value.lines();
        let hands = lines
            .map(|line| line.try_into())
            .collect::<Result<Vec<Hand>, _>>()?;
        Ok(Self(hands))
    }
}

impl TryFrom<&str> for Hand {
    type Error = Day7Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let (cards, bet) = value
            .split_once(' ')
            .ok_or(Day7Error::UnparsableHand(value.to_string()))?;
        let cards = cards
            .chars()
            .map(|char| char.try_into())
            .collect::<Result<Vec<Card>, _>>()?;

        let hand_type = decide_hand_type(&cards[..]);

        let bet: usize = bet.parse()?;

        Ok(Self {
            cards,
            hand_type,
            bet,
        })
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_type_ordering = self.hand_type.cmp(&other.hand_type);
        if hand_type_ordering != Ordering::Equal {
            hand_type_ordering
        } else {
            self.cards
                .iter()
                .zip(other.cards.iter())
                .find_map(|(l, r)| match l.cmp(r) {
                    Ordering::Equal => None,
                    cmp => Some(cmp),
                })
                .unwrap_or(Ordering::Equal)
        }
    }
}

impl TryFrom<char> for Card {
    type Error = Day7Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' => Ok(Card::Ace),
            'K' => Ok(Card::King),
            'Q' => Ok(Card::Queen),
            'J' => Ok(Card::Jack),
            'T' => Ok(Card::Ten),
            '9' => Ok(Card::Nine),
            '8' => Ok(Card::Eight),
            '7' => Ok(Card::Seven),
            '6' => Ok(Card::Six),
            '5' => Ok(Card::Five),
            '4' => Ok(Card::Four),
            '3' => Ok(Card::Three),
            '2' => Ok(Card::Two),
            any => Err(Self::Error::UnrecognizedCharacter(any)),
        }
    }
}

impl Display for Day7Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnrecognizedCharacter(cha) => write!(f, "Unrecognized character '{}'", cha),
            Self::UnparsableHand(hand) => write!(f, "Unparsable hand '{}'", hand),
            Self::ParseIntError(err) => err.fmt(f),
            Self::IoError(err) => err.fmt(f),
        }
    }
}

impl Error for Day7Error {}

impl From<ParseIntError> for Day7Error {
    fn from(value: ParseIntError) -> Self {
        Day7Error::ParseIntError(value)
    }
}

impl From<IoError> for Day7Error {
    fn from(value: IoError) -> Self {
        Day7Error::IoError(value)
    }
}

fn decide_hand_type(cards: &[Card]) -> HandType {
    let unique_cards = HashSet::<Card>::from_iter(cards.iter().cloned());

    match unique_cards.len() {
        // AAAAA
        1 => HandType::FiveOfAKind,
        // AAAAK AAAKK AAKKK AKKKK
        2 => {
            let max_count = unique_cards
                .iter()
                .map(|unique_card| {
                    cards
                        .iter()
                        .filter(|vec_card| unique_card == *vec_card)
                        .count()
                })
                .max()
                .unwrap();
            if max_count == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        // AAAKQ AAKKQ AAKQQ AKKKQ AKKQQ AKQQQ
        3 => {
            let max_count = unique_cards
                .iter()
                .map(|unique_card| {
                    cards
                        .iter()
                        .filter(|vec_card| unique_card == *vec_card)
                        .count()
                })
                .max()
                .unwrap();
            if max_count == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        // AAKQJ AKKQJ AKQQJ AKQJJ
        4 => HandType::OnePair,
        // AKQJT
        5 => HandType::HighCard,
        _ => unreachable!("Too many unique cards!"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Card, Hand, HandType, Puzzle};

    #[test]
    fn provided_example() {
        let expected = Puzzle(vec![
            Hand {
                cards: vec![Card::Three, Card::Two, Card::Ten, Card::Three, Card::King],
                hand_type: HandType::OnePair,
                bet: 765,
            },
            Hand {
                cards: vec![Card::Ten, Card::Five, Card::Five, Card::Jack, Card::Five],
                hand_type: HandType::ThreeOfAKind,
                bet: 684,
            },
            Hand {
                cards: vec![Card::King, Card::King, Card::Six, Card::Seven, Card::Seven],
                hand_type: HandType::TwoPair,
                bet: 28,
            },
            Hand {
                cards: vec![Card::King, Card::Ten, Card::Jack, Card::Jack, Card::Ten],
                hand_type: HandType::TwoPair,
                bet: 220,
            },
            Hand {
                cards: vec![Card::Queen, Card::Queen, Card::Queen, Card::Jack, Card::Ace],
                hand_type: HandType::ThreeOfAKind,
                bet: 483,
            },
        ]);
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n";
        let puzzle: Puzzle = input.try_into().unwrap();
        assert_eq!(puzzle, expected);
    }
}
