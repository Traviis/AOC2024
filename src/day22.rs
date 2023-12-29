use std::{
    cmp::{max, min},
    fmt::{Display, Formatter},
};

type InputType = Vec<Brick>;
type OutputType = i64;

#[derive(Debug, Clone, Copy,PartialEq,Eq)]
pub struct Brick {
    x: i64,
    x_2: i64,
    y: i64,
    y_2: i64,
    z: i64,
    z_2: i64,
}

#[aoc_generator(day22)]
fn day22_parse(input: &str) -> InputType {
    input
        .lines()
        .map(|line| {
            let mut parts = line.trim().split('~');
            let mut parts_1 = parts.next().unwrap().split(',');
            let mut parts_2 = parts.next().unwrap().split(',');
            Brick {
                x: parts_1.next().unwrap().parse().unwrap(),
                x_2: parts_2.next().unwrap().parse().unwrap(),
                y: parts_1.next().unwrap().parse().unwrap(),
                y_2: parts_2.next().unwrap().parse().unwrap(),
                z: parts_1.next().unwrap().parse().unwrap(),
                z_2: parts_2.next().unwrap().parse().unwrap(),
            }
        })
        .collect()
}

impl Display for Brick {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{},{},{},{},{},{}",
            self.x, self.y, self.z, self.x_2, self.y_2, self.z_2
        )
    }
}

impl Brick {
    //Do they overlap? Ignore Z
    fn overlap_from_top(&self, other: &Brick) -> bool {
        max(self.x, other.x) <= min(self.x_2, other.x_2)
            && max(self.y, other.y) <= min(self.y_2, other.y_2)
    }
}

fn settle_bricks(bricks: &mut InputType) {
    bricks.sort_by_key(|brick| brick.z);

    //Make all the bricks fall
    for i in 0..bricks.len() {
        let mut max_z = 1;
        //Check all the bricks below
        for check in bricks.iter().skip(i + 1) {
            if bricks[i].overlap_from_top(check) {
                max_z = max(max_z, check.z_2 + 1);
            }
        }
        bricks[i].z_2 -= bricks[i].z - max_z;
        bricks[i].z = max_z;
    }

    bricks.sort_by_key(|brick| brick.z);
}

#[aoc(day22, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let mut bricks = input.clone();

    //Apply gravity and make all the bricks fall down
    settle_bricks(&mut bricks);

    for brick in bricks.iter() {
        println!("{}", brick);
    }
    0
}

#[aoc(day22, part2)]
pub fn part2(input: &InputType) -> OutputType {
    todo!();
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
    }

    fn settled_raw() -> &'static str {
        "1,0,1~1,2,1
0,0,2~2,0,2
0,2,2~2,2,2
0,0,3~0,2,3
2,0,3~2,2,3
0,1,4~2,1,4
1,1,5~1,1,6"
    }

    #[test]
    fn day22_settle() {
        let mut bricks = day22_parse(get_test_input());
        settle_bricks(&mut bricks);
        assert_eq!(bricks, day22_parse(settled_raw()));
    }

    #[test]
    fn day22_part1() {
        assert_eq!(part1(&day22_parse(get_test_input())), 5);
    }

    #[test]
    fn day22_part2() {
        assert_eq!(part2(&day22_parse(get_test_input())), 0);
    }
}
