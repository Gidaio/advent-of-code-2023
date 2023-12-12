use super::*;

pub fn sum_calibration_values(state: State) -> Result<u32, Day1Error> {
    let results: Vec<u32> = state
        .0
        .iter()
        .map(|str| parse_calibration_value(str))
        .collect::<Result<Vec<u32>, Day1Error>>()?;

    Ok(results.iter().sum())
}

fn parse_calibration_value(line: &str) -> Result<u32, Day1Error> {
    let tens = line
        .chars()
        .find(|char| char.is_ascii_digit())
        .ok_or(Day1Error::NoDigits)?
        .to_digit(10)
        // Unwrapping here is safe because we already know it's a digit.
        .unwrap()
        * 10;
    let ones = line
        .chars()
        .filter(|char| char.is_ascii_digit())
        .next_back()
        // Unwrapping here is safe because we know there's at least one digit from above.
        .unwrap()
        .to_digit(10)
        .unwrap();

    Ok(tens + ones)
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
    fn errors_if_no_digits() {
        let line = String::from("none");
        let result = parse_calibration_value(&line);
        assert_eq!(result, Err(Day1Error::NoDigits));
    }

    #[test]
    fn provided_test() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let result = sum_calibration_values(input.into());
        assert_eq!(result, Ok(142),);
    }
}
