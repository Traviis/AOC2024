type InputType = Vec<u64>;
type OutputType = u64;

#[aoc_generator(day1)]
fn day1_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| line.parse::<u64>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input.iter().map(|x| x / 3 - 2).sum()
}

#[aoc(day1, part2)]
pub fn part2(input: &InputType) -> OutputType {

    let fuel_reqs = input.iter().map(|x| *x as i64 / 3 - 2).collect::<Vec<i64>>();
    let component_fuel : i64 = fuel_reqs.iter().sum();


    let other_fuel : i64 = fuel_reqs.iter().map(|&x| {
        let mut fuel = x;
        let mut total = 0;
        while fuel > 0 {
            total += fuel;
             println!("fuel: {}", fuel);
            fuel = fuel / 3 - 2;
        }
        println!("Total: {}", total);
        total
    }).sum();
    println!("Component fuel: {}", component_fuel);

    //(component_fuel + other_fuel) as u64
    other_fuel as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "100756"
    }

    #[test]
    fn day1_part1() {
        assert_eq!(part1(&day1_parse(get_test_input())), 33583);
    }

    #[test]
    fn day1_part2() {
        assert_eq!(part2(&day1_parse(get_test_input())), 50346);
    }
}
