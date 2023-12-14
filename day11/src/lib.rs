#[derive(Debug)]
struct Image {
    galaxies: Vec<(usize, usize)>,
    width: usize,
    height: usize,
}

impl Image {
    fn new(input: &str) -> Self {
        let data = || input.trim().lines().map(|line| line.chars()).flatten();

        let width = input.trim().lines().next().unwrap().len();
        let height = data().count() / width;

        let galaxies: Vec<_> = data()
            .enumerate()
            .filter_map(|(i, d)| {
                if d == '#' {
                    Some((i % width, i / width))
                } else {
                    None
                }
            })
            .collect();

        Self {
            galaxies,
            width,
            height,
        }
    }

    fn row(&self, index: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.galaxies
            .iter()
            .filter(move |(_, y)| index == *y)
            .copied()
    }

    fn column(&self, index: usize) -> impl Iterator<Item = (usize, usize)> + '_ {
        self.galaxies
            .iter()
            .filter(move |(x, _)| index == *x)
            .copied()
    }

    fn insert_rows(&mut self, index: usize, count: usize) {
        for (x, y) in self.galaxies.iter_mut() {
            if *y >= index {
                *y += count;
            }
        }
        self.height += count;
    }

    fn insert_columns(&mut self, index: usize, count: usize) {
        for (x, y) in self.galaxies.iter_mut() {
            if *x >= index {
                *x += count;
            }
        }
        self.width += count;
    }

    fn expand(&mut self, factor: usize) {
        // Find empty rows and columns
        let empty_rows: Vec<_> = (0..self.height)
            .filter(|row| self.row(*row).count() == 0)
            .collect();
        let empty_cols: Vec<_> = (0..self.width)
            .filter(|col| self.column(*col).count() == 0)
            .collect();

        // Expand
        for row in empty_rows.iter().rev() {
            self.insert_rows(*row, factor);
        }
        for col in empty_cols.iter().rev() {
            self.insert_columns(*col, factor);
        }
    }
}

fn distance(a: (usize, usize), b: (usize, usize)) -> usize {
    let (x1, y1) = a;
    let (x2, y2) = b;

    (x2 as isize - x1 as isize).abs() as usize + (y2 as isize - y1 as isize).abs() as usize
}

fn sum_of_distances(input: &str, expansion: usize) -> u32 {
    let mut image = Image::new(input);

    // Adjust for exapansion
    image.expand(expansion - 1);

    let mut sum = 0;

    // For each pair of galaxies
    for i in 0..image.galaxies.len() {
        for j in (i + 1)..image.galaxies.len() {
            let g1 = image.galaxies[i];
            let g2 = image.galaxies[j];

            sum += distance(g1, g2);
        }
    }

    // TODO: Runner needs to expect more than 32 bits
    println!("Sum: {}", sum);
    sum.try_into().expect("Result too large")
}

pub fn part1(input: &str) -> u32 {
    sum_of_distances(input, 2)
}

pub fn part2(input: &str) -> u32 {
    sum_of_distances(input, 1000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
"#;

    const EXPANDED_INPUT: &str = r#"
....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......    
"#;

    #[test]
    fn test_distance() {
        assert_eq!(distance((1, 6), (5, 11)), 9);
        assert_eq!(distance((4, 0), (9, 10)), 15);
        assert_eq!(distance((0, 2), (12, 7)), 17);
        assert_eq!(distance((0, 11), (5, 11)), 5);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 374);
    }

    #[test]
    fn test_part2() {
        assert_eq!(sum_of_distances(TEST_INPUT, 10), 1030);
        assert_eq!(sum_of_distances(TEST_INPUT, 100), 8410);
    }
}
