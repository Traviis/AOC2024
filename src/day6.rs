use std::collections::{BTreeMap, BTreeSet};

type InputType = Map;
type OutputType = u64;

#[derive(Clone)]
pub struct Map {
    //I suspect part 2 will introduce multiple guards or sightlines, so let's keep this flexible
    guard: ((i32, i32), GuardFacing),
    initial_guard_position: ((i32, i32), GuardFacing),
    visited: BTreeMap<(i32, i32),i32>, //May need to do this for each guard; for now, keep it simple
    map: BTreeMap<(i32, i32), Location>,
    max_x: i32,
    max_y: i32,
}

static LOOP_THRESHOLD: i32 = 10;

#[derive(Clone, Copy, PartialEq)]
pub enum GuardFacing {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy)]
pub enum Location {
    Empty,
    Wall,
    Guard(GuardFacing),
}

impl GuardFacing {
    fn from_char(c: char) -> GuardFacing {
        match c {
            '^' => GuardFacing::North,
            '>' => GuardFacing::East,
            'v' => GuardFacing::South,
            '<' => GuardFacing::West,
            _ => panic!("Invalid character for guard facing"),
        }
    }

    fn turn_right(&self) -> GuardFacing {
        match self {
            GuardFacing::North => GuardFacing::East,
            GuardFacing::East => GuardFacing::South,
            GuardFacing::South => GuardFacing::West,
            GuardFacing::West => GuardFacing::North,
        }
    }

    fn get_delta_step(&self) -> (i32, i32) {
        match self {
            GuardFacing::North => (0, -1),
            GuardFacing::East => (1, 0),
            GuardFacing::South => (0, 1),
            GuardFacing::West => (-1, 0),
        }
    }

    #[allow(dead_code)]
    fn into_char(self) -> char {
        match self {
            GuardFacing::North => '^',
            GuardFacing::East => '>',
            GuardFacing::South => 'v',
            GuardFacing::West => '<',
        }
    }
}

impl Location {
    fn from_char(c: char) -> Location {
        match c {
            '.' => Location::Empty,
            '#' => Location::Wall,
            '^' | '>' | 'v' | '<' => Location::Guard(GuardFacing::from_char(c)),
            c => panic!("Invalid character in map '{}'",c),
        }
    }
}

#[aoc_generator(day6)]
fn day6_parse(input: &str) -> InputType {
    input.lines().enumerate().fold(
        Map {
            guard: ((-100, -100), GuardFacing::North),
            initial_guard_position: ((-100, -100), GuardFacing::North),
            visited: BTreeMap::new(),
            map: BTreeMap::new(),
            max_x: 0,
            max_y: 0,
        },
        |mut map, (y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if x as i32 > map.max_x {
                    map.max_x = x as i32;
                }
                if y as i32 > map.max_y {
                    map.max_y = y as i32;
                }
                let loc = Location::from_char(c);
                if let Location::Guard(g) = loc {
                    map.guard = ((x as i32, y as i32), g);
                    map.initial_guard_position= ((x as i32, y as i32), g);
                    map.visited.insert((x as i32, y as i32),1);
                } else if let Location::Wall = loc {
                    map.map.insert((x as i32, y as i32), loc);
                } else {
                    //Do nothing, we don't need empty locations
                }
            });
            map
        },
        )
}

impl Map {
    #[allow(dead_code)]
    fn dump_map(&self, corner_x: i32, corner_y: i32, obstacle_x: i32, obstacle_y: i32) {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                if x == corner_x && y == corner_y {
                    print!("X");
                    continue;
                }

                if x == obstacle_x && y == obstacle_y {
                    print!("O");
                    continue;
                }
                if let Some(count) = self.visited.get(&(x, y)) {
                    print!("{}", count);
                    continue;
                }
                let loc = self.map.get(&(x, y)).unwrap_or(&Location::Empty);
                match loc {
                    Location::Empty => print!("."),
                    Location::Wall => print!("#"),
                    Location::Guard(facing) => print!("{}", facing.into_char()),
                }
            }
            println!();
        }
    }

    fn find_infinite_loop(&self, obstacle: (i32,i32)) -> bool {

        //I don't know why my original code works for the test case, but not my input, instead of trying to figure out why, just brute force this.

        //Inject the obstacle into the map, run the simulation, and see if we end up in a loop
        let mut map = self.clone();
        //Inject our fake obstacle
        map.map.insert(obstacle, Location::Wall);
        map.guard = map.initial_guard_position; //Reset

        #[cfg(test)]
        map.dump_map(-100, -100, obstacle.0, obstacle.1);

        map.simulate_until_exit()
    }

    //Return true if we got stuck in a loop
    fn simulate_until_exit(&mut self) -> bool {
        loop {
            //Do a step
            let ((guard_x, guard_y), facing) = self.guard;

            //First, what's in front of the guard?
            //NOTE: coordinates are inverted for y
            let (dx, dy) = facing.get_delta_step();

            let (next_x, next_y) = (guard_x + dx, guard_y + dy);
            match self.map.get(&(next_x, next_y)) {
                Some(Location::Wall) => {
                    //Turn right
                    self.guard = ((guard_x, guard_y), facing.turn_right());
                    //self.turn_locations.insert((guard_x, guard_y));
                    *self.visited.entry((guard_x, guard_y)).or_insert(0) += 1;
                }
                Some(Location::Empty) => {
                    //Move forward
                    panic!("We don't have explicit empties in here");
                }
                Some(Location::Guard(_)) => {
                    panic!("Guards can't run into each other");
                }
                None => {
                    // There is nothing here, BUT, we can't move off the map
                    if next_x < 0 || next_x > self.max_x || next_y < 0 || next_y > self.max_y {
                        //Guard walked out of bounds
                        return false; //Not a loop
                    } else {
                        //Move forward
                        self.guard = ((next_x, next_y), facing);
                        *self.visited.entry((next_x, next_y)).or_insert(0) += 1;
                        let visits = self.visited.get(&(next_x, next_y)).unwrap();
                        #[cfg(test)]
                        self.dump_map(guard_x, guard_y, -100, -100);
                        if *visits > LOOP_THRESHOLD {
                            return true;
                        }

                    }
                }
            }
        } //end match
    }
}

#[aoc(day6, part1)]
    pub fn part1(input: &InputType) -> OutputType {
        //It's possible that we can optimize by using raycasting to determine how far to move, but I
        //have a suspicion that there will be multiple guards that end up running into eachother or
        //something, so lets go through each step

        let mut map = input.clone();

        map.simulate_until_exit();

        map.visited.len() as u64
    }

#[aoc(day6, part2)]
    pub fn part2(input: &InputType) -> OutputType {
        let mut map = input.clone();

        //First, do part one,
        map.simulate_until_exit();

        //Find all the visited locations
        let mut visited_locations = map.visited.keys().cloned().collect::<BTreeSet<(i32, i32)>>();
        visited_locations.remove(&map.initial_guard_position.0); //Except the initial position

        visited_locations.iter().filter(|(x,y)| map.find_infinite_loop((*x,*y))).count() as u64

    }

#[cfg(test)]
    mod tests {

        use super::*;

        fn get_test_input() -> &'static str {
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."
        }

        #[test]
        fn day6_part1() {
            assert_eq!(part1(&day6_parse(get_test_input())), 41);
        }
        #[test]
        fn day6_part2() {
            assert_eq!(part2(&day6_parse(get_test_input())), 6);
        }
    }
