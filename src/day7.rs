type InputType = Vec<Equation>;
type OutputType = u64;

#[derive(Debug,Clone)]
enum Oper {
    Add,
    Multiply,
    Concat, // For concat, we just add the number to the previous number (AS A STRING)
}

type Equation = (u64, Vec<u64>);



fn valid_line(equa: &Equation, part2: bool) -> bool {

    //This might be pretty bruteforcy, but I don't know what other kind of algorithm to use here?
    let vals = equa.1.clone();

    //The most brute forcy way to do this is to determine the slots (len - 1) and then generate every possible combination of symbols (Add and Multiply in the part 1) to determine if the result is possible.
    let slots = vals.len() - 1;

    // Generate every possible combination of symbols
    //Use itertools
    let symbols= generate_combinations(slots, part2);
    //println!("{:?}", symbols);

    for symbol in symbols {
        let mut current = vals[0];
        for (i, val) in vals.iter().skip(1).enumerate() {
            match symbol[i] {
                Oper::Add => current += val,
                Oper::Multiply => current *= val,
                Oper::Concat => current = format!("{}{}", current, val).parse().unwrap(),
            }
        }
        if current == equa.0 {
            return true;
        }


    }

    false



}
fn generate_combinations(n: usize, part2: bool) -> Vec<Vec<Oper>> {
    use Oper::*;

    let mut operators = vec![Add, Multiply];
    if part2 {
        operators.push(Concat);
    }
    let mut results = Vec::new();
    let mut current = Vec::new();

    fn backtrack(
        n: usize,
        operators: &[Oper],
        current: &mut Vec<Oper>,
        results: &mut Vec<Vec<Oper>>,
    ) {
        if n == 0 {
            results.push(current.clone());
            return;
        }
        for op in operators {
            current.push(op.clone());
            backtrack(n - 1, operators, current, results);
            current.pop(); // backtrack
        }
    }

    backtrack(n, &operators, &mut current, &mut results);
    results
}




#[aoc_generator(day7)]
fn day7_parse(input: &str) -> InputType {
    input.lines().map(|line| {
        let mut parts = line.split(": ");
        let num = parts.next().unwrap().parse().unwrap();
        let vals = parts.next().unwrap().split(" ").map(|x| x.parse().unwrap()).collect();
        (num, vals)
    }).collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input.iter().filter(|x| valid_line(x,false)).map(|equa| equa.0).sum()
}

#[aoc(day7, part2)]
pub fn part2(input: &InputType) -> OutputType {
    input.iter().filter(|x| valid_line(x,true)).map(|equa| equa.0).sum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    }

    #[test]
    fn day7_part1() {
        assert_eq!(part1(&day7_parse(get_test_input())), 3749);
    }

    #[test]
    fn day7_part2() {
        assert_eq!(part2(&day7_parse(get_test_input())), 11387);
    }
}
