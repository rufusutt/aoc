fn shoelace_formula(vertices: &[[isize; 2]]) -> usize {
    let n = vertices.len();

    let mut sum = 0;

    for i in 0..n {
        let x1 = vertices[i][0];
        let y1 = vertices[i][1];

        let x2 = vertices[(i + 1) % n][0];
        let y2 = vertices[(i + 1) % n][1];

        sum += x1 * y2 - x2 * y1;
    }

    (sum.abs() / 2) as usize
}

pub fn part1(input: &str) -> String {
    let mut vertices: Vec<[isize; 2]> = Vec::new();

    let mut pos = [0, 0];
    let mut move_count = 0;
    for line in input.trim().lines() {
        let mut parts = line.split_ascii_whitespace();
        let direction = parts.next().unwrap();
        let moves = parts.next().unwrap().parse::<isize>().unwrap();

        match direction {
            "U" => pos[1] -= moves,
            "D" => pos[1] += moves,
            "L" => pos[0] -= moves,
            "R" => pos[0] += moves,
            _ => unreachable!(),
        };

        vertices.push(pos);
        move_count += moves as usize;
    }

    let area = shoelace_formula(&vertices);

    // Use Pick's Theorem to find total squares from boundary and area
    (area + (move_count / 2) + 1).to_string()
}

pub fn part2(input: &str) -> String {
    let mut vertices: Vec<[isize; 2]> = Vec::new();

    let mut pos = [0, 0];
    let mut move_count = 0;
    for line in input.trim().lines() {
        let colour = line.split_ascii_whitespace().last().unwrap();
        let moves_hex = &colour[2..7];
        let direction = &colour[7..8];
        let moves = isize::from_str_radix(moves_hex, 16).unwrap();

        match direction {
            "3" => pos[1] -= moves,
            "1" => pos[1] += moves,
            "2" => pos[0] -= moves,
            "0" => pos[0] += moves,
            _ => unreachable!(),
        };

        vertices.push(pos);
        move_count += moves as usize;
    }

    let area = shoelace_formula(&vertices);

    // Use Pick's Theorem to find total squares from boundary and area
    (area + (move_count / 2) + 1).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
"#;

    #[test]
    fn test_part1() {
        assert_eq!(&part1(TEST_INPUT), "62");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(TEST_INPUT), "952408144115");
    }
}
