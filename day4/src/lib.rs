fn count_wins(card: &str) -> usize {
    let (winning, mine) = card.split_once('|').expect("Bad input");

    let mine = mine
        .split_whitespace()
        .map(|x| x.parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    winning
        .split_whitespace()
        .skip(2)
        .map(|x| x.parse::<u32>().unwrap())
        .filter(|x| mine.contains(x))
        .count()
}

pub fn part1(input: &str) -> String {
    input
        .trim()
        .lines()
        .map(|card| {
            let n = count_wins(card) as u32;

            if n > 0 {
                2u32.pow(n - 1)
            } else {
                0
            }
        })
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    let n = input.trim().lines().count();

    // DP: Iterate through lines in reverse to build solution
    let mut win_table = vec![1; n];

    for card in input.trim().lines().rev().skip(1) {
        let (card_num, _) = card.split_once(':').unwrap();

        let card_num = card_num
            .split_whitespace()
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let wins = count_wins(card);

        for winning_card_num in card_num + 1..=(card_num + wins).min(n) {
            win_table[card_num - 1] += win_table[winning_card_num - 1];
        }
    }

    win_table.into_iter().sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"#;

    #[test]
    fn test_part1() {
        assert_eq!(&part1(TEST_INPUT), "13");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(TEST_INPUT), "30");
    }
}
