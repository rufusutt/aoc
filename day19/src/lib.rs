use std::collections::HashMap;

#[derive(Debug)]
struct Part {
    categories: [usize; 4],
}

impl Part {
    fn parse(input: &str) -> Part {
        let categories = array_init::from_iter(
            input[1..input.len() - 1]
                .split(',')
                .map(|cat| cat.split_once('=').unwrap().1.parse::<usize>().unwrap()),
        )
        .unwrap();

        Part { categories }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Destination<'a> {
    Workflow(&'a str),
    Rejected,
    Accepted,
}

impl<'a> Destination<'a> {
    fn parse(input: &'a str) -> Destination<'a> {
        match input {
            "R" => Destination::Rejected,
            "A" => Destination::Accepted,
            _ => Destination::Workflow(input),
        }
    }
}

#[derive(Debug, Clone)]
struct Rule<'a> {
    category: usize,
    condition: char,
    rhs: usize,
    destination: Destination<'a>,
}

impl<'a> Rule<'a> {
    fn parse(input: &'a str) -> Rule<'a> {
        let category = match input.chars().nth(0).unwrap() {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        };
        let condition = input.chars().nth(1).unwrap();

        let (rhs, destination) = input[2..].split_once(':').unwrap();

        let rhs = rhs.parse::<usize>().unwrap();
        let destination = Destination::parse(destination);

        Rule {
            category,
            condition,
            rhs,
            destination,
        }
    }

    fn matches(&self, part: &Part) -> Option<Destination> {
        if match self.condition {
            '<' => part.categories[self.category] < self.rhs,
            '>' => part.categories[self.category] > self.rhs,
            _ => unreachable!(),
        } {
            Some(self.destination)
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    rules: Vec<Rule<'a>>,
    otherwise: Destination<'a>,
}

impl<'a> Workflow<'a> {
    fn parse(line: &'a str) -> (&'a str, Workflow<'a>) {
        let mut parts = line.split(['{', '}']);

        let name = parts.next().unwrap();

        let rules = parts.next().unwrap();
        let rules = || rules.split(',');

        let otherwise = rules().last().map(Destination::parse).unwrap();
        let rules: Vec<_> = rules().take(rules().count() - 1).map(Rule::parse).collect();

        (name, Workflow { rules, otherwise })
    }

    fn sort(&self, part: &Part) -> Destination {
        for rule in self.rules.iter() {
            if let Some(destination) = rule.matches(part) {
                return destination;
            }
        }
        self.otherwise
    }
}

pub fn part1(input: &str) -> String {
    // Parse input
    let (workflows, parts) = input.trim().split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows.lines().map(Workflow::parse).collect();
    let parts: Vec<_> = parts.lines().map(Part::parse).collect();

    let mut accepted_parts = Vec::new();

    for part in parts {
        let mut current_workflow = workflows.get("in").unwrap();

        loop {
            match current_workflow.sort(&part) {
                Destination::Workflow(name) => current_workflow = workflows.get(name).unwrap(),
                Destination::Rejected => break,
                Destination::Accepted => {
                    accepted_parts.push(part);
                    break;
                }
            }
        }
    }

    accepted_parts
        .into_iter()
        .map(|p| p.categories.iter().sum::<usize>())
        .sum::<usize>()
        .to_string()
}

fn find_accepted(
    workflows: &HashMap<&str, Workflow>,
    current_workflow: &Workflow,
    ranges: [[usize; 2]; 4],
    valid_ranges: &mut Vec<[[usize; 2]; 4]>,
) {
    // The ranges that have not yet matched any rules
    let mut current_ranges = ranges;

    for rule in current_workflow.rules.iter() {
        // Index of range to be updated by rule
        let i = rule.category;

        // Work out the set of ranges that match this condition
        let mut matched_ranges = current_ranges;
        match rule.condition {
            '<' => matched_ranges[i][1] = matched_ranges[i][1].min(rule.rhs - 1),
            '>' => matched_ranges[i][0] = matched_ranges[i][0].max(rule.rhs + 1),
            _ => unreachable!(),
        }

        // Calculate valid ranges for matches
        match rule.destination {
            Destination::Workflow(name) => find_accepted(
                workflows,
                workflows.get(name).unwrap(),
                matched_ranges,
                valid_ranges,
            ),
            Destination::Rejected => {}
            Destination::Accepted => valid_ranges.push(matched_ranges),
        }

        // Update current ranges to those that don't match
        match rule.condition {
            '<' => current_ranges[i][0] = current_ranges[i][0].max(rule.rhs),
            '>' => current_ranges[i][1] = current_ranges[i][1].min(rule.rhs),
            _ => unreachable!(),
        }
    }

    match current_workflow.otherwise {
        Destination::Workflow(name) => find_accepted(
            workflows,
            workflows.get(name).unwrap(),
            current_ranges,
            valid_ranges,
        ),
        Destination::Rejected => (),
        Destination::Accepted => valid_ranges.push(current_ranges),
    }
}

pub fn part2(input: &str) -> String {
    // Parse input
    let (workflows, _) = input.trim().split_once("\n\n").unwrap();
    let workflows: HashMap<_, _> = workflows.lines().map(Workflow::parse).collect();

    let ranges: [[usize; 2]; 4] = array_init::array_init(|_| [1, 4000]);

    let mut valid_ranges = Vec::new();
    let start_workflow = workflows.get("in").unwrap();
    find_accepted(&workflows, start_workflow, ranges, &mut valid_ranges);

    valid_ranges
        .iter()
        .map(|ranges| {
            ranges
                .iter()
                .filter(|range| range[0] <= range[1])
                .map(|range| range[1] - range[0] + 1)
                .product::<usize>()
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
"#;

    #[test]
    fn test_part1() {
        assert_eq!(&part1(TEST_INPUT), "19114");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(TEST_INPUT), "167409079868000");
    }
}
