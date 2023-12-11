mod day;
mod day1;
mod day10;
mod day11;
mod day12;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod input;

use day::Day;
use day1::Day1;
use day10::Day10;
use day11::Day11;
use day12::Day12;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;
use day8::Day8;
use day9::Day9;
use input::InputReader;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: u32 = args[1].parse::<u32>().unwrap();
    let input_num: u32 = args[2].parse::<u32>().unwrap();
    let input: String = InputReader::read_file_input(day_num);
    let day: &dyn Day;
    match day_num {
        1 => day = &Day1,
        2 => day = &Day2,
        3 => day = &Day3,
        4 => day = &Day4,
        5 => day = &Day5,
        6 => day = &Day6,
        7 => day = &Day7,
        8 => day = &Day8,
        9 => day = &Day9,
        10 => day = &Day10,
        11 => day = &Day11,
        12 => day = &Day12,
        _ => panic!("Day {} not implemented yet", day_num),
    }
    match input_num {
        1 => day.part1(input),
        2 => day.part2(input),
        _ => panic!("Input {} not implemented yet", input_num),
    }
}
