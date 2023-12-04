mod input;
mod day1;

use std::env;
use input::InputReader;
use day1::Day1;

fn main() {
    let args: Vec<String> = env::args().collect();
    let day_num = args[1].parse::<u32>().unwrap();
    let input_num = args[2].parse::<u32>().unwrap();
    let input = InputReader::read_file_input(day_num);
    let day;
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
