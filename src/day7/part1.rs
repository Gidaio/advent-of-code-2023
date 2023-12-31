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
    use super::{calculate_total_winnings, Puzzle};

    #[test]
    fn provided_example() {
        let puzzle: Puzzle = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483\n"
            .try_into()
            .unwrap();
        let result = calculate_total_winnings(puzzle).unwrap();
        assert_eq!(result, 6440);
    }
}
