use std::collections::HashMap;
use rayon::prelude::*;

type InputType = HashMap<(u64, u64), Thing>;
type OutputType = u64;

#[derive(Debug, Clone, PartialEq)]
pub enum Thing {
    Galaxy, // #
    Empty,  // .
}

fn expand_galaxy(input: &InputType, expansion: u64) -> InputType {
    //For every empty row and column, add a new empty row and column
    let mut new_input = input.clone();

    let mut max_x = *input.keys().map(|(x, _)| x).max().unwrap() as u64;
    let mut max_y = input.len() as u64;

    let mut expanded_rows = Vec::new();
    let mut expanded_columns = Vec::new();
    


    for y in 0..=max_y {
        let mut galaxy_in_row = false;
        for x in 0..=max_x {
            if let Some(thing) = new_input.get(&(x, y as u64)) {
                if let Thing::Galaxy = thing {
                    galaxy_in_row = true;
                    break;
                }
            }
        }

        if !galaxy_in_row {
            expanded_rows.push(y);
        }
    }

    //Now expand columns
    for x in 0..=max_x {
        let mut galaxy_in_column = false;
        for y in 0..=max_y {
            if let Some(thing) = new_input.get(&(x as u64, y)) {
                if let Thing::Galaxy = thing {
                    galaxy_in_column = true;
                    break;
                }
            }
        }

        if !galaxy_in_column {
            expanded_columns.push(x);
        }
    }

    max_x += expanded_columns.len()  as u64 * expansion;
    max_y += expanded_rows.len() as u64 * expansion;


    //TODO: This is slow when we have massive numbers, but the relative number of galaxies is really small, instead, iterate over the galaxies and see if any of them match up to the criteria we are looking for.
    /*
    //DRAFT BY GALAXY
    for y in expanded_rows.iter().rev() {
        //Expand the row by expansion if there is a galaxy in the row,
        // iterate over all galaxies and see if they fit this criteria
        let y = *y as usize;
        let valid_galaxies = input.keys().filter(|(_, y2)| *y2 >= y as u64).collect::<Vec<_>>();
        for (x,y) in valid_galaxies {
            if let Some(thing) = new_input.get(&(*x, *y)) {
                if let Thing::Galaxy = thing {
                    new_input.insert((*x, (*y as u64 + expansion) as u64), Thing::Galaxy);
                    //remove the "old" galaxy
                    new_input.remove(&(*x, *y));
                }
            }
        }

    }
    */
    //TODO: Replace each of th expansions with a single loop over the galaxies and see if they fit the criteria
    // Can do 1 at a time since the test data is pretty forgiving in terms of time

    for y in expanded_rows.iter().rev() {
        let y = *y as usize;
        //expand row (and modify all y values  beyond this +1)
        for dy in (y..=(max_y as usize)).rev() {
            for dx in 0..=max_x {
                if let Some(thing) = new_input.get(&(dx, dy as u64)) {
                    if let Thing::Galaxy = thing {
                        new_input.insert((dx, (dy as u64 + expansion) as u64), Thing::Galaxy);
                        //remove the "old" galaxy
                        new_input.remove(&(dx, dy as u64));
                    }
                }
            }
        }
    }

    for x in expanded_columns.iter().rev() {
        let x = *x as usize;
        //expand column (and modify all x values  beyond this +1)
        for dx in (x..=(max_x as usize)).rev() {
            for dy in 0..=max_y {
                if let Some(thing) = new_input.get(&(dx as u64, dy)) {
                    if let Thing::Galaxy = thing {
                        new_input.insert(((dx as u64 + expansion) as u64, dy), Thing::Galaxy);
                        //remove the "old" galaxy
                        new_input.remove(&(dx as u64, dy));
                    }
                }
            }
        }
    }



    new_input
}

#[cfg(test)]
fn dump_galaxy(input: &InputType) {
    let max_x = input.keys().map(|(x, _)| x).max().unwrap();
    let max_y = input.keys().map(|(_, y)| y).max().unwrap();

    for y in 0..=*max_y {
        for x in 0..=*max_x {
            if let Some(thing) = input.get(&(x, y)) {
                match thing {
                    Thing::Galaxy => print!("#"),
                    Thing::Empty => print!("."),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[aoc_generator(day11)]
fn day11_parse(input: &str) -> InputType {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            let line = line.trim();
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '#' {
                    Some(((x as u64, y as u64), Thing::Galaxy))
                } else {
                    None
                }
            })
        })
    .collect()
}

fn manhatten_distance(a: (u64, u64), b: (u64, u64)) -> u64 {
    ((a.0 as i64 - b.0 as i64).abs() + (a.1 as i64 - b.1 as i64).abs()) as u64
}

#[aoc(day11, part1)]

pub fn part1(input: &InputType) -> OutputType {
    solver(input, 1)
}

pub fn solver(input: &InputType, expansion_rate: u64) -> OutputType {
    let input = expand_galaxy(input,expansion_rate);
    input.keys().map(|(x,y)| {

        let mut dist = 0;
        for (x2,y2) in input.keys() {
            if x == x2 && y == y2 {
                continue;
            }
            dist += manhatten_distance((*x,*y), (*x2,*y2));

        }
        dist as u64

    }).sum::<u64>() / 2 
    //(I'm double counting)

}

#[aoc(day11, part2)]
pub fn part2(input: &InputType) -> OutputType {
    solver(input,1000000 - 1)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    }

    // #[test]
    // fn simple_galaxy_expansion() {
    // }

    #[test]
    fn galaxy_expansion() {
        let expanded_galaxy = "....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......";
            let input = day11_parse(get_test_input());
        let expanded = expand_galaxy(&input,1);
        let expanded_galaxy = day11_parse(expanded_galaxy);
        println!("Input");
        dump_galaxy(&input);
        println!("Expanded");
        dump_galaxy(&expanded);
        println!("Expanded Galaxy (target)");
        dump_galaxy(&expanded_galaxy);
        assert_eq!(expanded, expanded_galaxy);
    }

    #[test]
    fn day11_part1() {
        assert_eq!(part1(&day11_parse(get_test_input())), 374);
    }

    #[test]
    fn day11_part2() {
        assert_eq!(solver(&day11_parse(get_test_input()),9), 1030);
        assert_eq!(solver(&day11_parse(get_test_input()),99), 8410);
    }
}
