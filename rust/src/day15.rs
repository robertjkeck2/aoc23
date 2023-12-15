/*
 * Advent of Code 2023 Day 15
 */
use crate::day::Day;

pub struct Day15;

impl Day for Day15 {
    fn part1(&self, input: String) {
        let values: Vec<&str> = input.split(",").collect();
        let mut sum: i32 = 0;
        for value in values {
            let hash = Hash::new(value.to_string());
            sum += hash.value;
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let _: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub struct Hash {
    value: i32,
}

impl Hash {
    pub fn new(input: String) -> Hash {
        let mut value: i32 = 0;
        for c in input.chars() {
            let new_val: i32 = value + c as i32;
            value = new_val * 17 % 256;
        }
        Hash { value }
    }
}
