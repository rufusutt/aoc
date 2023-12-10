use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<char>,
    width: usize,
    height: usize,
}

impl Map {
    const CORNERS: [char; 4] = ['L', 'J', '7', 'F'];
    const NORTH_TILES: [char; 4] = ['S', '|', 'L', 'J'];
    const EAST_TILES: [char; 4] = ['S', '-', 'L', 'F'];
    const SOUTH_TILES: [char; 4] = ['S', '|', '7', 'F'];
    const WEST_TILES: [char; 4] = ['S', '-', 'J', '7'];

    fn new(input: &str) -> Map {
        let tiles: Vec<_> = input
            .trim()
            .lines()
            .map(|line| line.chars())
            .flatten()
            .collect();

        // Get map width
        let width = input.trim().lines().next().unwrap().chars().count();
        let height = tiles.len() / width;

        assert_eq!(width * height, tiles.len());

        Map {
            tiles,
            width,
            height,
        }
    }

    fn at(&self, pos: Position) -> char {
        let index = pos.y * self.width + pos.x;
        self.tiles[index]
    }

    fn start(&self) -> Position {
        // Find start tile
        let position = self.tiles.iter().position(|t| *t == 'S').unwrap();

        let x = position % self.width;
        let y = position / self.width;

        Position { x, y }
    }

    fn north(&self, pos: Position) -> Option<Position> {
        if pos.y == 0 {
            return None;
        }
        let next = Position {
            x: pos.x,
            y: pos.y - 1,
        };

        if Self::NORTH_TILES.contains(&self.at(pos)) && Self::SOUTH_TILES.contains(&self.at(next)) {
            Some(next)
        } else {
            None
        }
    }

    fn east(&self, pos: Position) -> Option<Position> {
        if pos.x == self.width - 1 {
            return None;
        }
        let next = Position {
            x: pos.x + 1,
            y: pos.y,
        };

        if Self::EAST_TILES.contains(&self.at(pos)) && Self::WEST_TILES.contains(&self.at(next)) {
            Some(next)
        } else {
            None
        }
    }

    fn south(&self, pos: Position) -> Option<Position> {
        if pos.y == self.height - 1 {
            return None;
        }
        let next = Position {
            x: pos.x,
            y: pos.y + 1,
        };

        if Self::SOUTH_TILES.contains(&self.at(pos)) && Self::NORTH_TILES.contains(&self.at(next)) {
            Some(next)
        } else {
            None
        }
    }

    fn west(&self, pos: Position) -> Option<Position> {
        if pos.x == 0 {
            return None;
        }
        let next = Position {
            x: pos.x - 1,
            y: pos.y,
        };

        if Self::WEST_TILES.contains(&self.at(pos)) && Self::EAST_TILES.contains(&self.at(next)) {
            Some(next)
        } else {
            None
        }
    }

    fn edges(&self, pos: Position) -> impl Iterator<Item = Position> {
        [
            self.north(pos),
            self.east(pos),
            self.south(pos),
            self.west(pos),
        ]
        .into_iter()
        .filter_map(|pos| pos)
    }
}

pub fn part1(input: &str) -> u32 {
    let map = Map::new(input);

    // Get start position
    let start_pos = map.start();

    // Initial conditions
    let mut queue: VecDeque<(Position, u32)> = VecDeque::from_iter([(start_pos, 0)]);
    let mut visited: HashSet<Position> = HashSet::from_iter([start_pos]);

    // Track max steps
    let mut max_steps = 0;

    while let Some((pos, steps)) = queue.pop_front() {
        // Update max
        max_steps = max_steps.max(steps);

        // Explore new edges
        for edge in map.edges(pos) {
            if !visited.contains(&edge) {
                visited.insert(edge);
                queue.push_back((edge, steps + 1));
            }
        }
    }

    max_steps
}

fn find_loop(map: &Map, start: Position) -> Vec<Position> {
    // Initial conditions
    let mut queue: Vec<(Position, Vec<Position>)> = Vec::from_iter([(start, vec![start])]);
    let mut visited = HashSet::new();

    while let Some((next, path)) = queue.pop() {
        // Check position has not been visited yet
        if visited.contains(&next) {
            // If returned to start
            if next == start && path.len() > 3 {
                return path;
            }
            continue;
        }

        // Insert into visited list
        visited.insert(next);

        for next in map.edges(next) {
            let mut next_path = path.clone();
            next_path.push(next);
            queue.push((next, next_path));
        }
    }

    panic!("No path")
}

fn shoelace_formula(corners: &[Position]) -> usize {
    let n = corners.len();

    let mut sum = 0;

    for i in 0..n {
        let x1 = corners[i].x as isize;
        let y1 = corners[i].y as isize;

        let x2 = corners[(i + 1) % n].x as isize;
        let y2 = corners[(i + 1) % n].y as isize;

        sum += x1 * y2 - x2 * y1;
    }

    (sum.abs() / 2) as usize
}

pub fn part2(input: &str) -> u32 {
    let map = Map::new(input);

    // Get start position
    let start_pos = map.start();

    // Find loop
    let path = find_loop(&map, start_pos);

    let boundary_count = path.len() - 1;

    dbg!(boundary_count);

    // Find all corners
    let mut corners: Vec<_> = path
        .into_iter()
        .filter(|pos| Map::CORNERS.contains(&map.at(*pos)))
        .collect();

    // TODO: Case when start not a corner
    corners.insert(0, start_pos);
    corners.push(start_pos);

    let area = shoelace_formula(&corners);

    // Use Pick's theorem to find number of integer points inside polygon
    (area - (boundary_count / 2) + 1) as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r#"
.....
.S-7.
.|.|.
.L-J.
.....
"#;

    const TEST_INPUT_2: &str = r#"
..F7.
.FJ|.
SJ.L7
|F--J
LJ...
"#;

    const TEST_INPUT_3: &str = r#"
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
"#;

    const TEST_INPUT_4: &str = r#"
..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
"#;

const TEST_INPUT_5: &str = r#"
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
"#;

const TEST_INPUT_6: &str = r#"
FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
"#;


    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 4);
        assert_eq!(part1(TEST_INPUT_2), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_3), 4);
        assert_eq!(part2(TEST_INPUT_4), 4);
        assert_eq!(part2(TEST_INPUT_5), 8);
        assert_eq!(part2(TEST_INPUT_6), 10);
    }
}
