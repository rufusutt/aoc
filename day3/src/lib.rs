use std::collections::HashSet;

struct Schematic {
    inner: Vec<char>,
    width: usize,
    height: usize,
}

impl Schematic {
    fn new(input: &str) -> Self {
        // Trim input
        let input = input.trim();

        // Collect map chars into continuous vec
        let inner: Vec<_> = input.chars().filter(|c| !c.is_whitespace()).collect();

        let width = input.lines().next().expect("Bad input").chars().count();
        let height = inner.len() / width;
        assert_eq!(width * height, inner.len());

        Self {
            inner,
            width,
            height,
        }
    }

    fn at(&self, x: usize, y: usize) -> char {
        assert!(x < self.width);
        assert!(y < self.height);
        let index = y * self.width + x;
        self.inner[index]
    }
}

fn parse_number(
    schematic: &Schematic,
    checked: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) -> Option<u32> {
    // If position already checked
    if checked.contains(&(x, y)) {
        return None;
    }

    // If not a digit
    if !schematic.at(x, y).is_ascii_digit() {
        return None;
    }

    // Find first and last char in part number
    let mut start = x;
    let mut end = x;

    while start > 0 {
        if schematic.at(start - 1, y).is_ascii_digit() {
            start -= 1;
        } else {
            break;
        }
    }

    while end < schematic.width - 1 {
        if schematic.at(end + 1, y).is_ascii_digit() {
            end += 1;
        } else {
            break;
        }
    }

    // Mark positions as checked
    for x in start..=end {
        checked.insert((x, y));
    }

    let number_string: String = (start..=end).map(|x| schematic.at(x, y)).collect();
    Some(number_string.parse::<u32>().unwrap())
}

fn check_surrounding(
    schematic: &Schematic,
    checked: &mut HashSet<(usize, usize)>,
    x: usize,
    y: usize,
) -> Vec<u32> {
    let x_min = x.saturating_sub(1);
    let x_max = (schematic.width - 1).min(x + 1);
    let y_min = y.saturating_sub(1);
    let y_max = (schematic.height - 1).min(y + 1);

    let mut labels = Vec::new();

    for y in y_min..=y_max {
        for x in x_min..=x_max {
            if let Some(label) = parse_number(schematic, checked, x, y) {
                labels.push(label)
            }
        }
    }

    labels
}

pub fn part1(input: &str) -> u32 {
    // Parse input
    let schematic = Schematic::new(input);

    // Keep track of visited positions
    let mut checked = HashSet::new();

    // Solution output
    let mut part_num_sum = 0;

    for y in 0..schematic.height {
        for x in 0..schematic.width {
            let ch = schematic.at(x, y);
            // If character is symbol
            if !ch.is_ascii_digit() && ch != '.' {
                for part_num in check_surrounding(&schematic, &mut checked, x, y) {
                    part_num_sum += part_num;
                }
            }
        }
    }

    part_num_sum
}

pub fn part2(input: &str) -> u32 {
    // Parse input
    let schematic = Schematic::new(input);

    let mut gear_ratio_sum = 0;

    // Find all gears
    for y in 0..schematic.height {
        for x in 0..schematic.width {
            // If character is gear
            if schematic.at(x, y) == '*' {
                let mut checked = HashSet::new();
                let labels = check_surrounding(&schematic, &mut checked, x, y);
                if labels.len() == 2 {
                    gear_ratio_sum += labels.into_iter().product::<u32>();
                }
            }
        }
    }

    gear_ratio_sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 4361);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 467835);
    }
}
