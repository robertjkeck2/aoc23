/*
 * Advent of Code 2023 Day 9
 */
use crate::day::Day;

pub struct Day9;

impl Day for Day9 {
    fn part1(&self, input: String) {
        let readings: Vec<&str> = input.split("\n").collect();
        let mut sum: i32 = 0;
        for reading in readings {
            let mut row = PatternRow::new(reading.to_string());
            sum += row.get_new_pattern_digit();
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let _: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub struct PatternRow {
    pattern: Vec<i32>,
    last_digits: Vec<i32>,
}

impl PatternRow {
    pub fn new(raw_line: String) -> PatternRow {
        let pattern: Vec<i32> = raw_line
            .split(" ")
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        PatternRow {
            pattern,
            last_digits: Vec::new(),
        }
    }

    pub fn get_new_pattern_digit(&mut self) -> i32 {
        let mut new_pattern_digit: i32 = 0;
        let mut all_zeroes = false;
        let mut new_pattern = self.pattern.clone();
        self.add_last_digit(new_pattern[new_pattern.len() - 1]);
        while !all_zeroes {
            let next_array = self.get_next_array(new_pattern.clone());
            self.add_last_digit(next_array[next_array.len() - 1]);
            all_zeroes = self.check_all_zeroes(next_array.clone());
            new_pattern = next_array;
        }
        for i in self.last_digits.clone() {
            new_pattern_digit += i;
        }
        new_pattern_digit
    }

    pub fn add_last_digit(&mut self, digit: i32) {
        self.last_digits.push(digit);
    }

    pub fn get_next_array(&self, input_array: Vec<i32>) -> Vec<i32> {
        let mut next_array: Vec<i32> = Vec::new();
        for i in 0..(input_array.len() - 1) {
            next_array.push(input_array[i + 1] - input_array[i]);
        }
        next_array
    }

    pub fn check_all_zeroes(&self, input_array: Vec<i32>) -> bool {
        for i in input_array {
            if i != 0 {
                return false;
            }
        }
        true
    }
}
