/*
 * Advent of Code 2023 Day 15
 */
use crate::day::Day;

use std::collections::HashMap;

pub struct Day15;

impl Day for Day15 {
    fn part1(&self, input: String) {
        let values: Vec<&str> = input.split(",").collect();
        let mut sum: i32 = 0;
        for value in values {
            let hash = Hash::new(value.to_string());
            sum += hash.value;
        }
        println!("Part 1: {}", sum);
    }

    fn part2(&self, input: String) {
        let values: Vec<&str> = input.split(",").collect();
        let mut boxes: HashMap<i32, Vec<(String, i32)>> = HashMap::new();
        for value in values {
            let instruction: Instruction = Instruction::new(value.to_string());
            let box_num: i32 = instruction.hash.value;
            if instruction.op == "=" {
                let mut replace: bool = false;
                let new_box: (String, i32) = (instruction.name.clone(), instruction.value.clone());
                let mut existing_boxes: Vec<(String, i32)> = Vec::new();
                if boxes.contains_key(&box_num) {
                    existing_boxes = boxes.get(&box_num).unwrap().to_vec();
                    for i in 0..existing_boxes.len() {
                        if existing_boxes[i].0 == new_box.0 {
                            replace = true;
                            existing_boxes[i] = new_box.clone();
                            break;
                        }
                    }
                }

                if !replace {
                    existing_boxes.push(new_box);
                }
                boxes.insert(box_num, existing_boxes);
            } else {
                if boxes.contains_key(&box_num) {
                    let existing_boxes: Vec<(String, i32)> =
                        boxes.get(&box_num).unwrap().to_vec().clone();
                    for i in 0..existing_boxes.len() {
                        if existing_boxes[i].0 == instruction.name {
                            let mut new_boxes: Vec<(String, i32)> = Vec::new();
                            for j in 0..existing_boxes.len() {
                                if j != i {
                                    new_boxes.push(existing_boxes[j].clone());
                                }
                            }
                            boxes.insert(box_num, new_boxes);
                        }
                    }
                }
            }
        }

        let mut sum: i32 = 0;
        for (i, values) in boxes.iter() {
            for (j, value) in values.iter().enumerate() {
                sum += (i + 1) as i32 * (j + 1) as i32 * (value.1);
            }
        }
        println!("Part 2: {}", sum);
    }
}

pub struct Hash {
    value: i32,
}

impl Hash {
    pub fn new(input: String) -> Hash {
        let mut value: i32 = 0;
        for c in input.chars() {
            let new_val: i32 = value + c as i32;
            value = new_val * 17 % 256;
        }
        Hash { value }
    }
}

pub struct Instruction {
    op: String,
    name: String,
    value: i32,
    hash: Hash,
}

impl Instruction {
    pub fn new(input: String) -> Instruction {
        let op: String;
        if input.contains("-") {
            op = "-".to_string();
        } else {
            op = "=".to_string();
        }
        let mut value: i32 = 0;
        let parts: Vec<&str> = input.split(&op).collect();
        let name: String = parts[0].to_string();
        if parts.len() > 1 && !parts[1].is_empty() {
            value = parts[1].parse::<i32>().unwrap();
        }
        Instruction {
            op,
            name: name.clone(),
            value,
            hash: Hash::new(name),
        }
    }
}
