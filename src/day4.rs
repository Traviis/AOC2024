use regex::Regex;
use std::collections::{vec_deque::VecDeque, HashMap};

type InputType = Vec<Card>;
type OutputType = u64;

#[derive(Clone)]
pub struct Card {
    pub id: u64,
    //I would think you could just put these into a set, but I have a sneaking suspicion that the
    //order is going to matter for part 2
    pub numbers: Vec<u64>,
    pub winning_numbers: Vec<u64>,
}

#[aoc_generator(day4)]
fn day4_parse(input: &str) -> InputType {
    // Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    let re = Regex::new(r"Card +(\d+): +((?:\d+ *)+) \| +((?:\d+ +)+\d+)").unwrap();
    input
        .lines()
        .map(|line| {
            let captures = re.captures(line);
            if captures.is_none() {
                panic!("Failed to parse line: {}", line);
            }
            let captures = captures.unwrap();
            let id = captures
                .get(1)
                .unwrap()
                .as_str()
                .trim()
                .parse::<u64>()
                .unwrap();
            let winning_numbers = captures
                .get(2)
                .unwrap()
                .as_str()
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            let numbers = captures
                .get(3)
                .unwrap()
                .as_str()
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect::<Vec<u64>>();
            //#[cfg(test)]
            //{
            //    println!("id: {}", id);
            //    println!("winning_numbers: {:?}", winning_numbers);
            //    println!("numbers: {:?}", numbers);
            //}
            Card {
                id,
                numbers,
                winning_numbers,
            }
        })
        .collect::<Vec<Card>>()
}

fn get_card_score(card: &Card) -> u64 {
    let match_count = card
        .numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count();
    if match_count == 0 {
        return 0;
    }
    2_u64.pow((match_count - 1) as u32)
}

#[aoc(day4, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input
        .iter()
        .map(|card| {
            get_card_score(card)
            //Determine how many matches we have; could do this with a set intersection, but just do a n^2 loop
        })
        .sum()
}

#[aoc(day4, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let match_count = input
        .iter()
        .map(|card| {
            (
                card.id,
                card.numbers
                    .iter()
                    .filter(|n| card.winning_numbers.contains(n))
                    .count(),
            )
        })
        .collect::<std::collections::HashMap<u64, usize>>();

    let mut card_count = input
        .iter()
        .map(|card| (card.id, 1))
        .collect::<std::collections::HashMap<u64, u64>>();
    // Go through each card and determine how many matches it has, then add that count to the copy
    // map; Start with 1 copy of each

    let mut card_stack = input.into_iter().cloned().collect::<VecDeque<Card>>();

    //TODO: Too tired to do this.
    while let Some(card) = card_stack.pop_front() {
        //for card in card_stack {
        let mut num_matches = *match_count.get(&card.id).unwrap();
        if num_matches == 0 {
            continue;
        }
        let cur_card_count = *card_count.get(&card.id).unwrap();
        #[cfg(test)]
        println!("Card {} has {} matches", card.id, num_matches - 1);
        for id in card.id + 1..=(card.id + num_matches as u64) {
            #[cfg(test)]
            {
                println!("Card {} adding copy of {} to card_count", card.id, id);
            }
            //TODO: I'm too tired to figure this out, but I think you can do the same thing I'm
            //doing here, by multiping the current card count and adding that many to the "stack"
            //of cards. As opposed to literally adding a copy of the card to the stack. This should
            //be viable since we can't "come back" to a card once we have processed it.

            *card_count.entry(id).or_insert(0) += 1;
            //Just copy the card, there is probably a fancy math way to multiply this properly, but
            //I can't see it at 2:30 AM
            let card = input[(id - 1) as usize].clone();
            card_stack.push_back(card);
            #[cfg(test)]
            {
                println!(
                    "card_stack: {:?}",
                    card_stack.iter().map(|c| c.id).collect::<Vec<u64>>()
                );
            }
        }
    }

    #[cfg(test)]
    {
        println!("match_count: {:?}", match_count);
        println!("card_count: {:?}", card_count);
    }

    // Now go through the map and multiply the scores together
    card_count.iter().map(|(_, count)| count).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
"
    }

    #[test]
    fn day4_part1() {
        assert_eq!(part1(&day4_parse(get_test_input())), 13);
    }

    #[test]
    fn day4_part2() {
        assert_eq!(part2(&day4_parse(get_test_input())), 30);
    }
}
