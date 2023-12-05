use itertools::Itertools as _;
use std::{
    cmp::{max, min},
    collections::{HashMap, VecDeque},
    fmt::{self, Display, Formatter},
}; //just import traits

pub type SeedList = Vec<u64>;

pub type InputType = (SeedList, HashMap<(Thing, Thing), Vec<Mapping>>);
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

#[derive(Clone)]
pub struct Mapping {
    //Also referred to as a rule
    length: u64,
    source: u64,
    dest: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    start: u64,
    end: u64,
}

impl Range {
    fn segment_by_rules(&self, rules: &Vec<Mapping>) -> Vec<Range> {
        let mut new_ranges = Vec::new();

        // For each rule, determine if it falls in a range,
        let ordered_rules = get_ordered_rules(rules);
        //determine what rules this range falls into
        #[cfg(test)]
        println!("Ordered Rules: {:?}", ordered_rules);

        let rules = ordered_rules
            .iter()
            .filter(|rule| rule.rule_applies(self))
            .collect::<Vec<&Mapping>>();

        //For each rule that applies, create a new segment

        //Find the first rule we intersect with
        //Since the rules are ordered, check if we are fully contained in the first rule
        //If we are, then we can just apply the rule and return
        //If we aren't, then we need to split the range into multiple, and apply the rule to the first
        #[cfg(test)]
        println!("Valid Rules for {:?}: {:?}", self, rules);

        for rule in rules.iter() {
            #[cfg(test)]
            println!("Considering rule: {:?}", rule);
            if rule.source <= self.start && rule.get_source_end() >= self.end {
                //We are fully contained in this rule
                new_ranges.push(rule.apply_rule(self));
            }

            if rule.source <= self.start && rule.get_source_end() < self.end
                || rule.source > self.start && rule.get_source_end() >= self.end
            {
                //We are partially contained in this rule, so we need to split the range
                new_ranges.push(rule.apply_rule(&Range {
                    start: max(self.start, rule.source),
                    end: min(rule.get_source_end(), self.end),
                }));
                //Then we need to apply the rule to the rest of the range
            }
        }

        #[cfg(test)]
        println!("Old Range: {:?} To Segments {:?}", self, new_ranges);

        new_ranges
    }
}

impl Mapping {
    fn get_rule(&self) -> i64 {
        (self.dest as i64) - (self.source as i64)
    }

    fn rule_applies(&self, range: &Range) -> bool {
        self.source <= range.start && range.start <= self.get_source_end()
            || self.source <= range.end && range.end <= self.get_source_end()
    }

    fn apply_rule(&self, range: &Range) -> Range {
        Range {
            start: ((range.start as i64) + self.get_rule()) as u64,
            end: ((range.end as i64) + self.get_rule()) as u64,
        }
    }

    fn get_source_end(&self) -> u64 {
        ((self.source as i64 + self.length as i64) - 1) as u64
    }
}

impl Display for Mapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "R[{}..{}] ({})", self.source, self.dest, self.get_rule())
    }
}

impl fmt::Debug for Mapping {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "R[{}..{}] ({})",
            self.source,
            self.get_source_end(),
            self.get_rule()
        )
    }
}

/// Also generate the default +0 rules
fn get_ordered_rules(rules: &Vec<Mapping>) -> Vec<Mapping> {
    let mut ordered_rules = rules.clone();
    ordered_rules.sort_by_key(|x| x.source);

    //Get the first, and last rules, and add the default rules
    //NOTE: THIS ASSUMES THAT RULES DON'T OVERLAP AND ARE CONTINGIOUS

    //Sick nasty clone
    let first_rule = ordered_rules.first().unwrap().clone();
    let last_rule = ordered_rules.last().unwrap().clone();

    //If the contigous range thought doesn't hold, we will have to geenerate ranges in between
    //those for the default rules

    if first_rule.source > 0 {
        ordered_rules.insert(
            0,
            Mapping {
                source: 0,
                length: first_rule.source, //Go up to the start of the first rule
                dest: 0,
            },
        );
    }

    if last_rule.get_source_end() < std::u64::MAX {
        ordered_rules.push(Mapping {
            source: last_rule.get_source_end() + 1,
            length: std::u64::MAX - last_rule.get_source_end(), //Go up to the start of the first rule
            dest: last_rule.get_source_end() + 1,
        });
    }

    ordered_rules
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
        (Thing::Humidity, Thing::Location),
    ];

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
        (Thing::Humidity, Thing::Location),
    ];

    seeds
        .iter()
        .map(|seed| {
            let mut cur_value = *seed;
            for current_map_type in &map_order {
                let current_map = mappings.get(&current_map_type).unwrap();
                // Now that we have the current translation map, we need to see if the current seed
                // is in the current map, if it is, then we just translate it and move on to the
                // next map, if it's not, we assume it's the same value
                // Be naive for now, and assume that there isn't a partial range
                let found_mapping = current_map
                    .iter()
                    .find(|x| x.source <= cur_value && x.source + x.length > cur_value);

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
        .min()
        .unwrap()
}

#[aoc(day5, part2)]
pub fn part2(input: &InputType) -> OutputType {
    let (raw_seeds_ranges, mappings) = input;
    //The seeds themselves are actually ranges

    let mut ranges = raw_seeds_ranges
        .iter()
        .chunks(2)
        .into_iter()
        .map(|mut chunk| {
            let start = chunk.next().unwrap();
            let end = chunk.next().unwrap();
            Range {
                start: *start,
                end: start + end - 1,
            }
        })
        .collect::<VecDeque<Range>>();

    let map_order = vec![
        (Thing::Seed, Thing::Soil),
        (Thing::Soil, Thing::Fertilizer),
        (Thing::Fertilizer, Thing::Water),
        (Thing::Water, Thing::Light),
        (Thing::Light, Thing::Temperature),
        (Thing::Temperature, Thing::Humidity),
        (Thing::Humidity, Thing::Location),
    ];

    // Iterate through the seeds and the current level, but for each level, you need to find all the bisections and the rules
    for current_map_type in &map_order {
        #[cfg(test)]
        println!("Current Map: {:?}", current_map_type);
        let current_maps = mappings.get(&current_map_type).unwrap();
        let new_ranges = ranges.iter().map(|range| {
            //Map each range to its new values, this means every range can become one or more ranges on the next level
            //For each range, determine which rules apply to it (which ranges it's within)
            range.segment_by_rules(current_maps)
        });

        #[cfg(test)]
        println!("Orig Range: {:?} -> Ranges: {:?}", ranges, new_ranges);
        ranges = new_ranges.flatten().collect::<VecDeque<Range>>();
    }

    #[cfg(test)]
    println!("Final Ranges: {:?}", ranges);

    ranges.iter().map(|range| range.start).min().unwrap()
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
    fn test_rule_sanity() {
        let rule = Mapping {
            source: 50,
            length: 2,
            dest: 52,
        };

        println!("Rule: {:?}", rule);
        assert_eq!(rule.get_rule(), 2);
        assert_eq!(
            rule.apply_rule(&Range { start: 55, end: 67 }),
            Range { start: 57, end: 69 }
        );
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
