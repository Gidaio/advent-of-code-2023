mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

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

    run_puzzle(
        "Day 3, Part 1",
        day3::part1::sum_part_numbers,
        "inputs/day3.txt",
    );

    run_puzzle(
        "Day 3, Part 2",
        day3::part2::sum_gear_ratios,
        "inputs/day3.txt",
    );

    run_puzzle("Day 4, Part 1", day4::part1::sum_points, "inputs/day4.txt");
    run_puzzle("Day 4, Part 2", day4::part2::count_cards, "inputs/day4.txt");

    run_puzzle(
        "Day 5, Part 1",
        day5::part1::get_lowest_location,
        "inputs/day5.txt",
    );

    run_puzzle(
        "Day 5, Part 2",
        day5::part2::get_lowest_range_location,
        "inputs/day5.txt",
    );

    run_puzzle(
        "Day 6, Part 1",
        day6::part1::multiply_winning_ways,
        "inputs/day6.txt",
    );

    run_puzzle(
        "Day 6, Part 2",
        day6::part2::count_winning_ways,
        "inputs/day6.txt",
    );

    run_puzzle(
        "Day 7, Part 1",
        day7::part1::calculate_total_winnings,
        "inputs/day7.txt",
    );

    run_puzzle(
        "Day 7, Part 2",
        day7::part2::calculate_total_winnings,
        "inputs/day7.txt",
    );

    run_puzzle("Day 8, Part 1", day8::part1::count_steps, "inputs/day8.txt");
    run_puzzle(
        "Day 8, Part 2",
        day8::part2::count_ghost_steps,
        "inputs/day8.txt",
    );

    run_puzzle(
        "Day 9, Part 1",
        day9::part1::sum_extensions,
        "inputs/day9.txt",
    );

    run_puzzle(
        "Day 9, Part 2",
        day9::part2::sum_extensions,
        "inputs/day9.txt",
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
