use super::{Day6Error, Puzzle};

pub fn count_winning_ways(puzzle: Puzzle) -> Result<usize, Day6Error> {
    let time = concatenate_numbers(&puzzle.times);
    let distance = concatenate_numbers(&puzzle.distances);
    let mut ways: usize = 0;

    for i in 1..time {
        if i * (time - i) > distance {
            ways += 1;
        }
    }

    Ok(ways)
}

fn concatenate_numbers(nums: &[isize]) -> isize {
    nums.iter()
        .fold(0, |acc, val| acc * get_digit_multiplier(*val) + val)
}

fn get_digit_multiplier(num: isize) -> isize {
    let mut multiplier = 10;
    let mut num = num;
    while num >= 10 {
        num /= 10;
        multiplier *= 10;
    }

    multiplier
}

#[cfg(test)]
mod tests {
    use super::{count_winning_ways, Puzzle};

    #[test]
    fn provided_example() {
        let puzzle: Puzzle = "Time:      7  15   30\nDistance:  9  40  200\n"
            .try_into()
            .unwrap();
        let ways = count_winning_ways(puzzle).unwrap();
        assert_eq!(ways, 71503);
    }
}
