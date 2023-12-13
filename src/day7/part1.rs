use super::{Day7Error, Puzzle};

pub fn calculate_total_winnings(mut puzzle: Puzzle) -> Result<usize, Day7Error> {
    puzzle.0.sort();
    Ok(puzzle
        .0
        .into_iter()
        .enumerate()
        .map(|(rank, hand)| (rank + 1) * hand.bet)
        .sum::<usize>())
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
