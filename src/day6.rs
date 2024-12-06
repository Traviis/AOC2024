use std::collections::{BTreeMap, BTreeSet};

type InputType = Map;
type OutputType = u64;

#[derive(Clone)]
pub struct Map {
    //I suspect part 2 will introduce multiple guards or sightlines, so let's keep this flexible
    guard: ((i32, i32), GuardFacing),
    visited: BTreeSet<(i32, i32)>, //May need to do this for each guard; for now, keep it simple
    map: BTreeMap<(i32, i32), Location>,
    max_x: i32,
    max_y: i32,

    // Part 2
    turn_locations: BTreeSet<(i32, i32)>, //let's track locations where we turn, because each of these can be part of a box
    possible_obstacle_locations: BTreeSet<(i32, i32)>, //let's track locations where we can place an obstacle
    trail: BTreeMap<(i32, i32), Trail>, //let's track the trail we've left behind
}

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
    EastWest, // -
    Cross, // +
    Unknown, //?
    Turn, //T
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

    fn current_trail(&self) -> Trail {
        match self {
            GuardFacing::North | GuardFacing::South => Trail::NorthSouth,
            GuardFacing::East | GuardFacing::West => Trail::EastWest,
        }
    }
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
            _ => panic!("Invalid character in map"),
        }
    }
}

#[aoc_generator(day6)]
fn day6_parse(input: &str) -> InputType {
    input.lines().enumerate().fold(
        Map {
            guard: ((-100, -100), GuardFacing::North),
            visited: BTreeSet::new(),
            map: BTreeMap::new(),
            max_x: 0,
            max_y: 0,
            possible_obstacle_locations: BTreeSet::new(),
            turn_locations: BTreeSet::new(),
            trail: BTreeMap::new(),
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
                    map.visited.insert((x as i32, y as i32));
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
    #[cfg(test)]
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
                //if self.turn_locations.contains(&(x, y)) {
                //    print!("T");
                //    continue;
                //}
                if self.visited.contains(&(x, y)) {
                    print!("{}", self.trail.get(&(x,y)).unwrap().into_char());
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

    fn simulate_until_exit(&mut self) {
        loop {
            // Turns out you could do this with a raycast, but I'll just iterate to keep it simple for now
            //Do a step
            let ((guard_x, guard_y), facing) = self.guard;

            //First, what's in front of the guard?
            //NOTE: coordinates are inverted for y
            let (dx, dy) = facing.get_delta_step();

            let (next_x, next_y) = (guard_x + dx, guard_y + dy);
            match self.map.get(&(next_x, next_y)) {
                Some(Location::Wall) => {
                    //Turn right
                    self.trail.insert((guard_x, guard_y), Trail::Turn);
                    self.guard = ((guard_x, guard_y), facing.turn_right());
                    self.turn_locations.insert((guard_x, guard_y));
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
                    if next_x < 0 || next_x >= self.max_x || next_y < 0 || next_y >= self.max_y {
                        //Guard walked out of bounds
                        break;
                    } else {
                        //Move forward
                        //TODO: Something is broken here. Trails that should be - are always + already, for some reason
                        let new_trail_symbol = match self.trail.get(&(guard_x,guard_y)) {
                            Some(Trail::NorthSouth) if facing == GuardFacing::East || facing == GuardFacing::West => Trail::Cross,
                            Some(Trail::EastWest) if facing == GuardFacing::North || facing == GuardFacing::South => Trail::Cross,
                            Some(Trail::NorthSouth) if facing == GuardFacing::North || facing == GuardFacing::South => Trail::NorthSouth,
                            Some(Trail::EastWest) if facing == GuardFacing::East || facing == GuardFacing::West => Trail::EastWest,
                            Some(&Trail::NorthSouth) | Some(&Trail::EastWest) | Some(Trail::Unknown) => panic!("Invalid trail"),
                            Some(Trail::Cross) => Trail::Cross,
                            Some(Trail::Turn) => Trail::Turn,
                            None => facing.current_trail(),
                        };
                        #[cfg(test)]
                        {
                        println!("Placing trail at {},{} with symbol {}", next_x, next_y, new_trail_symbol.into_char());
                        println!("Guard is at {},{} facing {}, symbol currently: {}", next_x, next_y, facing.into_char(), self.trail.get(&(guard_x,guard_y)).unwrap_or(&Trail::Unknown).into_char());
                        }

                        self.trail.insert((next_x, next_y), new_trail_symbol);
                        //Set a trail for where I have been

                        //Then move forward
                        self.guard = ((next_x, next_y), facing);
                        self.visited.insert((next_x, next_y));
                    }
                }
            } //end match
              //Part 2, Check if placing an obstacle right in front of the guard would complete a cycle
              //Let's just check every move
            let ((x, y), current_guard_facing) = self.guard;
            let new_facing = current_guard_facing.turn_right();

            //The theoretical position that the guard would turn right would be x, y, and facing new_facing
            //From there, cast a ray until you hit an obstacle
            let (ray_dx, ray_dy) = new_facing.get_delta_step();

            let mut ray_x = x;
            let mut ray_y = y;

            // This new possible obstacle (and thus turn) would require that ray_x and ray_y eventually hit an obstacle
            //while self.map.get(&(ray_x, ray_y)).is_none() && ray_x >= 0 && ray_x <= self.max_x && ray_y >= 0 && ray_y <= self.max_y {
            while !self.map.contains_key(&(ray_x, ray_y))
                && ray_x >= 0
                && ray_x <= self.max_x
                && ray_y >= 0
                && ray_y <= self.max_y
            {
                ray_x += ray_dx;
                ray_y += ray_dy;
            }

            //step back one
            ray_x -= ray_dx;
            ray_y -= ray_dy;

            //println!("Finalized ray_cast {} {}", ray_x, ray_y);
            //self.dump_map(x,y,-100,100);

            //if (ray_x + ray_dx) < 0 || (ray_x + ray_dx) > self.max_x || (ray_y + ray_dy) < 0 || (ray_y + ray_dy) > self.max_y {
            //    // We hit the edge of the map, so we can't place an obstacle here
            //    //TODO: Special case: Check again if turning right and continueing would make a box
            //} else {
            let (dx, dy) = current_guard_facing.get_delta_step();
            //We know we hit a box if there is a wall right in front of the guard AND that location has already been visited
            if self.visited.contains(&(ray_x, ray_y)) && !(ray_x == x && ray_y == y) {
                self.possible_obstacle_locations.insert((x + dx, y + dy));
                #[cfg(test)]
                {
                    println!("Found a possible location at {},{}", x + dx, y + dy);
                    self.dump_map(x, y, x + dx, y+ dy);
                }
            }
            //}
        }
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

    // We have the turn locations from the simulation, 3 of those can form parts of the box, and it
    // should intersect with the first position at some point, there, we can put in a box

    map.possible_obstacle_locations.len() as u64
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
