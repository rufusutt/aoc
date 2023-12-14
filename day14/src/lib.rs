use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

    fn at(&self, x: usize, y: usize) -> char {
        let index = y * self.width + x;
        self.tiles[index]
    }

    fn at_mut(&mut self, x: usize, y: usize) -> &mut char {
        let index = y * self.width + x;
        &mut self.tiles[index]
    }
}

fn slide_north(map: &mut Map) {
    for start_y in 1..map.height {
        for x in 0..map.width {
            let mut y = start_y;
            while map.at(x, y) == 'O' && map.at(x, y - 1) == '.' {
                *map.at_mut(x, y) = '.';
                *map.at_mut(x, y - 1) = 'O';

                if y > 1 {
                    y -= 1;
                }
            }
        }
    }
}

fn slide_east(map: &mut Map) {
    for start_x in (0..(map.width - 1)).rev() {
        for y in 0..map.height {
            let mut x = start_x;
            while map.at(x, y) == 'O' && map.at(x + 1, y) == '.' {
                *map.at_mut(x, y) = '.';
                *map.at_mut(x + 1, y) = 'O';

                if x < (map.width - 2) {
                    x += 1;
                }
            }
        }
    }
}

fn slide_south(map: &mut Map) {
    for start_y in (0..(map.height - 1)).rev() {
        for x in 0..map.width {
            let mut y = start_y;
            while map.at(x, y) == 'O' && map.at(x, y + 1) == '.' {
                *map.at_mut(x, y) = '.';
                *map.at_mut(x, y + 1) = 'O';

                if y < (map.height - 2) {
                    y += 1;
                }
            }
        }
    }
}

fn slide_west(map: &mut Map) {
    for start_x in 1..map.width {
        for y in 0..map.height {
            let mut x = start_x;
            while map.at(x, y) == 'O' && map.at(x - 1, y) == '.' {
                *map.at_mut(x, y) = '.';
                *map.at_mut(x - 1, y) = 'O';

                if x > 1 {
                    x -= 1;
                }
            }
        }
    }
}

fn compute_load(map: &Map) -> u32 {
    let mut total_load = 0;
    for y in 0..map.height {
        let distance = map.height - y;

        for x in 0..map.width {
            if map.at(x, y) == 'O' {
                total_load += distance as u32;
            }
        }
    }
    total_load
}

pub fn part1(input: &str) -> u32 {
    let mut map = Map::new(input);

    // Slide all rocks up
    slide_north(&mut map);

    // North load
    compute_load(&map)
}

pub fn part2(input: &str) -> u32 {
    let mut map = Map::new(input);

    let mut map_map: HashMap<Map, usize> = HashMap::new();

    for i in 0..1000000000 {
        slide_north(&mut map);
        slide_west(&mut map);
        slide_south(&mut map);
        slide_east(&mut map);

        // Check if the current map state has been seen before
        if let Some(cycle_start) = map_map.get(&map) {
            // If a cycle is found, calculate the cycle length
            let cycle_len = i - cycle_start;

            // Calculate the last iteration within the cycle
            let last = (1000000000 - cycle_start) % cycle_len + cycle_start - 1;

            // Find the map state corresponding to the last iteration in the cycle
            let map = map_map.iter().find(|(_, i)| **i == last).unwrap().0;

            return compute_load(map);
        }

        // If the current map state is new, add it to the HashMap
        map_map.insert(map.clone(), i);
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 136);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 64);
    }
}
