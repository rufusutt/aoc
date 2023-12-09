fn parse_sequences(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

fn build_difference_triangle(sequence: &[i32]) -> Vec<Vec<i32>> {
    let mut differences: Vec<Vec<i32>> = vec![sequence.to_vec()];

    for n in 0..sequence.len() - 1 {
        let last = &differences[n];

        differences.push(
            (0..(sequence.len() - n - 1))
                .map(|i| last[i + 1] - last[i])
                .collect(),
        );
    }

    differences
}

pub fn part1(input: &str) -> u32 {
    let sequences = parse_sequences(input);
    let mut sum = 0;

    for sequence in sequences {
        let mut differences = build_difference_triangle(&sequence);

        // Initial case
        differences[sequence.len() - 1].push(0);

        for i in (0..sequence.len() - 2).rev() {
            let last = *differences[i].last().unwrap();
            let delta = *differences[i + 1].last().unwrap();

            differences[i].push(last + delta);
        }

        sum += differences[0].last().unwrap();
    }

    sum.try_into().unwrap()
}

pub fn part2(input: &str) -> u32 {
    let sequences = parse_sequences(input);
    let mut sum = 0;

    for mut sequence in sequences {
        sequence.reverse();
        let mut differences = build_difference_triangle(&sequence);

        // Initial case
        differences[sequence.len() - 1].push(0);

        for i in (0..sequence.len() - 2).rev() {
            let last = *differences[i].last().unwrap();
            let delta = *differences[i + 1].last().unwrap();

            differences[i].push(last + delta);
        }

        sum += differences[0].last().unwrap();
    }

    sum.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}
