/*
 * Advent of Code 2023 Day 3
 */
use crate::day::Day;
use fancy_regex::Regex;
use std::collections::HashMap;

pub struct Day3;

impl Day for Day3 {
    fn part1(&self, input: String) {
        let schematic_rows: Vec<&str> = input.split("\n").collect();
        let mut sum: i64 = 0;
        for row in 0..schematic_rows.len() {
            let mut symbol_cells: Vec<usize> = Vec::new();

            // Find all cells that are digits and have a symbol
            for col in 0..schematic_rows[row].len() {
                let cell = Cell::new(row as i64, col as i64, &schematic_rows);
                if cell.is_digit() && cell.has_symbol() {
                    symbol_cells.push(col);
                }
            }

            // Find the index of the first symbol cell for a given number
            let mut trimmed_symbol_cells: Vec<usize> = Vec::new();
            if symbol_cells.len() > 0 {
                let mut last: usize = 0;
                for symbol_cell in symbol_cells {
                    if symbol_cell - 1 != last {
                        trimmed_symbol_cells.push(symbol_cell);
                    }
                    last = symbol_cell;
                }
            }

            // Find the part numbers for the given row
            let (numbers, number_start_location) = get_numbers_and_location(row, &schematic_rows);

            // Create a map of the number location to the number
            let mut number_map: HashMap<usize, &str> = HashMap::new();
            for i in 0..numbers.len() {
                for j in 0..numbers[i].len() {
                    number_map.insert(number_start_location[i] + j, numbers[i]);
                }
            }

            // Map the symbol cells to the numbers
            for symbol_cell in trimmed_symbol_cells {
                let number = number_map.get(&symbol_cell);
                if number.is_none() {
                    continue;
                }
                let found_number = number.unwrap();
                let parsed_number = found_number.parse::<i64>().unwrap();
                sum += parsed_number;
            }
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let schematic_rows: Vec<&str> = input.split("\n").collect();
        let mut sum: i64 = 0;
        for row in 0..schematic_rows.len() {
            let mut gear_cells: Vec<usize> = Vec::new();
            let mut digit_locations: Vec<Vec<Vec<usize>>> = Vec::new();

            // Find all cells that are gears
            for col in 0..schematic_rows[row].len() {
                let cell = Cell::new(row as i64, col as i64, &schematic_rows);
                if cell.is_gear() {
                    gear_cells.push(col);
                    digit_locations.push(cell.digit_locations());
                }
            }

            // Find the part numbers for the first row
            let mut first_row_numbers: Vec<&str> = Vec::new();
            let mut first_row_number_start_location: Vec<usize> = Vec::new();
            if row != 0 {
                let (numbers, number_start_location) =
                    get_numbers_and_location(row - 1, &schematic_rows);
                first_row_numbers = numbers;
                first_row_number_start_location = number_start_location;
            }

            let (middle_row_numbers, middle_row_number_start_location) =
                get_numbers_and_location(row, &schematic_rows);

            let mut last_row_numbers: Vec<&str> = Vec::new();
            let mut last_row_number_start_location: Vec<usize> = Vec::new();
            if row != schematic_rows.len() - 1 {
                let (numbers, number_start_location) =
                    get_numbers_and_location(row + 1, &schematic_rows);
                last_row_numbers = numbers;
                last_row_number_start_location = number_start_location;
            }

            // Create a map of the number locations to the number for the first row
            let mut first_row_number_map: HashMap<usize, &str> = HashMap::new();
            for i in 0..first_row_numbers.len() {
                for j in 0..first_row_numbers[i].len() {
                    first_row_number_map
                        .insert(first_row_number_start_location[i] + j, first_row_numbers[i]);
                }
            }

            // Create a map of the number locations to the number for the middle row
            let mut middle_row_number_map: HashMap<usize, &str> = HashMap::new();
            for i in 0..middle_row_numbers.len() {
                for j in 0..middle_row_numbers[i].len() {
                    middle_row_number_map.insert(
                        middle_row_number_start_location[i] + j,
                        middle_row_numbers[i],
                    );
                }
            }

            // Create a map of the number locations to the number for the last row
            let mut last_row_number_map: HashMap<usize, &str> = HashMap::new();
            for i in 0..last_row_numbers.len() {
                for j in 0..last_row_numbers[i].len() {
                    last_row_number_map
                        .insert(last_row_number_start_location[i] + j, last_row_numbers[i]);
                }
            }

            // Map the gear cells to the numbers
            for locations in digit_locations {
                let mut gear_ratio: i64 = 1;
                for gear_row in 0..locations.len() {
                    let mut number: i64;
                    if locations[gear_row].len() == 0 {
                        continue;
                    }
                    match gear_row {
                        0 => {
                            let value = first_row_number_map.get(&locations[gear_row][0]);
                            number = value.unwrap().parse::<i64>().unwrap();
                            if locations[gear_row].len() == 2 {
                                let second_value =
                                    first_row_number_map.get(&locations[gear_row][1]);
                                let second_number = second_value.unwrap().parse::<i64>().unwrap();
                                number *= second_number;
                            }
                        }
                        1 => {
                            let value = middle_row_number_map.get(&locations[gear_row][0]);
                            number = value.unwrap().parse::<i64>().unwrap();
                            if locations[gear_row].len() == 2 {
                                let second_value =
                                    middle_row_number_map.get(&locations[gear_row][1]);
                                let second_number = second_value.unwrap().parse::<i64>().unwrap();
                                number *= second_number;
                            }
                        }
                        2 => {
                            let value = last_row_number_map.get(&locations[gear_row][0]);
                            number = value.unwrap().parse::<i64>().unwrap();
                            if locations[gear_row].len() == 2 {
                                let second_value = last_row_number_map.get(&locations[gear_row][1]);
                                let second_number = second_value.unwrap().parse::<i64>().unwrap();
                                number *= second_number;
                            }
                        }
                        _ => panic!("Invalid gear row"),
                    }
                    gear_ratio *= number;
                }
                sum += gear_ratio;
            }
        }
        println!("Part 2: {}", sum);
    }
}

pub fn get_numbers_and_location<'a>(
    row: usize,
    schematic_rows: &'a Vec<&'a str>,
) -> (Vec<&'a str>, Vec<usize>) {
    let re: Regex = Regex::new(r"[0-9]+").unwrap();
    let numbers: Vec<&str> = re
        .captures_iter(&schematic_rows[row])
        .map(
            |mat: Result<fancy_regex::Captures<'_>, fancy_regex::Error>| {
                mat.unwrap().get(0).unwrap().as_str()
            },
        )
        .collect();
    let number_start_location: Vec<usize> = re
        .captures_iter(&schematic_rows[row])
        .map(
            |mat: Result<fancy_regex::Captures<'_>, fancy_regex::Error>| {
                mat.unwrap().get(0).unwrap().start()
            },
        )
        .collect();
    return (numbers, number_start_location);
}

