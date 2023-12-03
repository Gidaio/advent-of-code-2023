use super::{Day2Error, Pull, PuzzleState};

pub fn sum_cube_powers(state: PuzzleState) -> Result<usize, Day2Error> {
    Ok(state
        .0
        .iter()
        .map(|game| {
            let min_pull = game.pulls.iter().fold(
                Pull {
                    red: 0,
                    green: 0,
                    blue: 0,
                },
                |min, pull| Pull {
                    red: pull.red.max(min.red),
                    green: pull.green.max(min.green),
                    blue: pull.blue.max(min.blue),
                },
            );
            min_pull.red * min_pull.green * min_pull.blue
        })
        .sum())
}
