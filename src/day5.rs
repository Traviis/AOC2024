use std::collections::BTreeMap;

type InputType = (BTreeMap<u64, Vec<u64>>, Vec<Updates>);
type OutputType = u64;

// .0 must be < .1
type Updates = Vec<u64>;

#[aoc_generator(day5)]
fn day5_parse(input: &str) -> InputType {
    let mut parts = input.split("\n\n");
    let rules = parts.next().unwrap().lines().fold(
        BTreeMap::new(),
        |mut acc: BTreeMap<u64, Vec<u64>>, l| {
            let mut line_splits = l.split("|");
            let num_1 = line_splits.next().unwrap().parse().unwrap();
            let num_2 = line_splits.next().unwrap().parse().unwrap();
            //Interesting thing from clippy here, it wanted me to use or_default instead of
            //or_insert, but then that confuses the type system and it makes me have to specify the
            //type for BTreeMap (in the closure) which is kind of amusing since it should be able
            //to determine the type of rules from the function signature...
            acc.entry(num_1).or_default().push(num_2);
            acc
        },
    );

    let updates = parts
        .next()
        .unwrap()
        .lines()
        .map(|l| l.split(",").map(|n| n.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

fn is_valid(rules: &BTreeMap<u64, Vec<u64>>, updates: &[u64]) -> bool {
    //Interesting note about the data, all the list lengths are odd (to be able to find the middle)
    assert_eq!(updates.len() % 2, 1);

    updates.iter().enumerate().all(|(idx, num)| {
        //First rule is always valid
        if idx == 0 {
            return true;
        }

        let rule = match rules.get(num) {
            Some(r) => r,
            None => return true,
        };

        // The rule states that the numbers in the list must be AFTER the number in the rule.
        // Thus, check if all previous numbers aren't violating the rule

        let previous = &updates[0..idx];
        for p in previous {
            if rule.contains(p) {
                return false;
            }
        }

        true
    })
}

#[aoc(day5, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let rules = &input.0;
    let updates = &input.1;

    //Let's verify the lines.
    updates
        .iter()
        //yes, I know filter_map exists
        .filter(|update| is_valid(rules, update))
        .map(|update| update.get(update.len() / 2).unwrap())
        .sum::<u64>()
}

#[aoc(day5, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let rules = &input.0;
    let updates = &input.1;

    updates
        .iter()
        .filter(|update| !is_valid(rules, update))
        //fix_update short circuits and just gives me the middle value
        .map(|update| fix_update(rules, update))
        .sum::<u64>()
}

fn fix_update(rules: &BTreeMap<u64, Vec<u64>>, update: &[u64]) -> u64 {
    let mut update = update.to_owned();

    //We have all the rules, fix them by moving the value in front of all the rules it's violating,
    //you can always move left, so find the one that moves it the furthest left that satisfies the
    //rule without just moving it to 0th

    // Do that by determining if a particular rule is violated, if it is, find the index of all of
    // the places it's violating, since we only care about the left most (as moving right will
    // always make it violate even more) then just find the lowest (left-most) that we can put the
    // value that satisfies all the rules.

    for (idx, num) in update.clone().iter().enumerate() {
        let rule = match rules.get(num) {
            Some(r) => r,
            None => &vec![], //If there are no rules, we can't be violating them
        };

        let previous = &update[0..idx];
        let mut violation_indexes = vec![];

        for (v_idx, p) in previous.iter().enumerate() {
            if rule.contains(p) {
                violation_indexes.push(v_idx);
            }
        }

        if !violation_indexes.is_empty() {
            //We know we are violating the rule
            let lowest = violation_indexes.iter().min().unwrap(); //Find the lowest violator
            update.remove(idx);
            update.insert(*lowest, *num);
        }
    }

    //Since we know we only care about the middle digit, just return it here instead of doing it elsewhere
    update[update.len() / 2]
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"
    }

    #[test]
    fn day5_part2_test_single() {
        let (rules, _) = day5_parse(get_test_input());

        let update = vec![61, 13, 29];
        assert!(!is_valid(&rules, &update));
        assert_eq!(fix_update(&rules, &update), 29);

        let update = vec![75, 97, 47, 61, 53];
        assert!(!is_valid(&rules, &update));
        assert_eq!(fix_update(&rules, &update), 47);
    }

    #[test]
    fn day5_part1() {
        assert_eq!(part1(&day5_parse(get_test_input())), 143);
    }

    #[test]
    fn day5_part2() {
        assert_eq!(part2(&day5_parse(get_test_input())), 123);
    }
}
