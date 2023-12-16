use super::{Day8Error, Puzzle, LR};

pub fn count_steps(puzzle: Puzzle) -> Result<usize, Day8Error> {
    let mut current_location = "AAA";
    let mut steps = 0;

    while current_location != "ZZZ" {
        let node = puzzle
            .nodes
            .get(current_location)
            .ok_or(Day8Error::MissingNode(String::from(current_location)))?;
        let direction = puzzle.path.get(steps % puzzle.path.len()).unwrap();

        current_location = match direction {
            LR::Left => &node.left,
            LR::Right => &node.right,
        };

        steps += 1;
    }

    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::{count_steps, Puzzle};

    #[test]
    fn example_1() {
        let input: Puzzle = "RL\n\nAAA = (BBB, CCC)\nBBB = (DDD, EEE)\nCCC = (ZZZ, GGG)\nDDD = (DDD, DDD)\nEEE = (EEE, EEE)\nGGG = (GGG, GGG)\nZZZ = (ZZZ, ZZZ)\n".try_into().unwrap();
        let steps = count_steps(input).unwrap();
        assert_eq!(steps, 2);
    }

    #[test]
    fn example_2() {
        let input: Puzzle = "LLR\n\nAAA = (BBB, BBB)\nBBB = (AAA, ZZZ)\nZZZ = (ZZZ, ZZZ)\n"
            .try_into()
            .unwrap();
        let steps = count_steps(input).unwrap();
        assert_eq!(steps, 6);
    }
}
