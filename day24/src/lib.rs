use nalgebra::{Matrix2, Vector2, Vector3};

#[derive(Debug, PartialEq)]
struct Line {
    p: Vector3<f64>,
    v: Vector3<f64>,
}

impl Line {
    fn at(&self, t: f64) -> Vector3<f64> {
        self.p + t * self.v
    }
}

fn parse_lines(input: &str) -> Vec<Line> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (p, v) = line.split_once('@').unwrap();
            let mut parts = p.split(", ").map(|p| p.trim().parse().unwrap());
            let p = Vector3::new(
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            );
            let mut parts = v.split(", ").map(|v| v.trim().parse().unwrap());
            let v = Vector3::new(
                parts.next().unwrap(),
                parts.next().unwrap(),
                parts.next().unwrap(),
            );
            Line { p, v }
        })
        .collect()
}

fn intersection_xy(l1: &Line, l2: &Line) -> Option<Vector2<f64>> {
    // Get parts
    let p1 = l1.p.xy();
    let v1 = l1.v.xy();
    let p2 = l2.p.xy();
    let v2 = l2.v.xy();

    // v1 - v2
    let v1v2 = Matrix2::from_columns(&[v1, -v2]);
    let v1v2_inverse = v1v2.try_inverse()?;

    Some(v1v2_inverse * (p2 - p1))
}

fn count_intersections(lines: &[Line], start: isize, stop: isize) -> usize {
    let mut count = 0;

    // For every combination of lines
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let l1 = &lines[i];
            let l2 = &lines[j];

            // Same line
            if l1 == l2 {
                continue;
            }

            if let Some(t) = intersection_xy(l1, l2) {
                // Find intersect point
                let intersect = l1.at(t.x).xy();

                // Intersection must be in specified range
                if intersect
                    .iter()
                    .any(|&c| c < start as f64 || c > stop as f64)
                {
                    continue;
                }

                // Must occur in future
                if t.iter().any(|&t| t < 0.0) {
                    continue;
                }

                count += 1;
            }
        }
    }

    count
}

pub fn part1(input: &str) -> String {
    let lines = parse_lines(input);
    count_intersections(&lines, 200000000000000, 400000000000000).to_string()
}

pub fn part2(input: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
19, 13, 30 @ -2,  1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @  1, -5, -3
"#;

    #[test]
    fn test_part1() {
        let lines = parse_lines(TEST_INPUT);
        assert_eq!(count_intersections(&lines, 7, 27), 2);
    }
}
