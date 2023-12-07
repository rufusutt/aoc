use std::cmp::Ordering;

const CARDS: [char; 14] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug, Eq)]
struct Hand {
    cards: [char; 5],
    joker: bool,
    bid: u32,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl Hand {
    fn best_hand_type(&self) -> HandType {
        let mut counts = [0; 14];

        for &card in self.cards.iter() {
            let index = CARDS.iter().position(|&c| c == card).unwrap();
            counts[index] += 1;
        }

        // If use jokers as wildcards
        if self.joker {
            // Get max position
            let max_index = counts
                .iter()
                .enumerate()
                .skip(1)
                .max_by_key(|(_, &count)| count)
                .unwrap()
                .0;

            counts[max_index] += counts[0];
            counts[0] = 0;
        }

        let max_count = *counts.iter().max().unwrap();

        match max_count {
            1 => HandType::HighCard,
            2 => {
                if counts.iter().filter(|&&count| count == 2).count() == 2 {
                    HandType::TwoPair
                } else {
                    HandType::OnePair
                }
            }
            3 => {
                if counts.iter().any(|&count| count == 2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            4 => HandType::FourOfAKind,
            5 => HandType::FiveOfAKind,
            _ => unreachable!(),
        }
    }

    fn card_rank(card: char) -> usize {
        CARDS.iter().position(|c| card == *c).expect("Invalid card")
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let hand_ordering = self.best_hand_type().cmp(&other.best_hand_type());
        if hand_ordering == Ordering::Equal {
            for (&a, &b) in self.cards.iter().zip(other.cards.iter()) {
                let card_ordering = Self::card_rank(a).cmp(&Self::card_rank(b));
                if card_ordering != Ordering::Equal {
                    return card_ordering;
                }
            }
        }
        hand_ordering
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn parse_hands(input: &str, joker: bool) -> Vec<Hand> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();

            // Collect into temp vec before convert to array
            let cards: [char; 5] = cards
                .chars()
                .collect::<Vec<char>>()
                .try_into()
                .expect("Expect 5 cards");

            let bid = bid.parse::<u32>().expect("Invalid bid");

            Hand { cards, joker, bid }
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let mut hands = parse_hands(input, false);

    // Sort hands
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    let mut hands = parse_hands(input, true);

    // Sort hands
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    #[test]
    fn test_hand_type() {
        let test_hands: Vec<_> = TEST_INPUT
            .trim()
            .lines()
            .map(|line| {
                let cards = line.split_once(' ').unwrap().0;

                let cards: [char; 5] = cards
                    .chars()
                    .collect::<Vec<char>>()
                    .try_into()
                    .expect("Expect 5 cards");

                Hand {
                    cards,
                    joker: false,
                    bid: 0,
                }
            })
            .collect();

        assert_eq!(
            test_hands
                .iter()
                .map(Hand::best_hand_type)
                .collect::<Vec<_>>(),
            [
                HandType::OnePair,
                HandType::ThreeOfAKind,
                HandType::TwoPair,
                HandType::TwoPair,
                HandType::ThreeOfAKind,
            ]
        );

        assert_eq!(
            test_hands
                .iter()
                .map(|hand| Hand {
                    cards: hand.cards,
                    joker: true,
                    bid: 0
                })
                .map(|hand| hand.best_hand_type())
                .collect::<Vec<_>>(),
            [
                HandType::OnePair,
                HandType::FourOfAKind,
                HandType::TwoPair,
                HandType::FourOfAKind,
                HandType::FourOfAKind,
            ]
        );
    }

    #[test]
    fn test_cmp_hands() {
        let a = Hand {
            cards: ['3', '3', '3', '3', '2'],
            joker: false,
            bid: 0,
        };
        let b = Hand {
            cards: ['2', 'A', 'A', 'A', 'A'],
            joker: false,
            bid: 0,
        };
        assert_eq!(a.cmp(&b), Ordering::Greater);

        let a = Hand {
            cards: ['7', '7', '8', '8', '8'],
            joker: false,
            bid: 0,
        };
        let b = Hand {
            cards: ['7', '7', '7', '8', '8'],
            joker: false,
            bid: 0,
        };
        assert_eq!(a.cmp(&b), Ordering::Greater);

        let a = Hand {
            cards: ['K', 'K', '6', '7', '7'],
            joker: false,
            bid: 0,
        };
        let b = Hand {
            cards: ['K', 'T', 'J', 'J', 'T'],
            joker: false,
            bid: 0,
        };
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 6440);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 5905);
    }
}
