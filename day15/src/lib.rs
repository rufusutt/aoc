fn hash(s: &str) -> u8 {
    s.chars().fold(0, |value, ch| {
        assert!(ch.is_ascii());
        let ascii = ch as u8;
        (((value as u32 + ascii as u32) * 17) % 256) as u8
    })
}

pub fn part1(input: &str) -> String {
    input
        .trim()
        .split(',')
        .map(|step| hash(step) as u32)
        .sum::<u32>()
        .to_string()
}

struct Lens<'a> {
    label: &'a str,
    focal_length: &'a str,
}

pub fn part2(input: &str) -> String {
    // Create our boxes
    let mut boxes: [Vec<Lens>; 256] = array_init::array_init(|_| Vec::new());

    for step in input.trim().split(',') {
        let operation_index = step.find(['=', '-']).expect("Invalid step");
        let operation = step.chars().nth(operation_index).unwrap();

        let (label, focal_length) = step.split_once(['=', '-']).unwrap();

        // Get box
        let i = hash(label) as usize;
        let lenses = &mut boxes[i];

        match operation {
            '=' => {
                if let Some(i) = lenses.iter().position(|lens| lens.label == label) {
                    lenses[i] = Lens {
                        label,
                        focal_length,
                    }
                } else {
                    lenses.push(Lens {
                        label,
                        focal_length,
                    })
                }
            }
            '-' => {
                if let Some(i) = lenses.iter().position(|lens| lens.label == label) {
                    lenses.remove(i);
                }
            }
            _ => panic!("Invalid operation"),
        }
    }

    let mut focusing_power = 0;
    for (box_number, lenses) in boxes.iter().enumerate() {
        for (slot_number, lens) in lenses.iter().enumerate() {
            let focal_length: u32 = lens.focal_length.parse().unwrap();
            focusing_power += (box_number as u32 + 1) * (slot_number as u32 + 1) * focal_length;
        }
    }

    focusing_power.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn test_part1() {
        assert_eq!(&part1(TEST_INPUT), "1320");
    }

    #[test]
    fn test_part2() {
        assert_eq!(&part2(TEST_INPUT), "145");
    }
}
