/*
 * Advent of Code 2023 Day 2
 */
use crate::day::Day;

pub struct Day2;

impl Day for Day2 {
    fn part1(&self, input: String) {
        let calibration_values: Vec<&str> = input.split("\n").collect();
        println!("Part 1: {}", "TODO");
    }

    fn part2(&self, input: String) {
        let calibration_values: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub struct Game {
    pub game_num: i32,
    //pub turns: Vec<Turn>,
}

// pub struct Turn {
//     pub green_cubes: i32,
//     pub blue_cubes: i32,
//     pub red_cubes: i32,
// }

// impl Turn {
//     pub fn new(input: String) -> Turn {
//         let raw_turn_split: Vec<&str> = input.split(", ").collect();
//         for raw_turn in raw_turn_split {
//             let turn_split: Vec<&str> = raw_turn.split(" ").collect();
//             let turn_value = turn_split[0];
//             let turn_type = turn_split[1];
//             match turn_type {
//                 "green" => green_cubes = turn_value,
//                 "blue" => blue_cubes = turn_value,
//                 "red" => red_cubes = turn_value,
//                 _ => println!("Invalid turn type: {}", turn_type),
//             }
//         }
//         Turn {
//             green_cubes,
//             blue_cubes,
//             red_cubes,
//         }
//     }
// }
