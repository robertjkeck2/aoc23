/*
 * Advent of Code 2023 Day 2
 */
use crate::day::Day;

pub struct Day2;

impl Day for Day2 {
    fn part1(&self, input: String) {
        const MAX_RED_CUBES: i32 = 12;
        const MAX_GREEN_CUBES: i32 = 13;
        const MAX_BLUE_CUBES: i32 = 14;
        let game_values: Vec<&str> = input.split("\n").collect();
        let mut all_games: i32 = 0;
        let mut invalid_games: i32 = 0;
        for game_value in game_values {
            let game = Game::new(game_value.to_string());
            all_games += game.game_num;
            for turn in game.turns {
                if turn.green_cubes > MAX_GREEN_CUBES
                    || turn.blue_cubes > MAX_BLUE_CUBES
                    || turn.red_cubes > MAX_RED_CUBES
                {
                    invalid_games += game.game_num;
                    break;
                }
            }
        }
        println!("Part 1: {}", all_games - invalid_games);
    }

    fn part2(&self, input: String) {
        let game_values: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub struct Game {
    pub game_num: i32,
    pub turns: Vec<Turn>,
}

impl Game {
    pub fn new(input: String) -> Game {
        let game_split: Vec<&str> = input.split(": ").collect();
        let game_num_split: Vec<&str> = game_split[0].split(" ").collect();
        let game_num: i32 = game_num_split[1].parse::<i32>().unwrap();
        let raw_turns: Vec<&str> = game_split[1].split("; ").collect();
        let mut turns: Vec<Turn> = Vec::new();
        for raw_turn in raw_turns {
            turns.push(Turn::new(raw_turn.to_string()));
        }
        Game { game_num, turns }
    }
}

pub struct Turn {
    pub green_cubes: i32,
    pub blue_cubes: i32,
    pub red_cubes: i32,
}

impl Turn {
    pub fn new(input: String) -> Turn {
        let raw_turn_split: Vec<&str> = input.split(", ").collect();
        let mut green: i32 = 0;
        let mut blue: i32 = 0;
        let mut red: i32 = 0;
        for raw_turn in raw_turn_split {
            let turn_split: Vec<&str> = raw_turn.split(" ").collect();
            let turn_value: &str = turn_split[0];
            let turn_type: &str = turn_split[1];
            match turn_type {
                "green" => green = turn_value.parse::<i32>().unwrap(),
                "blue" => blue = turn_value.parse::<i32>().unwrap(),
                "red" => red = turn_value.parse::<i32>().unwrap(),
                _ => println!("Invalid turn type: {}", turn_type),
            }
        }
        Turn {
            green_cubes: green,
            blue_cubes: blue,
            red_cubes: red,
        }
    }
}
