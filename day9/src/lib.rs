fn parse_sequence(line: &str, buf: &mut Vec<i64>) {
    buf.clear();
    buf.extend(
        line.split_ascii_whitespace()
            .map(|x| x.parse::<i64>().unwrap()),
    );
}

/// Generates coefficients for the nth difference operation based on Pascal's triangle.
///
/// Given an integer `n`, this function computes the coefficients used in the formula for the nth
/// difference operation. The formula involves binomial coefficients with alternating signs,
/// resembling a signed version of Pascal's triangle.
fn generate_coefficients(n: usize, coefficient_buf: &mut Vec<i64>) {
    coefficient_buf.clear();
    coefficient_buf.push(1);

    for k in 1..=n {
        let next_coefficient = coefficient_buf[k - 1] * (n as i64 - k as i64 + 1) / k as i64;
        coefficient_buf.push(next_coefficient);
    }

    // Apply alternating signs
    for coefficient in coefficient_buf.iter_mut().skip(1).step_by(2) {
        *coefficient *= -1;
    }
}

/// Interpolates a sequence of equally spaced points using a polynomial of degree at most n,
/// where n is the length of the input sequence minus 1.
/// The interpolation is based on the method of finite differences.
///
/// A polynomial y(x) of degree d defines a sequence of values at positive integer points,
/// y_j = y(j), and the (d + 1)th difference of this sequence is identically zero:
/// Δ^(d+1)y = 0.
///
/// Thus, given values y_0, ..., y_n at equally spaced points, where n = d + 1, we have:
/// (-1)^n * y_0 + (-1)^(n-1) * C(n,1) * y_1 + ... - C(n,n-1) * y_(n-1) + y_n = 0.
fn interpolate_sequence(sequence: &[i64], coefficient_buf: &mut Vec<i64>) -> i64 {
    generate_coefficients(sequence.len(), coefficient_buf);

    let interpolation = sequence
        .iter()
        .zip(coefficient_buf.iter().rev())
        .map(|(x, coeff)| x * coeff)
        .sum::<i64>();

    -interpolation
}

pub fn part1(input: &str) -> u32 {
    // Allocated single buffers to be reused
    let mut sequence_buf = Vec::new();
    let mut coefficient_buf = Vec::new();

    let ans = input
        .trim()
        .lines()
        .map(|line| {
            parse_sequence(line, &mut sequence_buf);
            interpolate_sequence(&sequence_buf, &mut coefficient_buf)
        })
        .sum::<i64>();

    // TODO
    if let Ok(ans) = ans.try_into() {
        ans
    } else {
        // println!("{}", ans);
        0
    }
}

pub fn part2(input: &str) -> u32 {
    // Allocated single buffers to be reused
    let mut sequence_buf = Vec::new();
    let mut coefficient_buf = Vec::new();

    let ans = input
        .trim()
        .lines()
        .map(|line| {
            parse_sequence(line, &mut sequence_buf);
            sequence_buf.reverse();
            interpolate_sequence(&sequence_buf, &mut coefficient_buf)
        })
        .sum::<i64>();

    // TODO
    if let Ok(ans) = ans.try_into() {
        ans
    } else {
        // println!("{}", ans);
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 114);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 2);
    }
}
