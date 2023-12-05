use super::{Day4Error, Puzzle};

pub fn count_cards(puzzle: Puzzle) -> Result<usize, Day4Error> {
    let mut card_counts = vec![1; puzzle.0.len()];

    for (i, card) in puzzle.0.iter().enumerate() {
        let matching_numbers = card
            .winning_numbers
            .intersection(&card.scratched_numbers)
            .count();
        for j in i + 1..i + 1 + matching_numbers {
            card_counts[j] += card_counts[i];
        }
    }

    Ok(card_counts.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::super::Puzzle;
    use super::count_cards;

    #[test]
    fn provided_example() {
        let puzzle: Puzzle = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n".try_into().unwrap();
        let result = count_cards(puzzle).unwrap();
        assert_eq!(result, 30);
    }
}