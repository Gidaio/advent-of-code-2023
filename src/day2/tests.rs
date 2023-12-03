use super::{part1::sum_impossible_game_ids, Day2Error, Game, Pull, PuzzleState};

#[test]
fn parses_colors() {
    let result: Result<Pull, Day2Error> = "1 red, 2 green, 6 blue".try_into();
    match result {
        Ok(pull) => {
            assert_eq!(pull.red, 1);
            assert_eq!(pull.green, 2);
            assert_eq!(pull.blue, 6);
        }
        Err(err) => panic!("Got error {}", err),
    }
}

#[test]
fn handles_missing_colors() {
    let result: Result<Pull, Day2Error> = "3 blue, 4 red".try_into();
    match result {
        Ok(pull) => {
            assert_eq!(pull.red, 4);
            assert_eq!(pull.green, 0);
            assert_eq!(pull.blue, 3);
        }
        Err(err) => panic!("Got error {}", err),
    }
}

#[test]
fn errors_if_empty_string() {
    let cube = "";
    let result: Result<Pull, Day2Error> = cube.try_into();
    match result {
        Ok(pull) => panic!("Got success {:?}", pull),
        Err(Day2Error::NoColor(err_cube)) => assert_eq!(err_cube, cube),
        Err(err) => panic!("Got wrong error {}", err),
    }
}

#[test]
fn errors_if_no_color() {
    let cube = "7";
    let result: Result<Pull, Day2Error> = cube.try_into();
    match result {
        Ok(pull) => panic!("Got success {:?}", pull),
        Err(Day2Error::NoColor(err_cube)) => assert_eq!(err_cube, cube),
        Err(err) => panic!("Got wrong error {}", err),
    }
}

#[test]
fn errors_if_count_isnt_number() {
    let cube = "nine red";
    let result: Result<Pull, Day2Error> = cube.try_into();
    match result {
        Ok(pull) => panic!("Got success {:?}", pull),
        Err(Day2Error::NotANumber(err_cube)) => assert_eq!(err_cube, cube),
        Err(err) => panic!("Got wrong error {}", err),
    }
}

#[test]
fn errors_if_color_is_bad() {
    let cube = "9 chartreuse";
    let result: Result<Pull, Day2Error> = cube.try_into();
    match result {
        Ok(pull) => panic!("Got success {:?}", pull),
        Err(Day2Error::UnrecognizedColor(err_cube)) => assert_eq!(err_cube, cube),
        Err(err) => panic!("Got wrong error {}", err),
    }
}

#[test]
fn parses_a_game() {
    let expected = Game {
        id: 3,
        pulls: vec![
            Pull {
                red: 20,
                green: 8,
                blue: 6,
            },
            Pull {
                red: 4,
                green: 13,
                blue: 5,
            },
            Pull {
                red: 1,
                green: 5,
                blue: 0,
            },
        ],
    };
    let line =
        String::from("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red");
    let result: Result<Game, Day2Error> = line.try_into();
    match result {
        Ok(game) => assert_eq!(game, expected),
        Err(err) => panic!("Got error {}", err),
    }
}

#[test]
fn provided_example() {
    let state: PuzzleState = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".try_into().unwrap();
    println!("{:#?}", state);
    let result = sum_impossible_game_ids(state);
    match result {
        Ok(sum) => assert_eq!(sum, 8),
        Err(err) => panic!("Got error {}", err),
    }
}
