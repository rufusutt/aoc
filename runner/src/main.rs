use std::{
    process::ExitCode,
    time::{Duration, Instant},
};

const REPEATS: u32 = 5;

struct Solution {
    part1: Option<fn(input: &str) -> u32>,
    part2: Option<fn(input: &str) -> u32>,
}

macro_rules! solutions {
    ($($part1:expr, $part2:expr),* $(,)?) => {
        [
            $(
                Solution { part1: $part1, part2: $part2 }
            ),*
        ]
    };
}

#[rustfmt::skip]
const SOLUTIONS: [Solution; 25] = solutions!(
    Some(day1::part1), Some(day1::part2),
    Some(day2::part1), Some(day2::part2),
    Some(day3::part1), Some(day3::part2),
    Some(day4::part1), Some(day4::part2),
    Some(day5::part1), Some(day5::part2),
    Some(day6::part1), Some(day6::part2),
    Some(day7::part1), Some(day7::part2),
    Some(day8::part1), Some(day8::part2),
    Some(day9::part1), Some(day9::part2),
    Some(day10::part1), Some(day10::part2),
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
    None, None,
);

fn print_usage(name: &str) {
    eprintln!("Usage: {} <DAY> <INPUT>", name);
}

fn run_part(input: &str, func: Option<fn(input: &str) -> u32>) {
    if let Some(func) = func {
        let start = Instant::now();

        // Run solution REPEAT times
        let solution = (0..REPEATS)
            .map(|_| func(input))
            .last()
            .expect("REPEATS must be greater than 0");

        let duration = start.elapsed() / REPEATS;

        println!("Solution: {}", solution);

        print_elapsed_time(duration);
    } else {
        println!("Not implemented!");
    }
}

fn print_elapsed_time(duration: Duration) {
    if duration < Duration::from_millis(1) {
        let micros = duration.as_micros();
        let nanos = duration.as_nanos() - micros * 1000;
        println!("Elapsed: {}.{:03} us", micros, nanos);
    } else if duration < Duration::from_secs(1) {
        let millis = duration.as_millis();
        let micros = duration.as_micros() - millis * 1000;
        println!("Elapsed: {}.{:03} ms", millis, micros);
    } else {
        let seconds = duration.as_secs();
        let millis = duration.as_millis() - seconds as u128 * 1000;
        println!("Elapsed: {}.{:03} secs", seconds, millis);
    }
}

fn main() -> Result<ExitCode, std::io::Error> {
    // Unpack arguments
    let args: Vec<String> = std::env::args().collect();
    let Some(day) = args.get(1) else {
        eprintln!("Missing day");
        print_usage(&args[0]);
        return Ok(ExitCode::FAILURE);
    };
    let Some(input_path) = args.get(2) else {
        eprintln!("Missing input");
        print_usage(&args[0]);
        return Ok(ExitCode::FAILURE);
    };

    // Validate inputs
    let Ok(day) = day.parse::<usize>() else {
        eprintln!("Invalid day: {}", day);
        return Ok(ExitCode::FAILURE);
    };
    if !(1..=25).contains(&day) {
        eprintln!("ಠ_ಠ: {}", day);
        return Ok(ExitCode::FAILURE);
    }

    // Get solution from table
    let solution = &SOLUTIONS[day - 1];

    // Read input into memory
    let input = std::fs::read_to_string(input_path)?;

    println!("==== Part 1 ====");
    run_part(&input, solution.part1);

    println!("==== Part 2 ====");
    run_part(&input, solution.part2);

    Ok(ExitCode::SUCCESS)
}
