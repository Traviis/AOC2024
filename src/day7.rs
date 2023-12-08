use std::{
    collections::HashMap,
    fmt::{Display, Formatter},
};

#[allow(unused_imports)]
use colored::Colorize as _;

#[derive(Debug, Clone, Copy, Hash, Eq, Ord)]
enum Card {
    Joker,
    N(u8),
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn card_to_num_val(&self) -> u8 {
        match self {
            Card::A => 14,
            Card::K => 13,
            Card::Q => 12,
            Card::J => 11,
            Card::T => 10,
            Card::N(n) => *n,
            Card::Joker => 1,
        }
    }
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        let my_val = self.card_to_num_val();
        let other_val = other.card_to_num_val();
        my_val == other_val
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let my_val = self.card_to_num_val();
        let other_val = other.card_to_num_val();
        my_val.partial_cmp(&other_val)
    }
}

type Hand = [Card; 5];

#[derive(Debug, Clone, Ord, Eq)]
pub struct CamelHand {
    hand: Hand,
    bet: u64,
}

impl Display for CamelHand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let colored_hand = self
            .hand
            .iter()
            .map(|c| match c {
                Card::A => "A".red(),
                Card::K => "K".blue(),
                Card::Q => "Q".green(),
                Card::J => "J".purple(),
                Card::T => "T".yellow(),
                Card::N(n) => format!("{}", n).white(),
                Card::Joker => "J".cyan(),
            })
            .collect::<Vec<_>>();
        let colored_string = colored_hand.iter().fold(String::new(), |mut acc, c| {
            acc.push_str(&c.to_string());
            acc
        });
        write!(f, "{} ({:?})", colored_string, self.rank())
    }
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd, Clone, Copy)]
enum HandRank {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl CamelHand {
    fn rank(&self) -> HandRank {
        let mut card_count: HashMap<Card, u8> = HashMap::new();
        for &c in self.hand.iter() {
            let count = card_count.entry(c).or_insert(0);
            *count += 1;
        }

        let joker_count = *card_count.entry(Card::Joker).or_insert(0);
        let diff_cards = card_count.len();

        // Five of a kind
        let max_same = card_count
            .iter()
            .filter(|(k, _)| **k != Card::Joker)
            .map(|(_, v)| v)
            .max()
            .unwrap_or(&0);
        if card_count.values().any(|&v| v == 5) || max_same + joker_count == 5 {
            return HandRank::FiveOfAKind;
        }
        //Get the max same non-joker

        //Need to account for this case: J2J83; This is a three of a kind at best (with the two jokers).
        // Four of a kind
        if joker_count == 4 || (max_same + joker_count) == 4 {
            return HandRank::FourOfAKind;
        }
        // Full hosue
        if card_count.values().any(|&v| v == 3) && card_count.values().any(|&v| v == 2) {
            return HandRank::FullHouse;
        }

        //Joker full houses are an odd case
        let hand_type = max_same + joker_count;
        if joker_count > 0 && hand_type == 3 {
            if diff_cards - 1 == 2 {
                return HandRank::FullHouse;
            } else {
                return HandRank::ThreeOfAKind;
            }
        }

        // three of a kind
        //if card_count.values().any(|&v| v == 3) || joker_count > 0 && card_count.values().any(|&v| v == 2){
        if (max_same + joker_count) == 3 {
            return HandRank::ThreeOfAKind;
        }
        // two pair
        if card_count.values().filter(|&v| *v == 2).count() == 2
            || joker_count > 0 && card_count.values().any(|&v| v == 2)
        {
            return HandRank::TwoPair;
        }
        // one pair
        if card_count.values().any(|&v| v == 2) || joker_count > 0 {
            return HandRank::OnePair;
        }
        // high card
        HandRank::HighCard
    }
}

impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        //push if hands are equal (ignoring order)
        let self_hand = self.hand.to_vec();
        let other_hand = other.hand.to_vec();
        self_hand == other_hand
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        //First determine if one hand is simply better than the other
        let my_rank = self.rank();
        let other_rank = other.rank();
        let hand_rank = my_rank.cmp(&other_rank);
        if hand_rank != std::cmp::Ordering::Equal {
            return Some(hand_rank);
        }
        //If the hands are the same rank, then compare the cards within IN ORDER (
        for (my_card, other_card) in self.hand.iter().zip(other.hand.iter()) {
            let card_rank = my_card.partial_cmp(other_card);
            if card_rank.unwrap() != std::cmp::Ordering::Equal {
                return Some(card_rank.unwrap());
            }
        }
        panic!(); //No concept of tie
    }
}

