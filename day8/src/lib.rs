use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Node<'a> {
    name: &'a str,
    left: &'a str,
    right: &'a str,
}

fn parse_nodes(nodes: &str) -> impl Iterator<Item = Node> {
    nodes.trim().lines().map(|line| {
        let (name, children) = line.split_once(" = ").unwrap();
        let (left, right) = children[1..children.len() - 1].split_once(", ").unwrap();

        Node { name, left, right }
    })
}

fn gcd(mut a: usize, mut b: usize) -> usize {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn lcm(numbers: &[usize]) -> usize {
    let mut res = numbers[0];

    for x in numbers.iter().skip(1) {
        res = x * res / gcd(*x, res);
    }

    res
}

pub fn part1(input: &str) -> u32 {
    let (commands, nodes) = input.split_once("\n\n").unwrap();

    let commands: Vec<_> = commands.trim().chars().collect();
    let nodes: HashMap<&str, Node> = parse_nodes(nodes).map(|node| (node.name, node)).collect();

    let mut current_node = nodes.get("AAA").expect("Couldn't find AAA");
    for (step, command) in commands.iter().cycle().enumerate() {
        if current_node.name == "ZZZ" {
            return step as u32;
        }

        let next_node = match command {
            'L' => current_node.left,
            'R' => current_node.right,
            _ => panic!("Bad command"),
        };
        current_node = nodes.get(next_node).expect("Node not found");
    }

    panic!("Empty map")
}

pub fn part2(input: &str) -> u32 {
    let (commands, nodes) = input.split_once("\n\n").unwrap();

    let commands: Vec<_> = commands.trim().chars().collect();
    let nodes: HashMap<&str, Node> = parse_nodes(nodes).map(|node| (node.name, node)).collect();

    // All nodes ending with A
    let steps: Vec<_> = nodes
        .values()
        .filter(|node| node.name.ends_with('A'))
        .map(|mut current_node| {
            for (step, command) in commands.iter().cycle().enumerate() {
                if current_node.name.ends_with('Z') {
                    return step;
                }

                let next_node = match command {
                    'L' => current_node.left,
                    'R' => current_node.right,
                    _ => panic!("Bad command"),
                };
                current_node = nodes.get(next_node).expect("Node not found");
            }
            panic!("Empty map");
        })
        .collect();

    let res = lcm(&steps);

    // This is the only day thats solution doesn't fit in 32 bits.
    // I need to rework the runner to account for this.
    // For now just print the answer
    println!("Solution: {}", res);
    res as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT_1: &str = r#"
RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
"#;

    const TEST_INPUT_2: &str = r#"
LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
"#;

    const TEST_INPUT_3: &str = r#"
LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT_1), 2);
        assert_eq!(part1(TEST_INPUT_2), 6);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT_3), 6);
    }
}
