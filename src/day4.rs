use std::collections::BTreeMap;

type InputType = BTreeMap<(i32, i32), char>;
type OutputType = u64;

#[aoc_generator(day4)]
fn day4_parse(input: &str) -> InputType {
    let mut set = BTreeMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            set.insert((x as i32, y as i32), c);
        }
    }

    set
}

fn look_for_letter(input: &InputType, x: i32, y: i32, letter: char) -> Vec<(i32, i32)> {
    let mut vecs = Vec::new();
    for dx in -1..=1 {
        for dy in -1..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }

            if let Some(c) = input.get(&(x + dx, y + dy)) {
                if c == &letter {
                    vecs.push((x + dx, y + dy));
                }
            }
        }
    }

    vecs
}

#[cfg(debug_assertions)]
use std::collections::BTreeSet;

#[allow(dead_code)]
#[cfg(debug_assertions)]
fn dump_map(input: &InputType, highlights: BTreeSet<(i32, i32)>) {
    println!("dumping map");
    let max_x = *input.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *input.keys().map(|(_, y)| y).max().unwrap();
    for x in 0..=max_x {
        for y in 0..=max_y {
            if highlights.contains(&(x, y)) {
                print!("{}", input.get(&(x, y)).unwrap().to_uppercase());
            } else if highlights.is_empty() {
                print!("{}", input.get(&(x, y)).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //Iterate through the entire map, looking for X, if you find X, look in all directions for M, if you find an M, go ahead and check the down that path to see if it matches up

    let mut count = 0;
    for ((x, y), c) in input.iter() {
        if *c == 'X' {
            let m_possibilities = look_for_letter(input, *x, *y, 'M');
            //Now that we have all the M possibilities, we need to look for A, but ONLY in the direction of the M
            for (m_pos_x, m_pos_y) in m_possibilities {
                let dx = m_pos_x + x * -1;
                let dy = m_pos_y + y * -1;
                // dx, dy contains the direction of the M
                if let Some(c) = input.get(&(m_pos_x + dx, m_pos_y + dy)) {
                    if *c == 'A' {
                        let a_pos_x = m_pos_x + dx;
                        let a_pos_y = m_pos_y + dy;
                        let dx = a_pos_x - m_pos_x;
                        let dy = a_pos_y - m_pos_y;
                        if let Some(c) = input.get(&(a_pos_x + dx, a_pos_y + dy)) {
                            if *c == 'S' {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }
    count
}

#[aoc(day4, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let mut count = 0;
    // Now we need to find the Crosses, but the letters can be jumbled. To do this, we can find all A's in the map, and then check the diagnols for M's and S's.
    for ((x,y), c) in input.iter() {
        if *c == 'A' {
            let bottom_left = input.get(&(x-1, y+1));
            let bottom_right = input.get(&(x+1, y+1));
            let top_left = input.get(&(x-1, y-1));
            let top_right = input.get(&(x+1, y-1));

            if bottom_left.is_none() || bottom_right.is_none() || top_left.is_none() || top_right.is_none() {
                //If you're on an edge, skip completely
                continue;
            }

            //Clean up and unwrap
            let bottom_left = *bottom_left.unwrap();
            let bottom_right = *bottom_right.unwrap();
            let top_left = *top_left.unwrap(); 
            let top_right = *top_right.unwrap();

            //Check the slashes, both must be true
            let forward_slash_good = (bottom_right == 'M' && top_left == 'S') || (bottom_right == 'S' && top_left == 'M');
            let back_slash_good = (bottom_left == 'M' && top_right == 'S') || (bottom_left == 'S' && top_right == 'M');

            if forward_slash_good && back_slash_good {
                count += 1;
            }

        }
    }
    count
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX"
    }

    #[test]
    fn day4_part1() {
        assert_eq!(part1(&day4_parse(get_test_input())), 18);
    }

    #[test]
    fn day4_part2() {
        assert_eq!(part2(&day4_parse(get_test_input())), 9);
    }
}
