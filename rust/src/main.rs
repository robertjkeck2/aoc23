mod day;
mod day1;
mod day2;
mod day3;
mod day4;
mod input;

use day::Day;
use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
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
        _ => panic!("Day {} not implemented yet", day_num),
    }
    match input_num {
        1 => day.part1(input),
        2 => day.part2(input),
        _ => panic!("Input {} not implemented yet", input_num),
    }
}
