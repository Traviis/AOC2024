
use std::collections::HashMap;
use itertools::Itertools as _; //just import traits

pub type SeedList = Vec<u64>;

pub type InputType = (SeedList,HashMap<(Thing,Thing),Vec<Mapping>>);
type OutputType = u64;

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Thing {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[derive(Debug, Clone)]
pub struct Mapping {
    length: u64,
    source: u64,
    dest: u64,
}

#[aoc_generator(day5)]
fn day5_parse(input: &str) -> InputType {
    //Well, first load all of those numbers into maps
    // The first number in the map is the destination range, the second number is the source range,
    // and the third number is how long the range is.
    // If the value isn't in the range, then it maps itself to itself (but we don't have to care
    // about that in the parsing part.

    let mut lines = input.lines();
    let seeds = lines
        .next()
        .unwrap()
        .split(": ")
        .skip(1)
        .next()
        .unwrap()
        .split(" ")
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<u64>>();

    //The input is in the same order as the test data, so we can parse that here
    let mut mappings = HashMap::new();

    let map_order = vec![
        (Thing::Seed, Thing::Soil),
        (Thing::Soil, Thing::Fertilizer),
        (Thing::Fertilizer, Thing::Water),
        (Thing::Water, Thing::Light),
        (Thing::Light, Thing::Temperature),
        (Thing::Temperature, Thing::Humidity),
        (Thing::Humidity, Thing::Location)];

    lines.next(); //Initial Space

    for cur_map in map_order {
        let mut number_map = Vec::new();

        lines.next(); //map header

        while let Some(mapping) = lines.next() {
            if mapping == "" {
                //we hit a space
                break;
            }

            #[cfg(test)]
            println!("Mapping: {:?}", mapping);
            let mut num_mapping = mapping.trim().split(" ").map(|x| x.parse::<u64>().unwrap());
            number_map.push(Mapping {
                dest: num_mapping.next().unwrap(),
                source: num_mapping.next().unwrap(),
                length: num_mapping.next().unwrap(),
            });
            //#[cfg(test)]
            //println!("{:?} -> {:?}: {:?}", cur_map.0, cur_map.1, number_map.last().unwrap());


        }
        mappings.insert(cur_map, number_map);
    }

    (seeds, mappings)

}

#[aoc(day5, part1)]
pub fn part1(input: &InputType) -> OutputType {
    //I'm suspect that this will end up with overlapping ranges, but try the naive approach first and just
    let (seeds, mappings) = input;

    let map_order = vec![
        (Thing::Seed, Thing::Soil),
        (Thing::Soil, Thing::Fertilizer),
        (Thing::Fertilizer, Thing::Water),
        (Thing::Water, Thing::Light),
        (Thing::Light, Thing::Temperature),
        (Thing::Temperature, Thing::Humidity),
        (Thing::Humidity, Thing::Location)];

    seeds.iter().map(|seed| {
        let mut cur_value = *seed;
        for current_map_type in &map_order {
            let current_map = mappings.get(&current_map_type).unwrap();
            // Now that we have the current translation map, we need to see if the current seed
            // is in the current map, if it is, then we just translate it and move on to the
            // next map, if it's not, we assume it's the same value
            // Be naive for now, and assume that there isn't a partial range
            let found_mapping = current_map.iter().find(|x| x.source <= cur_value && x.source + x.length > cur_value);

            let mapped_value = match found_mapping {
                Some(n) => cur_value - n.source + n.dest,
                None => cur_value,
            };
            #[cfg(test)]
            println!("{:?} -> {:?} -> {:?}", seed, current_map_type, mapped_value);
            cur_value = mapped_value
        }
        #[cfg(test)]
        println!("Seed {:?} -> Location {:?}", seed, cur_value);

        cur_value
        })
.min().unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let (raw_seeds_ranges, mappings) = input;
    //The seeds themselves are actually ranges

    let seed_ranges = raw_seeds_ranges.iter().chunks(2).into_iter().map(|mut chunk| {
        let start = chunk.next().unwrap();
        let end = chunk.next().unwrap();
        (*start, start+end-1)
    }).collect::<Vec<(u64,u64)>>();

    #[cfg(test)]
    println!("Seed Ranges: {:?}", seed_ranges);

    //These ranges will be too large, but as a naive approach, you could just back feed those seed numbers into part1

    let seeds = seed_ranges.iter().flat_map(|(start,end)| {
        (*start..=*end).collect::<Vec<u64>>()
    }).collect::<Vec<u64>>();

    let cloned_mappings = mappings.clone();
    let new_input = (seeds, cloned_mappings);
    part1(&new_input)

}

#[cfg(test)]
mod tests {

    use super::*;

    fn get_test_input() -> &'static str {
        "seeds: 79 14 55 13

            seed-to-soil map:
            50 98 2
            52 50 48

            soil-to-fertilizer map:
            0 15 37
            37 52 2
            39 0 15

            fertilizer-to-water map:
            49 53 8
            0 11 42
            42 0 7
            57 7 4

            water-to-light map:
            88 18 7
            18 25 70

            light-to-temperature map:
            45 77 23
            81 45 19
            68 64 13

            temperature-to-humidity map:
            0 69 1
            1 0 69

            humidity-to-location map:
            60 56 37
            56 93 4"
    }

    #[test]
    fn day5_part1() {
        assert_eq!(part1(&day5_parse(get_test_input())), 35);
    }

    #[test]
    fn day5_part2() {
        assert_eq!(part2(&day5_parse(get_test_input())), 46);
    }
}
