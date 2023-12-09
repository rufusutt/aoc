fn parse_sequences(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect()
}

/// Generates coefficients for the nth difference operation based on Pascal's triangle.
///
/// Given an integer `n`, this function computes the coefficients used in the formula for the nth
/// difference operation. The formula involves binomial coefficients with alternating signs,
/// resembling a signed version of Pascal's triangle.
fn generate_coefficients(n: usize) -> Vec<i32> {
    let mut coefficients = vec![1];

    for k in 1..=n {
        let next_coefficient = coefficients[k - 1] * (n as i32 - k as i32 + 1) / k as i32;
        coefficients.push(next_coefficient);
    }

    // Apply alternating signs
    for coefficient in coefficients.iter_mut().skip(1).step_by(2) {
        *coefficient *= -1;
    }

    coefficients
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
fn interpolate_sequence(sequence: &[i32]) -> i32 {
    let coefficients = generate_coefficients(sequence.len());

    let interpolation = sequence
        .iter()
        .zip(coefficients.iter().rev())
        .map(|(x, coeff)| x * coeff)
        .sum::<i32>();

    interpolation * -1
}

pub fn part1(input: &str) -> u32 {
    let sequences = parse_sequences(input);

    sequences
        .into_iter()
        .map(|s| interpolate_sequence(&s))
        .sum::<i32>()
        .try_into()
        .expect("Runner needs unsigned answer")
}

pub fn part2(input: &str) -> u32 {
    let mut sequences = parse_sequences(input);

    // Reverse all sequences
    sequences.iter_mut().for_each(|s| s.reverse());

    sequences
        .into_iter()
        .map(|s| interpolate_sequence(&s))
        .sum::<i32>()
        .try_into()
        .expect("Runner needs unsigned answer")
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
