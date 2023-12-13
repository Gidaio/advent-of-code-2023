use std::collections::HashSet;

use super::{Card, Day7Error, Hand, HandType, Puzzle};

pub fn calculate_total_winnings(mut puzzle: Puzzle) -> Result<usize, Day7Error> {
    puzzle
        .0
        .iter_mut()
        .for_each(|hand: &mut Hand| hand.hand_type = Some(decide_hand_type(&hand.cards)));
    puzzle.0.sort();
    Ok(puzzle
        .0
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bet)
        .sum::<usize>())
}

fn decide_hand_type(cards: &[Card]) -> HandType {
    let num_jokers = cards.iter().filter(|card| **card == Card::Jack).count();
    let non_joker_types =
        HashSet::<Card>::from_iter(cards.iter().filter(|card| **card != Card::Jack).cloned());

    match non_joker_types.len() {
        // JJJJJ 5K
        0 => HandType::FiveOfAKind,
        // AAAAA 5K AAAAJ 5K AAAJJ 5K AAJJJ 5K AJJJJ 5K
        1 => HandType::FiveOfAKind,
        // AAAAK 4K AAAKK FH AAAKJ 4K AAKKK FH AAKKJ FH AAKJJ 4K AKKKK 4K AKKKJ FH AKKJJ 4k AKJJJ 4K
        2 => {
            let max_count = non_joker_types
                .iter()
                .map(|unique_card| {
                    cards
                        .iter()
                        .filter(|vec_card| unique_card == *vec_card)
                        .count()
                })
                .max()
                .unwrap();
            if max_count + num_jokers == 4 {
                HandType::FourOfAKind
            } else {
                HandType::FullHouse
            }
        }
        // AAAKQ 3K AAKKQ 2P AAKQQ 2P AKKKQ 3K AKKQQ 2P AKQQQ 3K AAKQJ 3K AKKQJ 3K AKQQJ 3K AKQJJ 3K
        3 => {
            let max_count = non_joker_types
                .iter()
                .map(|unique_card| {
                    cards
                        .iter()
                        .filter(|vec_card| unique_card == *vec_card)
                        .count()
                })
                .max()
                .unwrap();
            if max_count + num_jokers == 3 {
                HandType::ThreeOfAKind
            } else {
                HandType::TwoPair
            }
        }
        // AAKQT AKKQT AKQQT AKQTT AKQTJ
        4 => HandType::OnePair,
        // AKQT9
        5 => HandType::HighCard,
        _ => unreachable!("Too many unique cards!"),
    }
}

#[cfg(test)]
mod tests {
    use super::{calculate_total_winnings, Puzzle};

    #[test]
    fn provided_example() {
        let puzzle: Puzzle = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n"
            .try_into()
            .unwrap();
        let result = calculate_total_winnings(puzzle).unwrap();
        assert_eq!(result, 5905);
    }
}
