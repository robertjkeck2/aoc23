/*
 * Advent of Code 2023 Day 3
 */
use crate::day::Day;
use fancy_regex::Regex;

pub struct Day3;

impl Day for Day3 {
    fn part1(&self, input: String) {
        let schematic_rows: Vec<&str> = input.split("\n").collect();
        let mut sum: i64 = 0;
        for i in 0..schematic_rows.len() {
            let mut symbol_locations: Vec<usize> = Vec::new();
            for j in 0..schematic_rows[i].len() {
                if schematic_rows[i].chars().nth(j).unwrap().is_digit(10) {
                    let is_symbol: bool = check_for_symbol(
                        j,
                        if i > 0 {
                            schematic_rows[i - 1].to_string()
                        } else {
                            "".to_string()
                        },
                        if i < schematic_rows.len() - 1 {
                            schematic_rows[i + 1].to_string()
                        } else {
                            "".to_string()
                        },
                        if j > 0 {
                            schematic_rows[i].chars().nth(j - 1).unwrap().to_string()
                        } else {
                            "".to_string()
                        },
                        if j < schematic_rows[i].len() - 1 {
                            schematic_rows[i].chars().nth(j + 1).unwrap().to_string()
                        } else {
                            "".to_string()
                        },
                    );
                    if is_symbol {
                        symbol_locations.push(j);
                    }
                }
            }
            let re: Regex = Regex::new(r"(\d+)").unwrap();
            let numbers: Vec<&str> = re
                .captures_iter(&schematic_rows[i])
                .map(
                    |mat: Result<fancy_regex::Captures<'_>, fancy_regex::Error>| {
                        mat.unwrap().get(1).unwrap().as_str()
                    },
                )
                .collect();
            let number_locations: Vec<usize> = numbers
                .iter()
                .map(|number| schematic_rows[i].find(number).unwrap())
                .collect();
            for k in 0..number_locations.len() {
                for l in 0..numbers[k].len() {
                    if symbol_locations.contains(&(number_locations[k] + l)) {
                        sum += numbers[k].parse::<i64>().unwrap();
                        break;
                    }
                }
            }
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let schematic_rows: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub fn check_for_symbol(
    index: usize,
    prev_row: String,
    next_row: String,
    prev_char: String,
    next_char: String,
) -> bool {
    let mut symbols: Vec<String> = Vec::new();
    if prev_char != "" {
        symbols.push(prev_char)
    };
    if next_char != "" {
        symbols.push(next_char)
    };
    if prev_row != "" {
        symbols.push(prev_row.chars().nth(index).unwrap().to_string());
        if index > 0 {
            symbols.push(prev_row.chars().nth(index - 1).unwrap().to_string())
        };
        if index < prev_row.len() - 1 {
            symbols.push(prev_row.chars().nth(index + 1).unwrap().to_string())
        };
    }
    if next_row != "" {
        symbols.push(next_row.chars().nth(index).unwrap().to_string());
        if index > 0 {
            symbols.push(next_row.chars().nth(index - 1).unwrap().to_string());
        }
        if index < next_row.len() - 1 {
            symbols.push(next_row.chars().nth(index + 1).unwrap().to_string());
        }
    }
    let re: Regex = Regex::new(r"[^a-zA-Z0-9.]").unwrap();
    for symbol in symbols {
        if symbol != "" && re.is_match(&symbol).unwrap() {
            return true;
        }
    }
    return false;
}
