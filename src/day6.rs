#[allow(unused_imports)]
use colored::Colorize as _;

type InputType = Vec<Race>;
type OutputType = u64;

#[derive(Debug)]
pub struct Race {
    time: u64,
    distance: u64,
}

#[aoc_generator(day6)]
fn day6_parse(input: &str) -> InputType {
    let mut lines = input.lines();

    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    let distances = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    std::iter::zip(times, distances)
        .map(|(t, d)| Race {
            time: t,
            distance: d,
        })
        .collect()
}

fn the_fancy_math_way(race: &Race) -> u64 {
    // (race_time - hold_time) * hold_time > record
    // hold_time * race_time - hold_time^2 - record > 0
    // -1 * hold_time^2 + race_time * hold_time + -record > 0
    // ax^2 + bc + c = 0
    // x = hold_time
    // a = -1
    // b = race_time
    // c = -record

    let t = race.time as f64;
    // Need to beat the record by 1 millisecond
    let d = race.distance as f64 + 1.0;
    let root = (t.powi(2) - 4.0 * d).sqrt();
    let a = (((-t - root) / -2.0).floor()) as usize;
    let b = (((-t + root) / -2.0).ceil()) as usize;
    #[cfg(test)]
    {
        println!(
            " a: {}, b: {}, root: {}, d: {}, t: {}",
            a.to_string().red(),
            b.to_string().blue(),
            root.to_string().green(),
            d.to_string().yellow(),
            t.to_string().purple()
        );
    }
    (a - b + 1) as u64
}

#[aoc(day6, part1, fancy_math)]
pub fn part1_math(input: &InputType) -> OutputType {
    input.iter().map(|race| the_fancy_math_way(race)).product()
}

#[aoc(day6, part2, fancy_math)]
pub fn part2_math(input: &InputType) -> OutputType {
    let bad_kerning = unkernify(input);
    part1_math(&bad_kerning)
}

#[aoc(day6, part1)]
pub fn part1(input: &InputType) -> OutputType {
    #[cfg(test)]
    println!("input: {:?}", input);

    //I'm sure there is some sort of fancy math to do this, but I'm just going to brute force it

    input
        .iter()
        .map(|race| {
            //Map to how many ways you can win
            let mut wins: u64 = 0;

            for button_hold_time in 1..race.time {
                //Since the button hold time determines the velocity, we can just multiply the velocity
                //by the time to get the distance
                let time_left = race.time - button_hold_time;
                let distance = time_left * button_hold_time;
                let won = distance > race.distance;

                if won {
                    wins += 1;
                }
            }
            #[cfg(test)]
            println!("Race {:?} wins: {}", race, wins);
            wins
        })
        .product()
}

fn unkernify(input: &InputType) -> InputType {
    let bad_kerning = input.iter().fold(
        Race {
            time: 0,
            distance: 0,
        },
        //Needlessly complicated way to stringify the numbers together
        |mut acc, race| {
            acc.time = (acc.time.to_string() + &race.time.to_string())
                .parse::<u64>()
                .unwrap();
            acc.distance = (acc.distance.to_string() + &race.distance.to_string())
                .parse::<u64>()
                .unwrap();
            acc
        },
    );
    vec![bad_kerning]
}

#[aoc(day6, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let bad_kerning = unkernify(input);
    part1(&bad_kerning)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "Time:      7  15   30
Distance:  9  40  200"
    }

    #[test]
    fn day6_part1() {
        assert_eq!(part1(&day6_parse(get_test_input())), 288);
    }

    #[test]
    fn day6_part2() {
        assert_eq!(part2(&day6_parse(get_test_input())), 71503);
    }

    #[test]
    fn day6_part1_math() {
        assert_eq!(part1_math(&day6_parse(get_test_input())), 288);
    }

    #[test]
    fn day6_part2_math() {
        assert_eq!(part2_math(&day6_parse(get_test_input())), 71503);
    }
}
