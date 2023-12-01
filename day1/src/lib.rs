const DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn parse_ascii_digit(digit: char) -> u32 {
    assert!(digit.is_ascii_digit());
    (digit as u8 - '0' as u8) as u32
}

fn parse_digit(string: &str) -> Option<u32> {
    // If numeric digit
    let first_char = string.chars().next().expect("Empty window");
    if first_char.is_ascii_digit() {
        return Some(parse_ascii_digit(first_char));
    }

    for (i, digit) in DIGITS.iter().enumerate() {
        if string.starts_with(digit) {
            return Some(i as u32);
        }
    }

    None
}

fn find_first_digit(string: &str) -> u32 {
    for (i, _) in string.char_indices() {
        let substr = &string[i..];
        if let Some(digit) = parse_digit(substr) {
            return digit;
        }
    }
    panic!()
}

fn find_last_digit(string: &str) -> u32 {
    for (i, _) in string.char_indices().rev() {
        let substr = &string[i..];
        if let Some(digit) = parse_digit(substr) {
            return digit;
        }
    }
    panic!()
}

pub fn part1(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            // Get digits
            let digits = || line.chars().filter(|c| c.is_ascii_digit());

            let first = digits().next().expect("No digits");
            let last = digits().last().expect("No remaining digits");

            // Convert to numbers
            let first = parse_ascii_digit(first);
            let last = parse_ascii_digit(last);
            first * 10 + last
        })
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            let first = find_first_digit(line);
            let last = find_last_digit(line);

            first * 10 + last
        })
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = r#"
1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
"#;

        assert_eq!(part1(input), 142);
    }

    #[test]
    fn test_part2() {
        let input = r#"
two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
"#;

        assert_eq!(part2(input), 281);
    }
}
