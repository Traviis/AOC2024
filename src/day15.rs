use regex::Regex;

type InputType = Vec<String>;
type OutputType = u64;

#[aoc_generator(day15)]
fn day15_parse(input: &str) -> InputType {
    input.split(",").map(|s| s.to_string()).collect()
}

#[aoc(day15, part1)]
pub fn part1(input: &InputType) -> OutputType {
    input.iter().map(|s| hash(s)).sum()
}

type Lens = String;

struct LBox {
    lenses: Vec<(Lens, u8)>, // label -> focal
}

impl LBox {
    fn add_lens(&mut self, label: &str, focal: u8) {
        // if label already exists, replace focal length
        for (l, f) in self.lenses.iter_mut() {
            if l == label {
                *f = focal;
                return;
            }
        }
        // otherwise add new lens
        self.lenses.push((label.to_string(), focal));
    }

    fn remove_lens(&mut self, label: &str) {
        self.lenses.retain(|(l, _)| l != label);
    }

    fn get_focusing_power(&self, idx: usize) -> i64 {
        self.lenses
            .iter()
            .enumerate()
            .map(|(slot, (_, focal))| {
                let mut power = idx + 1;
                power *= slot + 1;
                power *= *focal as usize;
                power as i64
            })
            .sum()
    }
}

#[aoc(day15, part2)]
pub fn part2(input: &InputType) -> OutputType {
    // set up boxes

    let mut boxes = Vec::new();
    for _ in 0..256 {
        boxes.push(LBox { lenses: Vec::new() });
    }

    let re = Regex::new(r"(\w+)([=-])(\d*)").unwrap();

    input.iter().for_each(|cmd| {
        let caps = re.captures(cmd).unwrap();
        let label = caps.get(1).unwrap().as_str();
        let op = caps.get(2).unwrap().as_str();
        let box_id = hash(label);
        match op {
            "=" => {
                let focal_length = caps.get(3).unwrap().as_str().parse::<u8>().unwrap();
                boxes[box_id as usize].add_lens(label, focal_length);
            }
            "-" => {
                boxes[box_id as usize].remove_lens(label);
            }
            _ => unreachable!(),
        }
    });

    boxes
        .iter()
        .enumerate()
        .map(|(i, b)| b.get_focusing_power(i))
        .sum::<i64>() as u64
}

fn hash(input: &str) -> u64 {
    let mut current_value: i64 = 0;
    input.chars().for_each(|c| {
        //let a_code = c.to_digit(10).expect(&format!("failed to ascii ({})", c)) as i64;
        let a_code = c as u8;
        current_value += a_code as i64;
        current_value *= 17;
        current_value %= 256;
    });
    current_value as u64
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"
    }

    #[test]
    fn hash_hash() {
        assert_eq!(hash("HASH"), 52);
    }

    #[test]
    fn day15_part1() {
        assert_eq!(part1(&day15_parse(get_test_input())), 1320);
    }

    #[test]
    fn day15_part2() {
        assert_eq!(part2(&day15_parse(get_test_input())), 145);
    }
}