pub struct Cell {
    pub row: i64,
    pub col: i64,
    pub tl: String,
    pub tm: String,
    pub tr: String,
    pub ml: String,
    pub mm: String,
    pub mr: String,
    pub bl: String,
    pub bm: String,
    pub br: String,
}

impl Cell {
    pub fn new(row: i64, col: i64, schematic_rows: &Vec<&str>) -> Cell {
        let mut tl: String = ".".to_string();
        let mut tm: String = ".".to_string();
        let mut tr: String = ".".to_string();
        let mut ml: String = ".".to_string();
        let mut mr: String = ".".to_string();
        let mut bl: String = ".".to_string();
        let mut bm: String = ".".to_string();
        let mut br: String = ".".to_string();

        let mm = schematic_rows[row as usize]
            .chars()
            .nth(col as usize)
            .unwrap()
            .to_string();

        if row != 0 && col != 0 {
            tl = schematic_rows[row as usize - 1]
                .chars()
                .nth(col as usize - 1)
                .unwrap()
                .to_string();
        }

        if row != 0 {
            tm = schematic_rows[row as usize - 1]
                .chars()
                .nth(col as usize)
                .unwrap()
                .to_string();
        }

        if row != 0 && col != schematic_rows[row as usize].len() as i64 - 1 {
            tr = schematic_rows[row as usize - 1]
                .chars()
                .nth(col as usize + 1)
                .unwrap()
                .to_string();
        }

        if col != 0 {
            ml = schematic_rows[row as usize]
                .chars()
                .nth(col as usize - 1)
                .unwrap()
                .to_string();
        }

        if col != schematic_rows[row as usize].len() as i64 - 1 {
            mr = schematic_rows[row as usize]
                .chars()
                .nth(col as usize + 1)
                .unwrap()
                .to_string();
        }

        if row != schematic_rows.len() as i64 - 1 && col != 0 {
            bl = schematic_rows[row as usize + 1]
                .chars()
                .nth(col as usize - 1)
                .unwrap()
                .to_string();
        }

        if row != schematic_rows.len() as i64 - 1 {
            bm = schematic_rows[row as usize + 1]
                .chars()
                .nth(col as usize)
                .unwrap()
                .to_string();
        }

        if row != schematic_rows.len() as i64 - 1
            && col != schematic_rows[row as usize].len() as i64 - 1
        {
            br = schematic_rows[row as usize + 1]
                .chars()
                .nth(col as usize + 1)
                .unwrap()
                .to_string();
        }

        Cell {
            row,
            col,
            tl,
            tm,
            tr,
            ml,
            mm,
            mr,
            bl,
            bm,
            br,
        }
    }

