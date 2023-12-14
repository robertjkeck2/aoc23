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
            let mut reflection: Reflection = Reflection::new(reflection.to_string());
            let reflection_lines: (i32, i32) = reflection.summarize_row_col(false, (0, 0));
            sum += reflection_lines.0 * 100 + reflection_lines.1;
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let reflections: Vec<&str> = input.split("\n\n").collect();
        let mut sum: i32 = 0;
        for reflection in reflections {
            let mut reflection: Reflection = Reflection::new(reflection.to_string());
            let first_reflections: (i32, i32) = reflection.summarize_row_col(false, (0, 0));
            let second_reflections: (i32, i32) =
                reflection.summarize_row_col(true, first_reflections);
            sum += second_reflections.0 * 100 + second_reflections.1;
        }
        println!("Part 2: {}", sum);
    }
}

pub struct Reflection {
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>,
    updated_rows: Vec<Vec<char>>,
    updated_cols: Vec<Vec<char>>,
    found_row_smudge: bool,
    found_col_smudge: bool,
}

impl Reflection {
    fn new(input: String) -> Reflection {
        let rows: Vec<Vec<char>> = input.split("\n").map(|row| row.chars().collect()).collect();
        let cols: Vec<Vec<char>> = (0..rows[0].len())
            .map(|col| rows.iter().map(|row| row[col]).collect())
            .collect();
        let updated_rows: Vec<Vec<char>> = rows.clone();
        let updated_cols: Vec<Vec<char>> = cols.clone();
        Reflection {
            rows,
            cols,
            updated_rows,
            updated_cols,
            found_row_smudge: false,
            found_col_smudge: false,
        }
    }

    fn summarize_row_col(&mut self, check_smudge: bool, deny_list: (i32, i32)) -> (i32, i32) {
        let mut row_col: (i32, i32) = self.find_middle(0, 0, check_smudge, deny_list);
        let mut valid_reflection: (bool, bool) = self.validate_reflection(row_col, check_smudge);
        while !valid_reflection.0 && !valid_reflection.1 {
            self.found_col_smudge = false;
            self.found_row_smudge = false;
            let new_row: i32 = if !valid_reflection.0 { row_col.0 } else { 0 };
            let new_col: i32 = if !valid_reflection.1 { row_col.1 } else { 0 };
            row_col = self.find_middle(new_row as usize, new_col as usize, check_smudge, deny_list);
            valid_reflection = self.validate_reflection(row_col, check_smudge);
        }
        let row_num: i32 = if valid_reflection.0 { row_col.0 } else { 0 };
        let col_num: i32 = if valid_reflection.1 { row_col.1 } else { 0 };
        return (row_num, col_num);
    }

    fn find_middle(
        &mut self,
        start_row: usize,
        start_col: usize,
        check_smudge: bool,
        deny_list: (i32, i32),
    ) -> (i32, i32) {
        let mut row_num: i32 = 0;
        let mut col_num: i32 = 0;
        for i in start_row..(self.rows.len() - 1) {
            if check_smudge
                && !self.found_row_smudge
                && self.has_smudge(self.rows[i].clone(), self.rows[i + 1].clone(), i, true)
                && (i as i32 + 1) != deny_list.0
            {
                self.found_row_smudge = true;
                row_num = i as i32 + 1;
                break;
            } else if self.rows[i] == self.rows[i + 1] && (i as i32 + 1) != deny_list.0 {
                row_num = i as i32 + 1;
                break;
            }
        }
        for i in start_col..(self.cols.len() - 1) {
            if check_smudge
                && !self.found_col_smudge
                && self.has_smudge(self.cols[i].clone(), self.cols[i + 1].clone(), i, false)
                && (i as i32 + 1) != deny_list.1
            {
                self.found_col_smudge = true;
                col_num = i as i32 + 1;
                break;
            } else if self.cols[i] == self.cols[i + 1] && (i as i32 + 1) != deny_list.1 {
                col_num = i as i32 + 1;
                break;
            }
        }
        (row_num, col_num)
    }

    fn validate_reflection(&mut self, middle: (i32, i32), check_smudge: bool) -> (bool, bool) {
        let mut valid_row: bool = true;
        let mut valid_col: bool = true;
        if middle.0 == 0 {
            valid_row = false;
        }
        if middle.1 == 0 {
            valid_col = false;
        }
        for i in 0..middle.0 {
            if (middle.0 + i + 1) <= (self.rows.len() - 1) as i32
                && (middle.0 - i - 2) >= 0
                && self.rows[(middle.0 - i - 2) as usize] != self.rows[(middle.0 + i + 1) as usize]
            {
                if !check_smudge
                    || self.found_row_smudge
                    || !self.has_smudge(
                        self.rows[(middle.0 - i - 2) as usize].clone(),
                        self.rows[(middle.0 + i + 1) as usize].clone(),
                        (middle.0 - i - 2) as usize,
                        true,
                    )
                {
                    valid_row = false;
                    break;
                } else {
                    self.found_row_smudge = true;
                }
            }
        }
        for i in 0..middle.1 {
            if (middle.1 + i + 1) <= (self.cols.len() - 1) as i32
                && (middle.1 - i - 2) >= 0
                && self.cols[(middle.1 - i - 2) as usize] != self.cols[(middle.1 + i + 1) as usize]
            {
                if !check_smudge
                    || self.found_col_smudge
                    || !self.has_smudge(
                        self.cols[(middle.1 - i - 2) as usize].clone(),
                        self.cols[(middle.1 + i + 1) as usize].clone(),
                        (middle.1 - i - 2) as usize,
                        false,
                    )
                {
                    valid_col = false;
                    break;
                } else {
                    self.found_col_smudge = true;
                }
            }
        }
        (valid_row, valid_col)
    }

    fn has_smudge(
        &mut self,
        row_one: Vec<char>,
        row_two: Vec<char>,
        row_one_num: usize,
        is_row: bool,
    ) -> bool {
        let mut num_diffs: i32 = 0;
        for i in 0..row_one.len() {
            if row_one[i] != row_two[i] {
                num_diffs += 1;
            }
        }
        if num_diffs == 1 {
            let found_smudge: bool;
            if is_row {
                found_smudge = self.found_row_smudge;
            } else {
                found_smudge = self.found_col_smudge;
            }
            if !found_smudge {
                self.update_reflection(row_two, row_one_num, is_row)
            };
            true
        } else {
            false
        }
    }

    fn update_reflection(&mut self, row: Vec<char>, row_num: usize, is_row: bool) {
        if is_row {
            self.updated_rows[row_num] = row;
        } else {
            self.updated_cols[row_num] = row;
        }
    }
}
