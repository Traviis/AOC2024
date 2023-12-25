use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::hash::{Hash, Hasher};

type Coordinate = (i64, i64);
type InputType = HashMap<Coordinate, i64>;
type OutputType = u64;

#[aoc_generator(day17)]
fn day17_parse(input: &str) -> InputType {
    input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, c)| ((x as i64, y as i64), c.to_digit(10).unwrap() as i64))
        })
        .collect()
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

#[derive(Clone, Debug, PartialEq, Eq, Copy, Ord, Hash)]
struct Node {
    x: i64,
    y: i64,
    dir: Direction,
    steps: i64,
    heat_loss: i64,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.heat_loss.cmp(&other.heat_loss))
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &InputType) -> OutputType {
    djik(1, 3, input)
}
fn djik(min_move: i64, max_move: i64, input: &InputType) -> OutputType {
    //Use djikstra's algorithm but don't go over the graph as if it was jsut the graph, I need to
    //encode the directions into a much larger graph that understands that the map is larger than
    //just that (each node has all the possible moves from it

    //The state map is actually (x,y,dir,steps) for each of the nodes
    let mut seen = HashSet::new();
    let mut prio_queue = BinaryHeap::new();

    let start_node = Node {
        x: 0,
        y: 0,
        dir: Direction::None,
        steps: 0,
        heat_loss: 0,
    };
    prio_queue.push(Reverse(start_node));

    //The destination is the bottom right
    let max_x = *input.keys().map(|(x, _)| x).max().unwrap();
    let max_y = *input.keys().map(|(_, y)| y).max().unwrap();

    while prio_queue.len() > 0 {
        let Reverse(node) = prio_queue.pop().unwrap();

        #[cfg(test)]
        println!("{:?}", node);

        if node.x == max_x && node.y == max_y && node.steps >= min_move {
            return node.heat_loss as u64;
        }

        let set_key = (node.x, node.y, node.dir, node.steps);
        //We don't need to staore the heat loss,
        if seen.contains(&set_key) {
            //If you've already been here, nope
            continue;
        }
        // You don't actually need the heat loss in here, but just keep it simple
        seen.insert(set_key);

        if node.steps < max_move && node.dir != Direction::None {
            let mut new_node = node.clone();
            new_node.steps += 1;
            match node.dir {
                Direction::North => new_node.y -= 1,
                Direction::South => new_node.y += 1,
                Direction::East => new_node.x += 1,
                Direction::West => new_node.x -= 1,
                Direction::None => {}
            }

            let (x, y) = (new_node.x, new_node.y);
            new_node.heat_loss += *input.get(&(x, y)).unwrap_or(&0);

            if input.get(&(new_node.x, new_node.y)).is_some() {
                prio_queue.push(Reverse(new_node));
            }
        }
        //Regardless of going forward, we can also turn, let's just check each direction
        if node.steps >= min_move || node.dir == Direction::None {
            for (dx, dy) in [(0, -1), (0, 1), (1, 0), (-1, 0)].iter() {
                //Don't go back the way you came, also don't go in the forward direction (we already did that)
                let dx = *dx;
                let dy = *dy;
                match node.dir {
                    Direction::North | Direction::South => {
                        if dx == 0 && (dy == -1 || dy == 1) {
                            continue;
                        }
                    }
                    Direction::East | Direction::West => {
                        if dy == 0 && (dx == -1 || dx == 1) {
                            continue;
                        }
                    }
                    Direction::None => {}
                }

                let new_x = node.x + dx;
                let new_y = node.y + dy;

                //Push the other valie moves
                let mut new_node = node.clone();
                new_node.x = new_x;
                new_node.y = new_y;
                new_node.dir = match (dx, dy) {
                    (0, n) if n == -1 => Direction::North,
                    (0, n) if n == 1 => Direction::South,
                    (n, 0) if n == 1 => Direction::East,
                    (n, 0) if n == -1 => Direction::West,
                    _ => panic!("Invalid direction"),
                };
                new_node.steps = 1;
                new_node.heat_loss += *input.get(&(new_node.x, new_node.y)).unwrap_or(&0);

                if input.get(&(new_node.x, new_node.y)).is_some() {
                    prio_queue.push(Reverse(new_node));
                }
            }
        }
    }

    unreachable!()
}

#[aoc(day17, part2)]
pub fn part2(input: &InputType) -> OutputType {
    djik(4, 10, input)
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
    }

    #[test]
    fn day17_part1() {
        assert_eq!(part1(&day17_parse(get_test_input())), 102);
    }

    #[test]
    fn day17_part2() {
        assert_eq!(part2(&day17_parse(get_test_input())), 94);
    }
}
