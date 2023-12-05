/*
 * Advent of Code 2023 Day 5
 */
use crate::day::Day;
use std::collections::HashMap;

pub struct Day5;

impl Day for Day5 {
    fn part1(&self, input: String) {
        let almanac_components: Vec<&str> = input.split("\n\n").collect();
        println!("Almanac Components: {:?}", almanac_components);
        println!("Part 1: {}", "TODO");
    }

    fn part2(&self, input: String) {
        let _: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub struct PlantMap {
    pub convert: HashMap<i64, i64>,
}

impl PlantMap {
    pub fn new(self, input: String) -> PlantMap {
        let mut convert: HashMap<i64, i64> = HashMap::new();
        for line in input.split("\n") {
            let mut split: std::str::Split<'_, &str> = line.split(" ");
            let dest_range: i64 = split.next().unwrap().parse::<i64>().unwrap();
            let source_range: i64 = split.next().unwrap().parse::<i64>().unwrap();
            let range_len: i64 = split.next().unwrap().parse::<i64>().unwrap();

            for i in 0..range_len {
                convert.insert(source_range + i, dest_range + i);
            }
        }
        PlantMap { convert }
    }

    pub fn get(&self, key: i64) -> i64 {
        match self.convert.get(&key) {
            Some(value) => *value,
            None => 0,
        }
    }
}
