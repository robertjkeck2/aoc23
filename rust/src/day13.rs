/*
 * Advent of Code 2023 Day 13
 */
use crate::day::Day;

pub struct Day13;

impl Day for Day13 {
    fn part1(&self, input: String) {
        let reflections: Vec<&str> = input.split("\n\n").collect();
        let mut sum: i32 = 0;
        for reflection in reflections {
            let reflection: Reflection = Reflection::new(reflection.to_string());
            sum += reflection.summarize_row_col();
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let _: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub struct Reflection {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
}

impl Reflection {
    fn new(input: String) -> Reflection {
        let rows: Vec<Vec<char>> = input.split("\n").map(|row| row.chars().collect()).collect();
        let cols: Vec<Vec<char>> = (0..rows[0].len())
            .map(|col| rows.iter().map(|row| row[col]).collect())
            .collect();
        Reflection { rows, cols }
    }

    fn summarize_row_col(&self) -> i32 {
        let mut row_col: (i32, i32) = self.find_middle(0, 0);
        let mut valid_reflection: (bool, bool) = self.validate_reflection(row_col);
        while !valid_reflection.0 && !valid_reflection.1 {
            let new_row: i32 = if !valid_reflection.0 { row_col.0 } else { 0 };
            let new_col: i32 = if !valid_reflection.1 { row_col.1 } else { 0 };
            row_col = self.find_middle(new_row as usize, new_col as usize);
            valid_reflection = self.validate_reflection(row_col);
        }
        let row_num: i32 = if valid_reflection.0 {
            row_col.0 * 100
        } else {
            0
        };
        let col_num: i32 = if valid_reflection.1 { row_col.1 } else { 0 };
        return row_num + col_num;
    }

    fn find_middle(&self, start_row: usize, start_col: usize) -> (i32, i32) {
        let mut row_num: i32 = 0;
        let mut col_num: i32 = 0;
        for i in start_row..(self.rows.len() - 1) {
            if self.rows[i] == self.rows[i + 1] {
                row_num = i as i32 + 1;
                break;
            }
        }
        for i in start_col..(self.cols.len() - 1) {
            if self.cols[i] == self.cols[i + 1] {
                col_num = i as i32 + 1;
                break;
            }
        }
        (row_num, col_num)
    }

    fn validate_reflection(&self, middle: (i32, i32)) -> (bool, bool) {
        let mut valid_row: bool = true;
        let mut valid_col: bool = true;
        for i in 0..middle.0 {
            if (middle.0 + i + 1) <= (self.rows.len() - 1) as i32
                && (middle.0 - i - 2) >= 0
                && self.rows[(middle.0 - i - 2) as usize] != self.rows[(middle.0 + i + 1) as usize]
            {
                valid_row = false;
                break;
            }
        }
        for i in 0..middle.1 {
            if (middle.1 + i + 1) <= (self.cols.len() - 1) as i32
                && (middle.1 - i - 2) >= 0
                && self.cols[(middle.1 - i - 2) as usize] != self.cols[(middle.1 + i + 1) as usize]
            {
                valid_col = false;
                break;
            }
        }
        if middle.0 == 0 {
            valid_row = false;
        }
        if middle.1 == 0 {
            valid_col = false;
        }
        (valid_row, valid_col)
    }
}