    pub fn digit_locations(&self) -> Vec<Vec<usize>> {
        let mut first_row: Vec<usize> = Vec::new();
        let mut second_row: Vec<usize> = Vec::new();
        let mut third_row: Vec<usize> = Vec::new();
        if self.tl.parse::<i64>().is_ok() {
            first_row.push(self.col as usize - 1);
        }
        if self.tm.parse::<i64>().is_ok() {
            first_row.push(self.col as usize);
        }
        if self.tr.parse::<i64>().is_ok() {
            first_row.push(self.col as usize + 1);
        }
        if self.ml.parse::<i64>().is_ok() {
            second_row.push(self.col as usize - 1);
        }
        if self.mr.parse::<i64>().is_ok() {
            second_row.push(self.col as usize + 1);
        }
        if self.bl.parse::<i64>().is_ok() {
            third_row.push(self.col as usize - 1);
        }
        if self.bm.parse::<i64>().is_ok() {
            third_row.push(self.col as usize);
        }
        if self.br.parse::<i64>().is_ok() {
            third_row.push(self.col as usize + 1);
        }

        // Find the index of the first row digits
        let mut trimmed_first_row: Vec<usize> = Vec::new();
        if first_row.len() > 0 {
            let mut last: usize = 0;
            for digit in first_row {
                if digit - 1 != last {
                    trimmed_first_row.push(digit);
                }
                last = digit;
            }
        }

        // Find the index of the second row digits
        let mut trimmed_second_row: Vec<usize> = Vec::new();
        if second_row.len() > 0 {
            let mut last: usize = 0;
            for digit in second_row {
                if digit - 1 != last {
                    trimmed_second_row.push(digit);
                }
                last = digit;
            }
        }

        // Find the index of the third row digits
        let mut trimmed_third_row: Vec<usize> = Vec::new();
        if third_row.len() > 0 {
            let mut last: usize = 0;
            for digit in third_row {
                if digit - 1 != last {
                    trimmed_third_row.push(digit);
                }
                last = digit;
            }
        }

        return vec![trimmed_first_row, trimmed_second_row, trimmed_third_row];
    }

    pub fn is_gear(&self) -> bool {
        if self.mm == "*" {
            let mut digit_count: i64 = 0;
            if self.ml.parse::<i64>().is_ok() {
                digit_count += 1;
            }
            if self.mr.parse::<i64>().is_ok() {
                digit_count += 1;
            }

            if self.tm == "." {
                if self.tl.parse::<i64>().is_ok() {
                    digit_count += 1;
                }
                if self.tr.parse::<i64>().is_ok() {
                    digit_count += 1;
                }
            } else if self.tl.parse::<i64>().is_ok()
                || self.tr.parse::<i64>().is_ok()
                || self.tm.parse::<i64>().is_ok()
            {
                digit_count += 1;
            }

            if self.bm == "." {
                if self.bl.parse::<i64>().is_ok() {
                    digit_count += 1;
                }
                if self.br.parse::<i64>().is_ok() {
                    digit_count += 1;
                }
            } else if self.bl.parse::<i64>().is_ok()
                || self.br.parse::<i64>().is_ok()
                || self.bm.parse::<i64>().is_ok()
            {
                digit_count += 1;
            }

            if digit_count == 2 {
                return true;
            }
        };

        return false;
    }

    pub fn is_digit(&self) -> bool {
        return self.mm.parse::<i64>().is_ok();
    }

    pub fn has_symbol(&self) -> bool {
        let re: Regex = Regex::new(r"[^a-zA-Z0-9. ]").unwrap();
        if re.is_match(&self.tl).unwrap() {
            return true;
        }
        if re.is_match(&self.tm).unwrap() {
            return true;
        }
        if re.is_match(&self.tr).unwrap() {
            return true;
        }
        if re.is_match(&self.ml).unwrap() {
            return true;
        }
        if re.is_match(&self.mr).unwrap() {
            return true;
        }
        if re.is_match(&self.bl).unwrap() {
            return true;
        }
        if re.is_match(&self.bm).unwrap() {
            return true;
        }
        if re.is_match(&self.br).unwrap() {
            return true;
        }
        return false;
    }

    pub fn _println(&self) {
        println!(
            "Cell: row: {}; col: {} 
            {} {} {}
            {} {} {}
            {} {} {}",
            self.row,
            self.col,
            self.tl,
            self.tm,
            self.tr,
            self.ml,
            self.mm,
            self.mr,
            self.bl,
            self.bm,
            self.br
        );
    }
}
