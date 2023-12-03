use super::Puzzle;
use std::io;

pub fn sum_part_numbers(puzzle: Puzzle) -> Result<usize, io::Error> {
    let part_numbers = puzzle
        .numbers
        .into_iter()
        .filter(|number| {
            puzzle.symbols.iter().any(|symbol| {
                symbol.position.0 as isize >= number.start.0 as isize - 1
                    && symbol.position.0 <= number.start.0 + number.width
                    && symbol.position.1 as isize >= number.start.1 as isize - 1
                    && symbol.position.1 <= number.start.1 + 1
            })
        })
        .collect::<Vec<_>>();

    Ok(part_numbers
        .into_iter()
        .fold(0, |sum, number| sum + number.value))
}

#[cfg(test)]
mod tests {
    use super::{super::Puzzle, sum_part_numbers};

    #[test]
    fn provided_example() {
        let puzzle_input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        let puzzle: Puzzle = puzzle_input.into();
        assert_eq!(sum_part_numbers(puzzle).unwrap_or(0), 4361);
    }
}
