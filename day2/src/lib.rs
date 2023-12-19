#[derive(Default)]
struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

struct Game {
    id: u32,
    rounds: Vec<Round>,
}

fn parse_round(input: &str) -> Round {
    let mut round = Round::default();

    for pick in input.split(',').map(|pick| pick.trim()) {
        let (count, colour) = pick.split_once(' ').unwrap();
        let count = count.parse::<u32>().expect("Invalid number");

        match colour {
            "red" => round.red = count,
            "green" => round.green = count,
            "blue" => round.blue = count,
            _ => panic!("Unknown colour"),
        }
    }
    round
}

fn parse_game(input: &str) -> Game {
    let (game, rounds) = input.split_once(':').unwrap();

    // Extract game ID
    let id = game
        .split_whitespace()
        .last()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    // Parse rounds separated by ';'
    let rounds: Vec<_> = rounds.split(';').map(parse_round).collect();

    Game { id, rounds }
}

fn game_possible(game: &Game) -> bool {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;

    !game
        .rounds
        .iter()
        .any(|r| r.red > MAX_RED || r.green > MAX_GREEN || r.blue > MAX_BLUE)
}

pub fn part1(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_game)
        .filter(game_possible)
        .map(|game| game.id)
        .sum::<u32>()
        .to_string()
}

pub fn part2(input: &str) -> String {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(parse_game)
        .map(|game| {
            let rounds = || game.rounds.iter();

            let max_red = rounds().map(|r| r.red).max().unwrap_or(0);
            let max_green = rounds().map(|r| r.green).max().unwrap_or(0);
            let max_blue = rounds().map(|r| r.blue).max().unwrap_or(0);

            max_red * max_green * max_blue
        })
        .sum::<u32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;

    #[test]
    fn test_part1() {
        assert_eq!(&part1(TEST_INPUT), "8");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(TEST_INPUT), "2286");
    }
}
