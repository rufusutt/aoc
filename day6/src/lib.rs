#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn parse_races(input: &str) -> Vec<Race> {
    let mut lines = input.trim().lines();
    let times = lines.next().unwrap().split_whitespace().skip(1);
    let distances = lines.next().unwrap().split_whitespace().skip(1);

    times
        .zip(distances)
        .map(|(time, distance)| Race {
            time: time.parse::<u64>().unwrap(),
            distance: distance.parse::<u64>().unwrap(),
        })
        .collect()
}

fn parse_single_race(input: &str) -> Race {
    let mut lines = input.trim().lines();
    let time: String = lines.next().unwrap().split_whitespace().skip(1).collect();
    let distance: String = lines.next().unwrap().split_whitespace().skip(1).collect();

    Race {
        time: time.parse::<u64>().unwrap(),
        distance: distance.parse::<u64>().unwrap(),
    }
}

fn satisfies_constraints(race: &Race, charge_time: u64) -> bool {
    // Solved using inequalites
    // charge_time <= race_time
    // charge_time + distance / charge_time <= race_time
    (charge_time * race.time) > (race.distance + charge_time.pow(2))
}

fn find_start_end_values(race: &Race) -> (u64, u64) {
    // Binary search for first satisfied constraint
    let mut low = 1;
    let mut high = race.time;

    while low < high {
        let mid = low + (high - low) / 2;
        if satisfies_constraints(race, mid) {
            high = mid;
        } else {
            low = mid + 1;
        }
    }
    let start = low;

    // Reset and perform search for last satisfied constraint
    low = start;
    high = race.time;

    while low < high {
        let mid = low + (high - low) / 2;
        if satisfies_constraints(race, mid) {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    let end = low - 1;

    (start, end)
}

pub fn part1(input: &str) -> String {
    let races = parse_races(input);

    races
        .into_iter()
        .map(|race| {
            let (start, end) = find_start_end_values(&race);
            (end - start) + 1
        })
        .product::<u64>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let race = parse_single_race(input);

    let (start, end) = find_start_end_values(&race);
    ((end - start) + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Time:      7  15   30
Distance:  9  40  200
"#;

    #[test]
    fn test_part1() {
        assert_eq!(&part1(TEST_INPUT), "288");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(TEST_INPUT), "71503");
    }
}
