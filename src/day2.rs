type InputType = Vec<Game>;
type OutputType = u64;

use regex::Regex;

struct Round {
    red: u8,
    blue: u8,
    green: u8,
}

pub struct Game {
    id: usize,
    round: Vec<Round>,
}

#[aoc_generator(day2)]
fn day2_parse(input: &str) -> InputType {
    let re = Regex::new(r"Game (\d+): (.*)").unwrap();
    let gamereg = Regex::new(r"(\d+) (\w+)").unwrap();
    input
        .lines()
        .map(|line| {
            let caps = re.captures(line).unwrap();
            let id = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
            //Example: Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            let round = caps
                .get(2)
                .unwrap()
                .as_str()
                .split(";")
                .map(|round| {
                    let mut red = 0;
                    let mut blue = 0;
                    let mut green = 0;
                    for cap in gamereg.captures_iter(round) {
                        let count = cap.get(1).unwrap().as_str().parse::<u8>().unwrap();
                        let color = cap.get(2).unwrap().as_str();
                        match color {
                            "red" => red = count,
                            "blue" => blue = count,
                            "green" => green = count,
                            _ => panic!("Unknown color {}", color),
                        }
                    }
                    Round { red, blue, green }
                })
                .collect();
            Game { id, round }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let max_red = 12;
    let max_blue = 14;
    let max_green = 13;

    input
        .iter()
        .filter_map(|game| {
            let good = game.round.iter().all(|round| {
                round.red <= max_red && round.blue <= max_blue && round.green <= max_green
            });
            if good {
                Some(game.id as u64)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn part2(input: &InputType) -> OutputType {
    input
        .iter()
        .map(|game| {
            let mut min_red = 0;
            let mut min_blue = 0;
            let mut min_green = 0;
            game.round.iter().for_each(|round| {
                min_red = min_red.max(round.red);
                min_blue = min_blue.max(round.blue);
                min_green = min_green.max(round.green);
            });
            (min_red as u64) * (min_blue as u64) * (min_green as u64)
        })
        .sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"
    }

    #[test]
    fn day2_part1() {
        assert_eq!(part1(&day2_parse(get_test_input())), 8);
    }

    #[test]
    fn day2_part2() {
        assert_eq!(part2(&day2_parse(get_test_input())), 2286);
    }
}
