mod day1;
mod input;

use day1::Day1;
use input::InputReader;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num: u32 = args[1].parse::<u32>().unwrap();
    let input_num: u32 = args[2].parse::<u32>().unwrap();
    let input: String = InputReader::read_file_input(day_num);
    let day: Day1;
    match day_num {
        1 => day = Day1,
        _ => panic!("Day {} not implemented yet", day_num),
    }
    match input_num {
        1 => day.part1(input),
        2 => day.part2(input),
        _ => panic!("Input {} not implemented yet", input_num),
    }
}
