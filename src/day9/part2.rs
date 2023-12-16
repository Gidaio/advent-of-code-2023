use super::{Day9Error, Puzzle};

pub fn sum_extensions(puzzle: Puzzle) -> Result<isize, Day9Error> {
    Ok(puzzle.0.into_iter().map(|list| get_extension(&list)).sum())
}

fn get_extension(nums: &[isize]) -> isize {
    let next = if nums.iter().all(|num| *num == 0) {
        0
    } else {
        let differences: Vec<isize> = nums.windows(2).map(|w| w[1] - w[0]).collect();
        nums.first().unwrap() - get_extension(&differences)
    };

    next
}

#[cfg(test)]
mod tests {
    use super::{sum_extensions, Puzzle};

    #[test]
    fn example() {
        let input: Puzzle = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45\n"
            .try_into()
            .unwrap();
        let result = sum_extensions(input).unwrap();
        assert_eq!(result, 2);
    }
}
