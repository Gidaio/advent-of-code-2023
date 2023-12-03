use super::{Day2Error, PuzzleState};

pub fn sum_impossible_game_ids(state: PuzzleState) -> Result<usize, Day2Error> {
    Ok(state
        .0
        .iter()
        .filter(|game| {
            game.pulls
                .iter()
                .all(|pull| pull.red <= 12 && pull.green <= 13 && pull.blue <= 14)
        })
        .fold(0, |sum, game| sum + game.id))
}
