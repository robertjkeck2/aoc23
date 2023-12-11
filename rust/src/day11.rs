use std::collections::HashMap;

/*
 * Advent of Code 2023 Day 11
 */
use crate::day::Day;

pub struct Day11;

impl Day for Day11 {
    fn part1(&self, input: String) {
        let mut image = Image::new(input);
        let mut sum: i32 = 0;
        image.expand();
        image.find_galaxies();
        for (i, galaxy) in &image.galaxies {
            sum += image.sum_of_distances(*i, *galaxy);
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let _: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub struct Image {
    pixels: Vec<Vec<char>>,
    galaxies: HashMap<i32, (i32, i32)>,
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
            galaxies: HashMap::new(),
        }
    }

    fn sum_of_distances(&self, index: i32, point: (i32, i32)) -> i32 {
        let mut sum: i32 = 0;
        for (i, &galaxy) in &self.galaxies {
            if (i > &index) && (galaxy != point) {
                sum += self.get_distance(point, galaxy);
            }
        }
        sum
    }

    fn find_galaxies(&mut self) {
        let mut galaxy_id: i32 = 1;
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
        let pixel_clone: Vec<Vec<char>> = self.pixels.clone();
        for i in 0..pixel_clone.len() {
            // Check if all chars are "."
            if pixel_clone[i].iter().all(|&c| c == '.') {
                let new_row = vec!['.'; pixel_clone[i].len()];
                self.pixels.insert(i + 1, new_row);
            }
        }
    }

    fn expand_x(&mut self) {
        let pixel_clone: Vec<Vec<char>> = self.pixels.clone();
        let mut new_column_indexes: Vec<usize> = Vec::new();
        for i in 0..pixel_clone[0].len() {
            let mut all_period: bool = true;
            for j in 0..pixel_clone.len() {
                if pixel_clone[j][i] != '.' {
                    all_period = false;
                    break;
                }
            }
            if all_period {
                new_column_indexes.push(i);
            }
        }

        for i in 0..pixel_clone.len() {
            for j in new_column_indexes.iter().rev() {
                self.pixels[i].insert(*j, '.');
            }
        }
    }

    fn get_distance(&self, a: (i32, i32), b: (i32, i32)) -> i32 {
        // Manhattan distance
        let x: i32 = (b.0 - a.0).abs();
        let y: i32 = (b.1 - a.1).abs();
        x + y
    }
}
