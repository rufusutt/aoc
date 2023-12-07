use std::cmp::Ordering;

const CARDS: [char; 14] = [
    'J', '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
];

#[derive(Debug, Eq)]
struct Hand {
    cards: [char; 5],
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
        let mut count = [0u32; 14];
        for card in self.cards.iter() {
            let index = Self::card_rank(*card);
            count[index] += 1;
        }

        if count.contains(&5) {
            return HandType::FiveOfAKind;
        }

        if count.contains(&4) {
            return HandType::FourOfAKind;
        }

        if count.contains(&3) {
            if count.contains(&2) {
                return HandType::FullHouse;
            } else {
                return HandType::ThreeOfAKind;
            }
        }

        let pair_count = count.iter().filter(|c| **c == 2).count();
        if pair_count == 2 {
            return HandType::TwoPair;
        } else if pair_count == 1 {
            return HandType::OnePair;
        }

        HandType::HighCard
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

fn parse_hands(input: &str) -> Vec<Hand> {
    input
        .trim()
        .lines()
        .map(|line| {
            let (cards, bid) = line.split_once(' ').unwrap();

            // Collect into temp vec before convert to array
            let cards: [char; 5] = cards
                .chars()
                .into_iter()
                .collect::<Vec<char>>()
                .try_into()
                .expect("Expect 5 cards");

            let bid = bid.parse::<u32>().expect("Invalid bid");

            Hand { cards, bid }
        })
        .collect()
}

pub fn part1(input: &str) -> u32 {
    let mut hands = parse_hands(input);

    // Sort hands
    hands.sort_unstable();

    hands
        .iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum::<u32>()
}

pub fn part2(input: &str) -> u32 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PART1: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
"#;

    const TEST_PART2: &str = r#"
32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483 
"#;

    #[test]
    fn test_hand_type() {
        let hand_types: Vec<_> = TEST_PART1
            .trim()
            .lines()
            .map(|line| {
                let cards = line.split_once(' ').unwrap().0;

                let cards: [char; 5] = cards
                    .chars()
                    .into_iter()
                    .collect::<Vec<char>>()
                    .try_into()
                    .expect("Expect 5 cards");

                Hand { cards, bid: 0 }.best_hand_type()
            })
            .collect();

        assert_eq!(
            hand_types,
            [
                HandType::OnePair,
                HandType::ThreeOfAKind,
                HandType::TwoPair,
                HandType::TwoPair,
                HandType::ThreeOfAKind
            ]
        );
    }

    #[test]
    fn test_cmp_hands() {
        let a = Hand {
            cards: ['3', '3', '3', '3', '2'],
            bid: 0,
        };
        let b = Hand {
            cards: ['2', 'A', 'A', 'A', 'A'],
            bid: 0,
        };
        assert_eq!(a.cmp(&b), Ordering::Greater);

        let a = Hand {
            cards: ['7', '7', '8', '8', '8'],
            bid: 0,
        };
        let b = Hand {
            cards: ['7', '7', '7', '8', '8'],
            bid: 0,
        };
        assert_eq!(a.cmp(&b), Ordering::Greater);

        let a = Hand {
            cards: ['K', 'K', '6', '7', '7'],
            bid: 0,
        };
        let b = Hand {
            cards: ['K', 'T', 'J', 'J', 'T'],
            bid: 0,
        };
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_PART1), 6440);
    }

    // #[test]
    // fn test_part2() {
    //     assert_eq!(part2(TEST_INPUT), 71503);
    // }
}