type InputType = Vec<CamelHand>;
type OutputType = u64;

fn day7_parser(input: &str, joker_wild: bool) -> InputType {
    input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let mut hand = [Card::N(0); 5];
            let mut i = 0;
            let hand_str = parts.next().unwrap();
            for c in hand_str.chars() {
                let card = match c {
                    'A' => Card::A,
                    'K' => Card::K,
                    'Q' => Card::Q,
                    'J' => {
                        if joker_wild {
                            Card::Joker
                        } else {
                            Card::J
                        }
                    }
                    'T' => Card::T,
                    n => Card::N(n.to_digit(10).unwrap() as u8),
                };
                hand[i] = card;
                i += 1;
            }
            let bet = parts.next().unwrap().parse::<u64>().unwrap();
            CamelHand { hand, bet }
        })
        .collect()
}

#[aoc_generator(day7, part2)]
fn day7_parse_part2(input: &str) -> InputType {
    day7_parser(input, true)
}

#[aoc_generator(day7, part1)]
fn day7_parse_part1(input: &str) -> InputType {
    day7_parser(input, false)
}

#[aoc(day7, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let mut sorted_input = input.clone();
    sorted_input.sort_by(|a, b| a.partial_cmp(b).unwrap());
    #[cfg(test)]
    {
        for hand in sorted_input
            .iter()
            .filter(|h| h.hand.iter().filter(|c| **c == Card::Joker).count() > 0)
        {
            println!("{}", hand);
        }
    }
    sorted_input
        .iter()
        .enumerate()
        .rev()
        .map(|(idx, ch)| ch.bet * ((idx + 1) as u64))
        .sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &InputType) -> OutputType {
    part1(&input)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
    }

    #[test]
    fn card_compare() {
        assert!(Card::N(2) < Card::N(4));
        assert!(Card::A > Card::N(2));
        assert!(HandRank::FiveOfAKind > HandRank::FourOfAKind);
    }

    #[test]
    fn inner_rank_ordering() {
        //CamelHand { hand: [A, N(2), N(2), N(2), N(2)], bet: 449 } (FourOfAKind)
        let hand_1 = CamelHand {
            hand: [Card::A, Card::N(2), Card::N(2), Card::N(2), Card::N(2)],
            bet: 449,
        };
        //CamelHand { hand: [A, N(4), N(4), N(4), N(4)], bet: 801 } (FourOfAKind)
        let hand_2 = CamelHand {
            hand: [Card::A, Card::N(4), Card::N(4), Card::N(4), Card::N(4)],
            bet: 801,
        };

        assert_eq!(hand_1.rank(), hand_2.rank());
        let mut hands = vec![hand_2, hand_1];
        hands.sort_by(|a, b| a.partial_cmp(b).unwrap());
        println!("{:?}", hands);
        assert_eq!(hands[0].bet, 449);
        //
        //This is showing up in my ordered list as:
        //CamelHand { hand: [A, N(2), N(2), N(2), N(2)], bet: 449 } (FourOfAKind)
        //CamelHand { hand: [A, N(4), N(4), N(4), N(4)], bet: 801 } (FourOfAKind)
    }

    #[test]
    fn joker_test() {
        let hand = CamelHand {
            hand: [Card::Joker, Card::N(2), Card::Joker, Card::N(8), Card::N(3)],
            bet: 177,
        };
        assert_eq!(hand.rank(), HandRank::ThreeOfAKind);
    }

    #[test]
    fn day7_part1() {
        assert_eq!(part1(&day7_parse_part1(get_test_input())), 6440);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(part2(&day7_parse_part2(get_test_input())), 5905);
    }
}
