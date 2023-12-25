use regex::Regex;

type InputType = Vec<Instruction>;
type OutputType = u64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Instruction {
    direction: Direction,
    steps: u64,
    color_code: String,
}

#[aoc_generator(day18)]
fn day18_parse(input: &str) -> InputType {
    let pat = Regex::new(r"([LURD]) (\d+) \((#[0-9a-f]{6})\)").unwrap();

    input
        .lines()
        .map(|line| {
            let pats = pat.captures(line).unwrap();

            let dir = match &pats[1] {
                "L" => Direction::Left,
                "U" => Direction::Up,
                "R" => Direction::Right,
                "D" => Direction::Down,
                _ => panic!("Invalid direction {}", &pats[0]),
            };
            let steps = pats[2].parse::<u64>().unwrap();
            let color_code = pats[3].to_string();
            Instruction {
                direction: dir,
                steps,
                color_code,
            }
        })
        .collect()
}

//Get the polygon that is the border of the map
fn get_sparse_border(input: &InputType) -> (Vec<(i64, i64)>, i64) {
    let mut cur_x = 0;
    let mut cur_y = 0;

    let mut boundary_points = 0;

    let mut map = input
        .iter()
        .map(|inst| {
            match inst.direction {
                Direction::Up => cur_y += inst.steps as i64,
                Direction::Down => cur_y -= inst.steps as i64,
                Direction::Left => cur_x -= inst.steps as i64,
                Direction::Right => cur_x += inst.steps as i64,
            }
            boundary_points += inst.steps;
            (cur_x, cur_y)
        })
        .collect::<Vec<(i64, i64)>>();

    //Push 0,0 onto the front
    map.insert(0, (0, 0));
    (map, boundary_points as i64)
}

#[aoc(day18, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //First use the instructions to dig a trench in the ground, this s hould be a complete circuit
    //and it also paints the edges while you go

    let (map, boundary_points) = get_sparse_border(input);

    //Shoelace formula is .5 * abs(sum(x_i*y_i+1 - x_i+1*y_i))
    //https://en.wikipedia.org/wiki/Shoelace_formula

    //Turns out we don't care about the color, just the shape

    let n = map.len();
    let mut area: f64 = 0.0;

    for i in 0..n {
        let j = (i + 1) % n;
        area += (map[i].0 * map[j].1) as f64;
        area -= (map[j].0 * map[i].1) as f64;
    }

    area = area.abs() / 2.0;

    let area = area as i64;

    //picks theorum
    //https://en.wikipedia.org/wiki/Pick%27s_theorem
    let i = (area - (boundary_points / 2)) + 1;

    (i + boundary_points) as u64

    //This works, but it's really slow, instead let's use the shoelace formula
    /*
    // Now that we have the border dug out, let's use the same algorithm as before to find the
    // inner parts that need to be filled in (as opposed to flood fill) (knot theory day10)

    let min_x = *map.keys().map(|(x, _)| x).min().unwrap();
    let max_x = *map.keys().map(|(x, _)| x).max().unwrap();
    let min_y = *map.keys().map(|(_, y)| y).min().unwrap();
    let max_y = *map.keys().map(|(_, y)| y).max().unwrap();



    for y in min_y..=max_y {
        let mut loop_count = 0;
        for x in min_x..=max_x {
            let segment = map.get(&(x, y));
            let above = map.get(&(x, y + 1));
            if above.is_some() && segment.is_some() {
                loop_count += 1;
            }
            if loop_count % 2 == 1 && segment.is_none() {
                map.insert((x, y), Tile::Dug);
            }
        }
    }


    #[cfg(test)]
    dump_map(&map);

    map.len() as u64
    */
}

fn convert_inst(inst: &Instruction) -> Instruction {
    let raw_hex = &inst.color_code[1..6];
    let hex_dist = u64::from_str_radix(raw_hex, 16).unwrap();
    let dir = match &inst.color_code[6..] {
        "0" => Direction::Right,
        "1" => Direction::Down,
        "2" => Direction::Left,
        "3" => Direction::Up,
        _ => panic!("Invalid direction {}", &inst.color_code[5..]),
    };
    Instruction {
        direction: dir,
        steps: hex_dist,
        color_code: inst.color_code.clone(),
    }
}

#[aoc(day18, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let fixed_map = input
        .iter()
        .map(|inst| convert_inst(inst))
        .collect::<Vec<_>>();

    part1(&fixed_map)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "R 6 (#70c710)
            D 5 (#0dc571)
            L 2 (#5713f0)
            D 2 (#d2c081)
            R 2 (#59c680)
            D 2 (#411b91)
            L 5 (#8ceee2)
            U 2 (#caa173)
            L 1 (#1b58a2)
            U 2 (#caa171)
            R 2 (#7807d2)
            U 3 (#a77fa3)
            L 2 (#015232)
            U 2 (#7a21e3)"
    }

    #[test]
    fn day18_part2_parse() {
        let bad_inst = Instruction {
            direction: Direction::Right,
            steps: 6,
            color_code: "#70c710".to_string(),
        };

        let converted = convert_inst(&bad_inst);

        assert_eq!(converted.direction, Direction::Right);
        assert_eq!(converted.steps, 461937);
    }

    #[test]
    fn day18_part1() {
        assert_eq!(part1(&day18_parse(get_test_input())), 62);
    }

    #[test]
    fn day18_part2() {
        assert_eq!(part2(&day18_parse(get_test_input())), 952408144115);
    }
}
