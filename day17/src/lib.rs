use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashSet},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map {
    tiles: Vec<u32>,
    width: usize,
    height: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let tiles: Vec<_> = input
            .trim()
            .lines()
            .flat_map(|line| line.chars())
            .map(|ch| ch.to_digit(10).unwrap())
            .collect();

        let width = input.trim().lines().next().unwrap().len();
        let height = tiles.len() / width;
        assert_eq!(width * height, tiles.len());

        Self {
            tiles,
            width,
            height,
        }
    }

    fn get(&self, x: isize, y: isize) -> Option<u32> {
        if x < 0 || y < 0 || x as usize >= self.width || y as usize >= self.height {
            return None;
        }
        let index = y as usize * self.width + x as usize;
        Some(self.tiles[index])
    }
}

#[derive(Clone, PartialEq, Eq)]
struct State {
    cost: u32,
    pos: [isize; 2],
    dir: [isize; 2],
    dist: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(map: &Map, min: usize, max: usize) -> u32 {
    // Construct heap with initial state
    let mut heap = BinaryHeap::from_iter([[0, 1], [1, 0]].map(|velocity| State {
        cost: 0,
        pos: [0, 0],
        dir: velocity,
        dist: 0,
    }));

    let mut visited = HashSet::new();

    while let Some(State {
        cost,
        pos: [x, y],
        dir: [dx, dy],
        dist,
    }) = heap.pop()
    {
        if x as usize == map.width - 1 && y as usize == map.height - 1 && dist >= min {
            return cost;
        }

        if !visited.insert((x, y, dx, dy, dist)) {
            continue;
        }

        let moves = [[-dy, dx], [dy, -dx], [dx, dy]];
        for &[dx2, dy2] in &moves[2 * usize::from(dist < min)..3 - usize::from(dist >= max)] {
            if let Some(move_cost) = map.get(x + dx2, y + dy2) {
                heap.push(State {
                    cost: cost + move_cost,
                    pos: [x + dx2, y + dy2],
                    dir: [dx2, dy2],
                    dist: usize::from([dx, dy] == [dx2, dy2]) * dist + 1,
                });
            }
        }
    }

    unreachable!()
}

pub fn part1(input: &str) -> u32 {
    let map = Map::new(input);
    dijkstra(&map, 0, 3)
}

pub fn part2(input: &str) -> u32 {
    let map = Map::new(input);
    dijkstra(&map, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 102);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 94);
    }
}
