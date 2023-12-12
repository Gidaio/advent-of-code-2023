use super::{Day5Error, MapEntry, Puzzle, Range};

pub fn get_lowest_range_location(puzzle: Puzzle) -> Result<isize, Day5Error> {
    let mut numbers = get_seed_ranges(&puzzle.seeds);
    let mut stage = "seed";

    while stage != "location" {
        let map = puzzle.maps.iter().find(|map| map.from == stage).unwrap();
        numbers = numbers
            .into_iter()
            .flat_map(|range| map_range_recursive(&map.entries, range))
            .collect::<Vec<_>>();
        stage = &map.to[..];
    }

    Ok(numbers.into_iter().map(|range| range.from).min().unwrap())
}

fn get_seed_ranges(seeds: &[isize]) -> Vec<Range> {
    seeds
        .iter()
        .step_by(2)
        .zip(seeds.iter().skip(1).step_by(2))
        .map(|(start, size)| Range::from_size(*start, *size))
        .collect()
}

fn map_range_recursive(entries: &Vec<MapEntry>, range: Range) -> Vec<Range> {
    let matching_entry = entries.iter().find(|entry| entry.source.overlaps(&range));
    if let Some(entry) = matching_entry {
        let (mapped, left, right) = entry.map_range(&range);
        let mut output = vec![mapped];

        if let Some(left) = left {
            output.append(&mut map_range_recursive(entries, left));
        }

        if let Some(right) = right {
            output.append(&mut map_range_recursive(entries, right));
        }

        output
    } else {
        vec![range]
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Puzzle, Range};
    use super::{get_lowest_range_location, get_seed_ranges};

    #[test]
    fn can_get_seed_ranges() {
        let seeds = vec![79, 14, 55, 13];
        let result = get_seed_ranges(&seeds);
        assert_eq!(
            result,
            vec![Range { from: 79, to: 92 }, Range { from: 55, to: 67 }]
        );
    }

    #[test]
    fn provided_example() {
        let input: Puzzle = "seeds: 79 14 55 13\n\nseed-to-soil map:\n50 98 2\n52 50 48\n\nsoil-to-fertilizer map:\n0 15 37\n37 52 2\n39 0 15\n\nfertilizer-to-water map:\n49 53 8\n0 11 42\n42 0 7\n57 7 4\n\nwater-to-light map:\n88 18 7\n18 25 70\n\nlight-to-temperature map:\n45 77 23\n81 45 19\n68 64 13\n\ntemperature-to-humidity map:\n0 69 1\n1 0 69\n\nhumidity-to-location map:\n60 56 37\n56 93 4\n".try_into().unwrap();
        let result = get_lowest_range_location(input);
        match result {
            Ok(location) => assert_eq!(location, 46),
            Err(err) => panic!("Got error {}", err),
        }
    }
}
