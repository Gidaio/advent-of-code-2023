use std::collections::HashMap;

use super::{Day8Error, Puzzle, LR};

pub fn count_ghost_steps(puzzle: Puzzle) -> Result<usize, Day8Error> {
    let paths = puzzle
        .nodes
        .keys()
        .filter_map(|name| {
            if name.ends_with('A') {
                Some(&name[..])
            } else {
                None
            }
        })
        .map(|name| {
            let steps = count_steps_from(&puzzle, name)?;
            Ok(steps)
        })
        .collect::<Result<Vec<_>, Day8Error>>()?;

    Ok(paths.iter().fold(1, |curr, path_len| lcm(curr, *path_len)))
}

fn count_steps_from(puzzle: &Puzzle, start: &str) -> Result<usize, Day8Error> {
    let mut current_location = start;
    let mut steps = 0;
    let end_point: usize;
    let mut tracked_locations = HashMap::<(&str, usize), usize>::new();

    loop {
        // Find end point.
        if is_end_node(current_location) {
            end_point = steps;
            break;
        }

        // Save it and take a step.
        tracked_locations.insert((current_location, steps % puzzle.path.len()), steps);

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

    Ok(end_point)
}

fn is_end_node(location: &str) -> bool {
    location.ends_with('Z')
}

fn lcm(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[cfg(test)]
mod tests {
    use super::{count_ghost_steps, Puzzle};

    #[test]
    fn example() {
        let puzzle: Puzzle = "LR\n\n11A = (11B, XXX)\n11B = (XXX, 11Z)\n11Z = (11B, XXX)\n22A = (22B, XXX)\n22B = (22C, 22C)\n22C = (22Z, 22Z)\n22Z = (22B, 22B)\nXXX = (XXX, XXX)\n".try_into().unwrap();
        let steps = count_ghost_steps(puzzle).unwrap();
        assert_eq!(steps, 6);
    }
}
