type InputType = Vec<Vec<i64>>;
type OutputType = u64;

#[aoc_generator(day9)]
fn day9_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i64>().unwrap())
                .collect()
        })
        .collect()
}

#[aoc(day9, part1)]
pub fn part1(input: &InputType) -> OutputType {
    solver(input, true)
}

pub fn solver(input: &InputType, last: bool) -> OutputType {
    input
        .iter()
        .map(|history| {
            //I'm sure I could optimize a lot of this, and remove the clones... but.... I don't
            //care.
            let mut cur_level = history.clone();
            let mut levels = Vec::new();
            levels.push(cur_level.clone());

            while !cur_level.iter().all(|x| x == &0) {
                let next_level = &cur_level[..]
                    .windows(2)
                    .map(|x| x[1] - x[0])
                    .collect::<Vec<i64>>();
                levels.push(next_level.clone());
                cur_level = next_level.to_vec();
            }

            #[cfg(test)]
            println!("{:?}", levels);

            //Now that we have the extrapolated levels, we need to find the next value in the sequence
            //(for the initial history) by extrapolating up
            //For example:

            //10  13  16  21  30  45  *68
            //  3   3   5   9  15  *23
            //    0   2   4   6   *8
            //      2   2   2   *2
            //        0   0   0

            let mut last_layer_diff = 0;

            let mut reverse_up = levels.iter_mut().rev();
            reverse_up.next(); //skip 0s

            while let Some(rev_layer) = reverse_up.next() {
                let last_value = if last {
                    rev_layer.iter().last().unwrap() + last_layer_diff
                } else {
                    //Since we only care about a single value, just tack the "next" value to the
                    //end of the array, rather than it's true position
                    rev_layer.get(0).unwrap() - last_layer_diff
                };
                rev_layer.push(last_value);
                last_layer_diff = last_value;

                #[cfg(test)]
                println!("{:?}", rev_layer);
            }

            #[cfg(test)]
            println!("{:?}", levels);

            levels.get(0).unwrap().iter().last().unwrap().clone()
        })
        .sum::<i64>() as u64
}

#[aoc(day9, part2)]
pub fn part2(input: &InputType) -> OutputType {
    solver(input, false)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    }

    #[test]
    fn day9_part1() {
        assert_eq!(part1(&day9_parse(get_test_input())), 114);
    }

    #[test]
    fn day9_part2() {
        assert_eq!(part2(&day9_parse(get_test_input())), 2);
    }
}
