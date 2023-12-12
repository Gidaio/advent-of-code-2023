use super::Puzzle;

use std::io;

pub fn sum_gear_ratios(puzzle: Puzzle) -> Result<usize, io::Error> {
    Ok(puzzle
        .symbols
        .into_iter()
        .filter_map(|symbol| {
            if symbol.symbol != '*' {
                return None;
            }

            let adjacent_numbers = puzzle
                .numbers
                .iter()
                .filter_map(|number| {
                    if symbol.position.0 as isize >= number.start.0 as isize - 1
                        && symbol.position.0 <= number.start.0 + number.width
                        && symbol.position.1 as isize >= number.start.1 as isize - 1
                        && symbol.position.1 <= number.start.1 + 1
                    {
                        Some(number.value)
                    } else {
                        None
                    }
                })
                .collect::<Vec<usize>>();

            if adjacent_numbers.len() == 2 {
                Some(
                    adjacent_numbers
                        .into_iter()
                        .product::<usize>(),
                )
            } else {
                None
            }
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::{super::Puzzle, sum_gear_ratios};

    #[test]
    fn provided_example() {
        let puzzle_input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let puzzle: Puzzle = puzzle_input.into();
        assert_eq!(sum_gear_ratios(puzzle).unwrap_or(0), 467835);
    }
}
