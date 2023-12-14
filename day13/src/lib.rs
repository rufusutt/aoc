struct Map {
    tiles: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let tiles: Vec<_> = input.trim().lines().flat_map(|line| line.chars()).collect();

        let width = input.trim().lines().next().unwrap().len();
        let height = tiles.len() / width;
        assert_eq!(width * height, tiles.len());

        Self {
            tiles,
            width,
            height,
        }
    }

    fn at(&mut self, x: usize, y: usize) -> &mut char {
        let index = (y - 1) * self.width + (x - 1);
        &mut self.tiles[index]
    }

    fn row(&self, index: usize) -> impl Iterator<Item = char> + '_ {
        self.tiles
            .iter()
            .skip((index - 1) * self.width)
            .take(self.width)
            .copied()
    }

    fn column(&self, index: usize) -> impl Iterator<Item = char> + '_ {
        self.tiles
            .iter()
            .skip(index - 1)
            .step_by(self.width)
            .copied()
    }
}

fn find_reflections(map: &Map, is_horizontal: bool) -> Vec<usize> {
    let mut reflections = Vec::new();
    let dimension = if is_horizontal { map.height } else { map.width };

    for start_left in 1..dimension {
        let start_right = start_left + 1;

        let mut left = start_left;
        let mut right = start_right;

        while left > 0 && right < dimension + 1 {
            let is_equal = if is_horizontal {
                map.row(left).eq(map.row(right))
            } else {
                map.column(left).eq(map.column(right))
            };

            if is_equal {
                left -= 1;
                right += 1;
            } else {
                break;
            }
        }

        if left == 0 || right == dimension + 1 {
            reflections.push(start_left);
        }
    }

    reflections
}

fn find_horizontal_reflections(map: &Map) -> Vec<usize> {
    find_reflections(map, true)
}

fn find_vertical_reflections(map: &Map) -> Vec<usize> {
    find_reflections(map, false)
}

pub fn part1(input: &str) -> u32 {
    let mut sum_left_columns = 0;
    let mut sum_above_rows = 0;

    for map in input.trim().split("\n\n") {
        let map = Map::new(map);

        if let Some(index) = find_vertical_reflections(&map).first() {
            sum_left_columns += *index as u32;
        } else if let Some(index) = find_horizontal_reflections(&map).first() {
            sum_above_rows += *index as u32;
        }
    }

    sum_left_columns + (100 * sum_above_rows)
}

fn find_smudge_reflection(map: &mut Map) -> (usize, usize) {
    let original_vertical = find_vertical_reflections(map).first().copied().unwrap_or(0);
    let original_horizontal = find_horizontal_reflections(map)
        .first()
        .copied()
        .unwrap_or(0);

    for y in 1..=map.height {
        for x in 1..=map.width {
            let original = *map.at(x, y);
            let flipped = if original == '#' { '.' } else { '#' };

            // Flip
            *map.at(x, y) = flipped;

            for index in find_vertical_reflections(map) {
                if index != original_vertical {
                    return (index, 0);
                }
            }

            for index in find_horizontal_reflections(map) {
                if index != original_horizontal {
                    return (0, index);
                }
            }

            // Reset
            *map.at(x, y) = original;
        }
    }

    panic!("No reflection")
}

pub fn part2(input: &str) -> u32 {
    let mut sum_left_columns = 0;
    let mut sum_above_rows = 0;

    for map in input.trim().split("\n\n") {
        let mut map = Map::new(map);
        let (vertical, horizontal) = find_smudge_reflection(&mut map);
        sum_left_columns += vertical as u32;
        sum_above_rows += horizontal as u32;
    }

    sum_left_columns + (100 * sum_above_rows)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 405);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 400);
    }
}
