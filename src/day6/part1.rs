use super::{Day6Error, Puzzle};

pub fn multiply_winning_ways(puzzle: Puzzle) -> Result<isize, Day6Error> {
    Ok(puzzle
        .times
        .iter()
        .zip(puzzle.distances.iter())
        .map(|(time, distance)| {
            let mut ways = 0;
            for i in 1..*time {
                if i * (time - i) > *distance {
                    ways += 1;
                }
            }

            ways
        })
        .fold(1, |product, num| product * num))
}

#[cfg(test)]
mod tests {
    use super::{multiply_winning_ways, Puzzle};

    #[test]
    fn provided_example() {
        let puzzle: Puzzle = "Time:      7  15   30\nDistance:  9  40  200\n"
            .try_into()
            .unwrap();
        let ways = multiply_winning_ways(puzzle).unwrap();
        assert_eq!(ways, 288);
    }
}
