use super::{Day5Error, Puzzle};

pub fn get_lowest_location(puzzle: Puzzle) -> Result<isize, Day5Error> {
    let mut numbers = puzzle.seeds.clone();
    let mut stage = "seed";

    while stage != "location" {
        let map = puzzle.maps.iter().find(|map| map.from == stage).unwrap();
        numbers = numbers
            .into_iter()
            .map(|num| {
                let matching_entry = map
                    .entries
                    .iter()
                    .find(|entry| entry.source.contains(num));
                if let Some(entry) = matching_entry {
                    entry.map_number(num)
                } else {
                    num
                }
            })
            .collect::<Vec<_>>();
        stage = &map.to[..];
    }

    Ok(numbers.into_iter().min().unwrap())
}

#[cfg(test)]
mod tests {
    use super::super::Puzzle;
    use super::get_lowest_location;

    #[test]
    fn provided_example() {
        let input: Puzzle = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n".try_into().unwrap();
        let result = get_lowest_location(input);
        match result {
            Ok(answer) => assert_eq!(answer, 35),
            Err(err) => panic!("Got error {}", err),
        }
    }
}
