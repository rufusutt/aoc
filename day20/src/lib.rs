use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Pulse {
    Low,
    High,
}

enum Logic<'a> {
    Broadcast,
    FlipFlop {
        on: bool,
    },
    Conjunction {
        input_states: HashMap<&'a str, Pulse>,
    },
}

struct Module<'a> {
    name: &'a str,
    logic: Logic<'a>,
    outputs: Vec<&'a str>,
}

impl<'a> Module<'a> {
    fn update(&mut self, input: &'a str, pulse: Pulse) -> Option<Pulse> {
        match &mut self.logic {
            Logic::Broadcast => Some(pulse),
            Logic::FlipFlop { on } => match pulse {
                Pulse::Low => {
                    if *on {
                        *on = false;
                        Some(Pulse::Low)
                    } else {
                        *on = true;
                        Some(Pulse::High)
                    }
                }
                Pulse::High => None,
            },
            Logic::Conjunction { input_states } => {
                input_states.insert(input, pulse);

                if input_states.values().all(|pulse| *pulse == Pulse::High) {
                    Some(Pulse::Low)
                } else {
                    Some(Pulse::High)
                }
            }
        }
    }
}

fn parse_modules(input: &str) -> HashMap<&str, Module<'_>> {
    let mut modules = HashMap::new();

    // Parse modules from lines
    for line in input.trim().lines() {
        let (name, outputs) = line.split_once(" -> ").unwrap();

        let outputs: Vec<_> = outputs.split(", ").collect();

        let (name, logic) = match name.chars().next().unwrap() {
            'b' => (name, Logic::Broadcast),
            '%' => (&name[1..], Logic::FlipFlop { on: false }),
            '&' => (
                &name[1..],
                Logic::Conjunction {
                    input_states: HashMap::new(),
                },
            ),
            _ => unreachable!(),
        };

        modules.insert(
            name,
            Module {
                name,
                logic,
                outputs,
            },
        );
    }

    // We need to populate the inputs for conjunctions
    // Collect all inputs for each module
    let mut inputs: HashMap<&str, Vec<&str>> = HashMap::new();
    for module in modules.values() {
        for output in module.outputs.iter() {
            inputs.entry(output).or_default().push(module.name);
        }
    }

    // Populate conjunctions
    for (module, inputs) in inputs.into_iter() {
        if let Some(Logic::Conjunction {
            ref mut input_states,
        }) = modules.get_mut(module).map(|m| &mut m.logic)
        {
            *input_states = inputs
                .into_iter()
                .map(|input| (input, Pulse::Low))
                .collect();
        }
    }

    modules
}

pub fn part1(input: &str) -> String {
    let mut modules = parse_modules(input);

    let mut low_pulse_count = 0;
    let mut high_pulse_count = 0;

    // Queue of modules to visit and the state to apply
    let mut queue: VecDeque<(&str, &str, Pulse)> = VecDeque::new();

    // Push the button 1000 times
    for _ in 0..1000 {
        // Button connected to broadcast modules
        queue.clear();
        queue.push_back(("broadcaster", "", Pulse::Low));

        while let Some((name, input, pulse)) = queue.pop_front() {
            // Update pulse count
            match pulse {
                Pulse::Low => low_pulse_count += 1,
                Pulse::High => high_pulse_count += 1,
            }

            // Get module
            if let Some(module) = modules.get_mut(name) {
                // Update module state with pulse
                if let Some(pulse) = module.update(input, pulse) {
                    // Propagate pulse
                    for output in module.outputs.iter() {
                        queue.push_back((output, name, pulse));
                    }
                }
            }
        }
    }

    (low_pulse_count * high_pulse_count).to_string()
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(numbers: &[usize]) -> usize {
    let mut res = numbers[0];

    for x in numbers.iter().skip(1) {
        res = x * res / gcd(*x, res);
    }

    res
}

pub fn part2(input: &str) -> String {
    let mut modules = parse_modules(input);

    // Push the button 1000 times
    let mut button_count = 0;

    // Queue of modules to visit and the state to apply
    let mut queue: VecDeque<(&str, &str, Pulse)> = VecDeque::new();

    // Module that outputs rx is a conjunction
    // Find all inputs to that conjunction
    let final_module = modules
        .values()
        .find(|m| m.outputs.contains(&"rx"))
        .unwrap();
    let final_module_name = final_module.name;

    // We need each of these input states to be high
    let input_states = match &final_module.logic {
        Logic::Conjunction { input_states } => input_states,
        _ => unreachable!(),
    };

    let mut cycle_lengths: HashMap<&str, Option<usize>> =
        input_states.iter().map(|(name, _)| (*name, None)).collect();

    loop {
        // Button connected to broadcast modules
        queue.clear();
        queue.push_back(("broadcaster", "", Pulse::Low));
        button_count += 1;

        while let Some((name, input, pulse)) = queue.pop_front() {
            if name == final_module_name && pulse == Pulse::High {
                // Find first time input is high
                if let Some(cycle_length) = cycle_lengths.get_mut(input) {
                    if cycle_length.is_none() {
                        *cycle_length = Some(button_count);
                    }
                }

                // If all cycles have been discovered
                if cycle_lengths.values().all(|c| c.is_some()) {
                    let cycle_lengths: Vec<_> = cycle_lengths.values().copied().flatten().collect();
                    return lcm(&cycle_lengths).to_string();
                }
            }

            // Get module
            if let Some(module) = modules.get_mut(name) {
                // Update module state with pulse
                if let Some(pulse) = module.update(input, pulse) {
                    // Propagate pulse
                    for output in module.outputs.iter() {
                        queue.push_back((output, name, pulse));
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r#"
broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
"#;

    const TEST_INPUT_2: &str = r#"
broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
"#;

    #[test]
    fn test_part1() {
        assert_eq!(&part1(TEST_INPUT_1), "32000000");
        assert_eq!(&part1(TEST_INPUT_2), "11687500");
    }
}
