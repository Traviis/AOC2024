use std::fmt;
use std::ops::{Deref, DerefMut};

type InputType = HardDrive;
type OutputType = u64;

type FileId = u64;
type Size = u64;

#[derive(Clone)]
pub struct HardDrive(Vec<Space>);

#[derive(Clone, Copy)]
pub enum Space {
    Empty(Size),
    File(FileId, Size),
}

impl fmt::Display for HardDrive {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for space in self.deref() {
            write!(f, "{}", space)?;
        }
        write!(f, "")
    }
}

impl HardDrive {
    fn checksum(&self) -> u64 {
        //  We could iterate through the list and do fancy math ORRRR we could just print out the values from my debug function and go from there

        let mut idx: u64 = 0;
        let mut running_total: u64 = 0;
        let mut c_idx: u64 = 0; //This counts the current index of the files if they were laid out on (printed)

        while idx < self.len() as u64 {
            match self[idx as usize] {
                Space::Empty(size) => {
                    c_idx += size;
                }
                Space::File(id, size) => {
                    #[cfg(test)]
                    println!("See file with ID: {} and size: {} at idx {}", id, size, idx);
                    for dx in 0..size {
                        #[cfg(test)]
                        println!("{} * {} = {}", id, size + c_idx, id * (dx + c_idx));
                        running_total += id * (dx + c_idx);
                    }
                    c_idx += size;
                }
            }
            idx += 1;
        }

        running_total
    }
}

impl Deref for HardDrive {
    type Target = Vec<Space>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for HardDrive {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let val = match self {
            Space::Empty(size) => (0..*size).map(|_| ".").collect::<String>(),
            Space::File(id, size) => (0..*size).map(|_| format!("{}", *id)).collect::<String>(),
            // The real input has IDs > 9... so don't use this for the real input
        };
        write!(f, "{}", val)
    }
}

#[aoc_generator(day9)]
fn day9_parse(input: &str) -> InputType {
    let mut id: u64 = 0;
    let mut is_file = true; //Start

    input.chars().fold(HardDrive(Vec::new()), |mut acc, c| {
        match c {
            '0'..='9' => {
                let size = c.to_digit(10).unwrap() as u64;
                if is_file {
                    acc.push(Space::File(id, size));
                    id += 1;
                } else {
                    acc.push(Space::Empty(size));
                }
                is_file = !is_file;
            }
            _ => {
                panic!("Invalid character in input '{}'", c);
            }
        }
        acc
    })
}

#[aoc(day9, part1)]
pub fn part1(input: &InputType) -> OutputType {
    let mut spaces: HardDrive = input.clone();
    #[cfg(test)]
    println!("Initial: {}", spaces);

    // The algorithm is to find empty spaces, then move ahead to the next file, and take as much as possible to try to satisfy the empty space, (which will split the file!)

    for idx in 0..spaces.len() {
        let space = &spaces[idx];
        match space {
            Space::File(_, _) => (), //We don't actually care about files,
            Space::Empty(size) => {
                //Find the next file and determine how much of it we can fill the space with
                let mut remaining = *size;
                let mut next_idx = spaces.len() - 1;
                if next_idx == idx + 1 {
                    //If we run out of items, just break
                    continue;
                }
                loop {
                    if let Space::File(id, file_size) = spaces[next_idx] {
                        if file_size >= remaining {
                            //We can't take the whole file, so we need to split it
                            spaces[idx] = Space::File(id, remaining);
                            spaces[next_idx] = Space::File(id, file_size - remaining);
                            #[cfg(test)]
                            println!(
                                "Splitting file {} into {} and {}",
                                id,
                                file_size - remaining,
                                remaining
                            );
                        } else {
                            //We can take the whole file
                            remaining -= file_size;
                            spaces[idx] = Space::File(id, file_size);
                            spaces[next_idx] = Space::Empty(0); //Should we just remove it?
                            spaces.insert(idx + 1, Space::Empty(remaining));
                            #[cfg(test)]
                            println!("Taking file {} of size {}", id, file_size);
                        }
                        break;
                    } else {
                        next_idx -= 1; //We found free space at the end, keep looking
                    }
                }
                #[cfg(test)]
                println!("Index: {}, {}", idx, spaces);
            }
        }
    }
    #[cfg(test)]
    println!("After: {}", spaces);

    spaces.checksum()
}

#[aoc(day9, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let mut spaces: HardDrive = input.clone();
    #[cfg(test)]
    println!("Initial: {}", spaces);

    for idx in (0..spaces.len()).rev() {
        match spaces[idx] {
            Space::Empty(..) => {}
            Space::File(id, size) => {
                //Going from the right, See if this file can be moved to the left
                //In effect, find a Free Space (starting from idx 0) that has <= size
                for forward_idx in 0..spaces.len() {
                    if forward_idx >= idx {
                        continue;
                    }
                    if let Space::Empty(forward_size) = spaces[forward_idx] {
                        if forward_size >= size {
                            //We can move the file to the left
                            spaces[forward_idx] = Space::File(id, size);
                            spaces[idx] = Space::Empty(size);
                            let remaining = forward_size - size;
                            if remaining > 0 {
                                spaces.insert(forward_idx + 1, Space::Empty(remaining));
                            }
                            #[cfg(test)]
                            println!("Index: {}, {}", idx, spaces);
                            break;
                        }
                    }
                }
            }
        };
    }
    #[cfg(test)]
    println!("After: {}", spaces);

    spaces.checksum()
}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "2333133121414131402"
    }

    #[test]
    fn day9_part1() {
        assert_eq!(part1(&day9_parse(get_test_input())), 1928);
    }

    #[test]
    fn day9_part2() {
        assert_eq!(part2(&day9_parse(get_test_input())), 2858);
    }
}
