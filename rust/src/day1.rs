/*
 * Advent of Code 2023 Day 1
 */
use fancy_regex::Regex;

pub struct Day1;

impl Day1 {
    pub fn part1(&self, input: String) {
        let calibration_values: Vec<&str> = input.split("\n").collect();
        let mut sum: i32 = 0;
        for value in calibration_values {
            let mut numbers: Vec<i32> = Vec::new();
            for character in value.chars() {
                if character.is_digit(10) {
                    numbers.push(character.to_digit(10).unwrap() as i32);
                }
            }
            let number: i32 = numbers[0] * 10 + numbers.last().unwrap();
            sum += number;
        }
        println!("Part 1: {}", sum);     
    }

    pub fn part2(&self, input: String) {
        let calibration_values: Vec<&str> = input.split("\n").collect();
        let mut sum: i32 = 0;
        for value in calibration_values {
            let numbers = self.parse_digits(value.to_string());
            let number: i32 = numbers[0] * 10 + numbers.last().unwrap();
            sum += number;
        }
        println!("Part 2: {}", sum);
    }

    pub fn parse_digits(&self, input: String) -> Vec<i32> {
        let re = Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|\d))").unwrap();
        let number_strings: Vec<&str> = re.captures_iter(&input).map(|mat| mat.unwrap().get(1).unwrap().as_str()).collect();
        let mut numbers: Vec<i32> = Vec::new();
        for number_string in number_strings {
            numbers.push(match number_string {
                "one" => 1,
                "two" => 2,
                "three" => 3,
                "four" => 4,
                "five" => 5,
                "six" => 6,
                "seven" => 7,
                "eight" => 8,
                "nine" => 9,
                _ => number_string.parse::<i32>().unwrap(),
            });
        }
        numbers
    }
}