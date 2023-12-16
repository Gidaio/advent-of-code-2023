use super::{Day8Error, Puzzle, LR};

pub fn count_ghost_steps(puzzle: Puzzle) -> Result<usize, Day8Error> {
    let mut steps = 0;
    let mut current_locations: Vec<&str> = puzzle
        .nodes
        .keys()
        .filter_map(|name| {
            if name.ends_with('A') {
                Some(&name[..])
            } else {
                None
            }
        })
        .collect();

    while !all_at_destinations(&current_locations) {
        let direction = puzzle.path.get(steps % puzzle.path.len()).unwrap();
        current_locations = current_locations
            .into_iter()
            .map(|loc| {
                let node = puzzle
                    .nodes
                    .get(loc)
                    .ok_or(Day8Error::MissingNode(String::from(loc)))?;

                Ok(match direction {
                    LR::Left => &node.left[..],
                    LR::Right => &node.right[..],
                })
            })
            .collect::<Result<Vec<&str>, Day8Error>>()?;

        steps += 1;
    }

    Ok(steps)
}

fn all_at_destinations(locations: &[&str]) -> bool {
    locations.iter().all(|loc| loc.ends_with('Z'))
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
