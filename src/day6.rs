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

#[aoc(day6, part2)]
pub fn part2(input: &InputType) -> OutputType {
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

    part1(&vec![bad_kerning])
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
}
