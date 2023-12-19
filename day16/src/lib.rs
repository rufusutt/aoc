use std::collections::HashSet;

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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn raytrace(map: &Map, x: isize, y: isize, direction: Direction) -> Vec<(isize, isize, Direction)> {
    return match map.at(x as usize, y as usize) {
        '.' => match direction {
            Direction::North => vec![(x, y - 1, Direction::North)],
            Direction::East => vec![(x + 1, y, Direction::East)],
            Direction::South => vec![(x, y + 1, Direction::South)],
            Direction::West => vec![(x - 1, y, Direction::West)],
        },
        '/' => match direction {
            Direction::North => vec![(x + 1, y, Direction::East)],
            Direction::East => vec![(x, y - 1, Direction::North)],
            Direction::South => vec![(x - 1, y, Direction::West)],
            Direction::West => vec![(x, y + 1, Direction::South)],
        },
        '\\' => match direction {
            Direction::North => vec![(x - 1, y, Direction::West)],
            Direction::East => vec![(x, y + 1, Direction::South)],
            Direction::South => vec![(x + 1, y, Direction::East)],
            Direction::West => vec![(x, y - 1, Direction::North)],
        },
        '|' => match direction {
            Direction::East | Direction::West => {
                vec![(x, y - 1, Direction::North), (x, y + 1, Direction::South)]
            }
            Direction::North => vec![(x, y - 1, Direction::North)],
            Direction::South => vec![(x, y + 1, Direction::South)],
        },
        '-' => match direction {
            Direction::North | Direction::South => {
                vec![(x - 1, y, Direction::West), (x + 1, y, Direction::East)]
            }
            Direction::East => vec![(x + 1, y, Direction::East)],
            Direction::West => vec![(x - 1, y, Direction::West)],
        },
        _ => todo!(),
    };
}

fn count_energised(map: &Map, start: (isize, isize, Direction)) -> usize {
    let mut ray_set = HashSet::new();

    let mut queue = vec![start];

    while let Some((x, y, direction)) = queue.pop() {
        // Mark as visited
        ray_set.insert((x, y, direction));

        for (next_x, next_y, next_direction) in raytrace(&map, x, y, direction).into_iter() {
            // If out of bounds
            if next_x < 0
                || next_y < 0
                || next_x as usize == map.width
                || next_y as usize == map.height
            {
                continue;
            }

            // If not yet visited
            if !ray_set.contains(&(next_x, next_y, next_direction)) {
                queue.push((next_x, next_y, next_direction));
            }
        }
    }

    let energised_set: HashSet<_> = ray_set.into_iter().map(|(x, y, _)| (x, y)).collect();
    energised_set.len()
}

pub fn part1(input: &str) -> u32 {
    let map = Map::new(input);
    count_energised(&map, (0, 0, Direction::East)) as u32
}

pub fn part2(input: &str) -> u32 {
    let map = Map::new(input);

    let mut max_energised = 0;

    for x in 0..(map.width as isize) {
        max_energised = max_energised.max(count_energised(&map, (x, 0, Direction::South)));
        max_energised = max_energised.max(count_energised(
            &map,
            (x, map.height as isize - 1, Direction::North),
        ));
    }

    for y in 0..(map.width as isize) {
        max_energised = max_energised.max(count_energised(&map, (0, y, Direction::East)));
        max_energised = max_energised.max(count_energised(
            &map,
            (map.width as isize - 1, y, Direction::West),
        ));
    }

    max_energised as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 46);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 51);
    }
}
