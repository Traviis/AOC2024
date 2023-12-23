use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

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

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Ord)]
struct Node {
    x: i64,
    y: i64,
    dir: Direction,
    steps: u64,
    heat_loss: u64,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.heat_loss.cmp(&other.heat_loss))
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &InputType) -> OutputType {
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

    while prio_queue.len() > 0 {
        let Reverse(node) = prio_queue.pop().unwrap();

        if input.get(&(node.x, node.y)).is_none() {
            //If you're off the map, nope
            continue;
        }

        //hack

        //We don't need to staore the heat loss,
        if seen.contains() {
            //If you've already been here, nope
            continue;
        }
        let dir = dir.clone();

        seen.insert(&(x, y, dir, steps));
    }

    0
}

#[aoc(day17, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
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
        assert_eq!(part1(&day17_parse(get_test_input())), 0);
    }

    #[test]
    fn day17_part2() {
        assert_eq!(part2(&day17_parse(get_test_input())), 0);
    }
}
