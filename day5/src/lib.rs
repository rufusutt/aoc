#[derive(Debug, PartialEq, Eq)]
struct MapRange {
    destination: u32,
    source: u32,
    len: u32,
}

impl MapRange {
    fn contains_source(&self, x: u32) -> bool {
        x >= self.source && x < (self.source + self.len)
    }
}

type Map = Vec<MapRange>;

#[derive(Debug, PartialEq, Eq)]
struct InputRange {
    start: u32,
    len: u32,
}

fn parse_seeds(input: &str) -> Vec<u32> {
    input
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

fn parse_seed_ranges(input: &str) -> Vec<InputRange> {
    let parts = input
        .trim()
        .split_whitespace()
        .skip(1)
        .map(|seed| seed.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    parts
        .chunks(2)
        .map(|chunk| InputRange {
            start: chunk[0],
            len: chunk[1],
        })
        .collect()
}

fn parse_range(input: &str) -> MapRange {
    let mut parts = input.split_whitespace();
    let mut next_part = || parts.next().unwrap().parse::<u32>().unwrap();

    let destination = next_part();
    let source = next_part();
    let len = next_part();

    MapRange {
        destination,
        source,
        len,
    }
}

fn parse_maps(input: &str) -> Vec<Map> {
    input
        .trim()
        .split("\n\n")
        .map(|map| map.lines().skip(1).map(parse_range).collect::<Map>())
        .collect()
}

fn map_input(input: u32, map: &Map) -> u32 {
    for range in map {
        // If input is in range
        if range.contains_source(input) {
            let offset = input - range.source;
            return range.destination + offset;
        }
    }

    // Unmapped correspond to the same destination number
    input
}

fn split_input_range(mut input_range: InputRange, map: &Map) -> Vec<InputRange> {
    // All the points where the input range needs to be split
    let split_points: Vec<u32> = map
        .iter()
        .map(|range| [range.source, range.source + range.len])
        .flatten()
        .collect();

    let mut output_ranges = Vec::new();

    while input_range.len > 0 {
        // Find the next smallest split range inside input range
        let next_split = split_points
            .iter()
            .filter(|&&split| {
                input_range.start < split && (input_range.start + input_range.len) >= split
            })
            .min();

        if let Some(next_split) = next_split {
            let input_range_end = input_range.start + input_range.len - 1;

            // We're done
            if *next_split > input_range_end {
                output_ranges.push(input_range);
                break;
            }

            output_ranges.push(InputRange {
                start: input_range.start,
                len: next_split - input_range.start,
            });

            input_range.len -= next_split - input_range.start;
            input_range.start = *next_split;
        } else {
            output_ranges.push(input_range);
            break;
        }
    }

    output_ranges
}

fn map_input_range(input_range: InputRange, map: &Map) -> Vec<InputRange> {
    let mut input_ranges = split_input_range(input_range, map);

    for range in input_ranges.iter_mut() {
        range.start = map_input(range.start, map);
    }
    input_ranges
}

pub fn part1(input: &str) -> u32 {
    // Parse input
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let mut inputs = parse_seeds(seeds);
    let maps = parse_maps(maps);

    for map in maps {
        for input in inputs.iter_mut() {
            *input = map_input(*input, &map);
        }
    }

    inputs.into_iter().min().unwrap()
}

pub fn part2(input: &str) -> u32 {
    // Parse input
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let mut input_ranges = parse_seed_ranges(seeds);
    let maps = parse_maps(maps);

    for map in maps {
        let mut output_ranges = Vec::new();
        for input_range in input_ranges.into_iter() {
            output_ranges.extend(map_input_range(input_range, &map));
        }
        input_ranges = output_ranges;
    }

    input_ranges.into_iter().map(|range| range.start).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 35);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 46);
    }

    #[test]
    fn test_split_range() {
        // Test with one range inside input
        let input_range = InputRange { start: 1, len: 6 };
        let map = vec![MapRange {
            destination: 0,
            source: 3,
            len: 3,
        }];

        let expected_output = vec![
            InputRange { start: 1, len: 2 },
            InputRange { start: 3, len: 3 },
            InputRange { start: 6, len: 1 },
        ];
        let actual_output = split_input_range(input_range, &map);
        assert_eq!(actual_output, expected_output);

        // Test with two ranges overlapping input start and end
        let input_range = InputRange { start: 1, len: 4 };
        let map = vec![
            MapRange {
                destination: 0,
                source: 0,
                len: 2,
            },
            MapRange {
                destination: 0,
                source: 4,
                len: 2,
            },
        ];
        let expected_output = vec![
            InputRange { start: 1, len: 1 },
            InputRange { start: 2, len: 2 },
            InputRange { start: 4, len: 1 },
        ];
        let actual_output = split_input_range(input_range, &map);
        assert_eq!(actual_output, expected_output);
    }

    #[test]
    fn test_map_range() {
        let input_range = InputRange { start: 10, len: 20 };
        let map = vec![
            MapRange {
                destination: 100,
                source: 10,
                len: 10,
            },
            MapRange {
                destination: 200,
                source: 20,
                len: 10,
            },
        ];

        let expected_output = vec![
            InputRange {
                start: 100,
                len: 10,
            },
            InputRange {
                start: 200,
                len: 10,
            },
        ];
        let actual_output = map_input_range(input_range, &map);
        assert_eq!(actual_output, expected_output);
    }
}
