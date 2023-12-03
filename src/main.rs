mod day1;
mod day2;

use std::error::Error;
use std::fmt::Display;
use std::fs::File;
use std::time::Instant;

fn main() {
    run_puzzle(
        "Day 1, Part 1",
        day1::part1::sum_calibration_values,
        "inputs/day1.txt",
    );

    run_puzzle(
        "Day 1, Part 2",
        day1::part2::sum_calibration_values,
        "inputs/day1.txt",
    );

    run_puzzle(
        "Day 2, Part 1",
        day2::part1::sum_impossible_game_ids,
        "inputs/day2.txt",
    );

    run_puzzle(
        "Day 2, Part 2",
        day2::part2::sum_cube_powers,
        "inputs/day2.txt",
    );
}

fn run_puzzle<In: TryFrom<File>, Out: Display, Err: Error>(
    puzzle_name: &str,
    puzzle_fn: fn(input: In) -> Result<Out, Err>,
    input_filename: &str,
) where
    In::Error: Error,
{
    let file = File::open(input_filename);
    if let Err(err) = file {
        println!("{}: Couldn't read file: {}", puzzle_name, err);
        return;
    }

    let start_time = Instant::now();
    let input = file.unwrap().try_into();
    if let Err(err) = input {
        println!("{}: Couldn't parse input: {}", puzzle_name, err);
        return;
    }

    let result = puzzle_fn(input.unwrap());
    let elapsed_millis = start_time.elapsed().as_millis();
    match result {
        Ok(answer) => println!("{}: {} (in {} ms)", puzzle_name, answer, elapsed_millis),
        Err(err) => println!(
            "{}: Errored with {} (in {} ms, if you care)",
            puzzle_name, err, elapsed_millis
        ),
    }
}
