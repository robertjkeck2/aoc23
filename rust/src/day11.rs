/*
 * Advent of Code 2023 Day 11
 */
use crate::day::Day;

use std::collections::BTreeMap;

pub struct Day11;

impl Day for Day11 {
    fn part1(&self, input: String) {
        let mut image = Image::new(input);
        let mut sum: i128 = 0;
        image.expand();
        image.find_galaxies();
        for (i, galaxy) in &image.galaxies {
            sum += image.sum_of_distances(*i, *galaxy, 1);
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let mut image = Image::new(input);
        let mut sum: i128 = 0;
        image.expand();
        image.find_galaxies();
        for (i, galaxy) in &image.galaxies {
            sum += image.sum_of_distances(*i, *galaxy, 1000000 - 1);
        }
        println!("Part 2: {}", sum);
    }
}

pub struct Image {
    pixels: Vec<Vec<char>>,
    galaxies: BTreeMap<i32, (i32, i32)>,
    extended_rows: Vec<usize>,
    extended_cols: Vec<usize>,
}

impl Image {
    fn new(input: String) -> Image {
        let mut pixels: Vec<Vec<char>> = Vec::new();
        for line in input.split("\n") {
            let mut row: Vec<char> = Vec::new();
            for c in line.chars() {
                row.push(c);
            }
            pixels.push(row);
        }
        Image {
            pixels,
            galaxies: BTreeMap::new(),
            extended_rows: Vec::new(),
            extended_cols: Vec::new(),
        }
    }

    fn sum_of_distances(&self, index: i32, point: (i32, i32), expansion_factor: i128) -> i128 {
        let mut sum: i128 = 0;
        for (i, &galaxy) in &self.galaxies {
            if i > &index {
                sum += self.get_distance(point, galaxy, expansion_factor);
            }
        }
        sum
    }

    fn find_galaxies(&mut self) {
        let mut galaxy_id: i32 = 0;
        for i in 0..(&mut self.pixels).len() {
            for j in 0..(&mut self.pixels[i]).len() {
                if self.pixels[i][j] == '#' {
                    self.galaxies.insert(galaxy_id, (i as i32, j as i32));
                    galaxy_id += 1;
                }
            }
        }
    }

    fn expand(&mut self) {
        self.expand_y();
        self.expand_x();
    }

    fn expand_y(&mut self) {
        for i in 0..self.pixels.len() {
            // Check if all chars are "."
            if self.pixels[i].iter().all(|&c| c == '.') {
                self.extended_rows.push(i);
            }
        }
    }

    fn expand_x(&mut self) {
        for i in 0..self.pixels[0].len() {
            let mut all_period: bool = true;
            for j in 0..self.pixels.len() {
                if self.pixels[j][i] != '.' {
                    all_period = false;
                    break;
                }
            }
            if all_period {
                self.extended_cols.push(i);
            }
        }
    }

    fn get_distance(&self, a: (i32, i32), b: (i32, i32), expansion_factor: i128) -> i128 {
        // Manhattan distance
        let x: i128 = (b.0 - a.0).abs() as i128;
        let y: i128 = (b.1 - a.1).abs() as i128;
        let mut expansion: i128 = 0;
        for i in 0..self.extended_rows.len() {
            if (b.0..a.0).contains(&(self.extended_rows[i] as i32))
                || (a.0..b.0).contains(&(self.extended_rows[i] as i32))
            {
                expansion += expansion_factor;
            }
        }
        for i in 0..self.extended_cols.len() {
            if (b.1..a.1).contains(&(self.extended_cols[i] as i32))
                || (a.1..b.1).contains(&(self.extended_cols[i] as i32))
            {
                expansion += expansion_factor;
            }
        }
        x + y + expansion
    }
}
