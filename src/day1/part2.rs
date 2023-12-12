use super::*;

const PATTERNS: [&str; 20] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "zero", "one", "two", "three", "four",
    "five", "six", "seven", "eight", "nine",
];

pub fn sum_calibration_values(state: State) -> Result<usize, Day1Error> {
    let results: Vec<usize> = state
        .0
        .iter()
        .map(|str| parse_calibration_value(str))
        .collect::<Result<Vec<usize>, Day1Error>>()?;

    Ok(results.iter().sum())
}

fn parse_calibration_value(line: &str) -> Result<usize, Day1Error> {
    let mut earliest_match: Option<(usize, usize)> = None;
    let mut latest_match: Option<(usize, usize)> = None;

    for pattern in PATTERNS {
        if let Some(index) = line.match_indices(pattern).next() {
            if let Some(earliest) = earliest_match {
                if index.0 < earliest.0 {
                    earliest_match = Some((index.0, parse_num_str(index.1)));
                }
            } else {
                earliest_match = Some((index.0, parse_num_str(index.1)));
            }
        }

        if let Some(index) = line.rmatch_indices(pattern).next() {
            if let Some(latest) = latest_match {
                if index.0 > latest.0 {
                    latest_match = Some((index.0, parse_num_str(index.1)));
                }
            } else {
                latest_match = Some((index.0, parse_num_str(index.1)));
            }
        }
    }

    if earliest_match.is_none() {
        return Err(Day1Error::NoDigits);
    }

    Ok(earliest_match.unwrap().1 * 10 + latest_match.unwrap().1)
}

fn parse_num_str(slice: &str) -> usize {
    match slice {
        "0" | "zero" => 0,
        "1" | "one" => 1,
        "2" | "two" => 2,
        "3" | "three" => 3,
        "4" | "four" => 4,
        "5" | "five" => 5,
        "6" | "six" => 6,
        "7" | "seven" => 7,
        "8" | "eight" => 8,
        "9" | "nine" => 9,
        misc => unreachable!("Somehow I parsed a {}", misc),
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_calibration_value, sum_calibration_values, Day1Error};

    #[test]
    fn can_parse_line() {
        let line = String::from("pqr3stu8vwx");
        let result = parse_calibration_value(&line);
        assert_eq!(
            result,
            Ok(38),
            "Parsed {:?} from {}, but expected {}.",
            result,
            line,
            38
        );
    }

    #[test]
    fn ignores_middle_digits() {
        let line = String::from("a1b2c3d4e5f");
        let result = parse_calibration_value(&line);
        assert_eq!(
            result,
            Ok(15),
            "Parsed {:?} from {}, but expected {}.",
            result,
            line,
            15
        );
    }

    #[test]
    fn one_digit_can_be_both() {
        let line = String::from("treb7uchet");
        let result = parse_calibration_value(&line);
        assert_eq!(
            result,
            Ok(77),
            "Parsed {:?} from {}, but expected {}.",
            result,
            line,
            77
        );
    }

    #[test]
    fn parses_spelled_digit() {
        let line = String::from("abcone2threexyz");
        let result = parse_calibration_value(&line);
        assert_eq!(
            result,
            Ok(13),
            "Parsed {:?} from {}, but expected {}.",
            result,
            line,
            13
        );
    }

    #[test]
    fn errors_if_no_digits() {
        let line = String::from("nodigits");
        let result = parse_calibration_value(&line);
        assert_eq!(result, Err(Day1Error::NoDigits));
    }

    #[test]
    fn provided_test() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";
        let result = sum_calibration_values(input.into());
        assert_eq!(result, Ok(281),);
    }
}
