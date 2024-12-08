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

#[derive(Clone, Copy)]
pub enum Trail {
    NorthSouth, // |
    EastWest,   // -
    Cross,      // +
    Unknown,    //?
    Turn,       //T
}

impl Trail {
    fn into_char(self) -> char {
        match self {
            Trail::NorthSouth => '|',
            Trail::EastWest => '-',
            Trail::Cross => '+',
            Trail::Unknown => '?',
            Trail::Turn => 'T',
        }
    }
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

    fn into_char(&self) -> char {
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

    fn find_infinite_loop(&self, obstacle: (i32,i32)) -> Option<(i32,i32)> {

        //I don't know why my original code works for the test case, but not my input, instead of trying to figure out why, just brute force this.

        //Inject the obstacle into the map, run the simulation, and see if we end up in a loop
        let mut map = self.clone();
        //Inject our fake obstacle
        map.map.insert(obstacle, Location::Wall);
        map.guard = map.initial_guard_position;
        #[cfg(test)]
        map.dump_map(-100, -100, obstacle.0, obstacle.1);
        let looped = map.simulate_until_exit();
        if looped {
            Some(obstacle)
        } else {
            None
        }

        /*
        //Part 2, Check if placing an obstacle right in front of the guard would complete a cycle
        //Let's just check every move
        let ((x, y), current_guard_facing) = self.guard;
        //Simulate turning right
        let new_facing = current_guard_facing.turn_right();

        //The theoretical position that the guard would turn right would be x, y, and facing new_facing
        //From there, cast a ray until you hit an obstacle
        let (ray_dx, ray_dy) = new_facing.get_delta_step();

        let mut ray_x = x;
        let mut ray_y = y;

        // This new possible obstacle (and thus turn) would require that ray_x and ray_y eventually hit an obstacle
        while !self.map.contains_key(&(ray_x, ray_y))
            && ray_x >= 0
                && ray_x <= self.max_x
                && ray_y >= 0
                && ray_y <= self.max_y
        {
            ray_x += ray_dx;
            ray_y += ray_dy;
        }

        //if ray_x == self.initial_guard_position.0.0 && ray_y == self.initial_guard_position.0.1 {
        //    // If we're at the initial position, don't count it
        //    break;
        //}

        //step back one; these are coordinates of the obstacle
        ray_x -= ray_dx;
        ray_y -= ray_dy;

        //println!("Finalized ray_cast {} {}", ray_x, ray_y);
        //self.dump_map(x,y,-100,100);

        let (dx, dy) = current_guard_facing.get_delta_step();
        //We know we hit a box if there is a wall right in front of the guard AND that location has already been visited
        if self.visited.contains_key(&(ray_x, ray_y)) && !(ray_x == x && ray_y == y) {
            self.possible_obstacle_locations.insert((x + dx, y + dy));
            #[cfg(test)]
            {
                println!("Found a possible location at {},{}", x + dx, y + dy);
                self.dump_map(x, y, x + dx, y + dy);
            }
        }
        */
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

                        //Then move forward
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
        //I overcomplicated this, there are not multiple guards, so I an just ignore that stuff.

        //We need to detect a box, and then put items in that would create it, which, given the rule,
        //means that the box need to be bounded by an object placed in front of the guard, that would
        //force him to turn right, one of the box corners, that doesn't currently have an object.

        let mut map = input.clone();

        //First, do part one,
        map.simulate_until_exit();

        //Find all the visited locations
        let mut visited_locations = map.visited.keys().cloned().collect::<BTreeSet<(i32, i32)>>();
        visited_locations.remove(&map.initial_guard_position.0); //Except the initial position

        let mut possible_locations = 0;

        //Now, add an obstacle to each visitied location, and see if we loop
        for possible_obstacle in visited_locations {
            if let Some(obstacle) = map.find_infinite_loop(possible_obstacle) {
                //println!("Found a possible location at {},{}", obstacle.0, obstacle.1);
                possible_locations += 1;
            }
        }


        // We have the turn locations from the simulation, 3 of those can form parts of the box, and it
        // should intersect with the first position at some point, there, we can put in a box

        possible_locations
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
        //Current

        //Found a possible location at 3,6 V
        //Found a possible location at 6,7 V
        //Found a possible location at 3,8
        //Found a possible location at 1,8
        //Found a possible location at 7,7 //Out of order?
        //Found a possible location at 7,9

        //TODO: I'm getting the right answer, and even the correct coordinates, but I'm discovering
        //them "out of order" from the example, so something is probably subtley wrong here,
        #[test]
        fn day6_part2() {
            //Should be turns obstacles at:
            // 3,6
            // 6,7
            // 7,7
            // 1,8
            // 3,8
            // 7,9
            assert_eq!(part2(&day6_parse(get_test_input())), 6);
        }
    }
