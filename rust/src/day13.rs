/*
 * Advent of Code 2023 Day 13
 */
use crate::day::Day;

pub struct Day13;

impl Day for Day13 {
    fn part1(&self, input: String) {
        let reflections: Vec<&str> = input.split("\n\n").collect();
        for reflection in reflections {
            let reflection: Reflection = Reflection::new(reflection.to_string());
            println!("{:?}", reflection.rows[0]);
        }
        println!("Part 1: {}", "TODO");
    }

    fn part2(&self, input: String) {
        let _: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub struct Reflection {
    rows: Vec<Vec<char>>,
}

impl Reflection {
    fn new(input: String) -> Reflection {
        let rows: Vec<Vec<char>> = input.split("\n").map(|row| row.chars().collect()).collect();
        Reflection { rows }
    }
}
