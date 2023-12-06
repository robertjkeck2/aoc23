/*
 * Advent of Code 2023 Day 5
 */
use crate::day::Day;
use std::collections::HashMap;

pub struct Day5;

impl Day for Day5 {
    fn part1(&self, input: String) {
        let almanac_components: Vec<&str> = input.split("\n\n").collect();
        let mut locations: Vec<i64> = Vec::new();
        let (
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ) = parse_input(almanac_components);

        for seed in &seeds {
            let soil: i64 = seed_to_soil.convert(seed.parse::<i64>().unwrap());
            let fertilizer: i64 = soil_to_fertilizer.convert(soil);
            let water: i64 = fertilizer_to_water.convert(fertilizer);
            let light: i64 = water_to_light.convert(water);
            let temperature: i64 = light_to_temperature.convert(light);
            let humidity: i64 = temperature_to_humidity.convert(temperature);
            let location: i64 = humidity_to_location.convert(humidity);
            locations.push(location);
        }

        println!("Part 1: {}", locations.iter().min().unwrap());
    }

    fn part2(&self, input: String) {
        let _: Vec<&str> = input.split("\n").collect();
        println!("Part 2: {}", "TODO");
    }
}

pub fn parse_input(
    almanac_components: Vec<&str>,
) -> (
    Vec<&str>,
    PlantMap,
    PlantMap,
    PlantMap,
    PlantMap,
    PlantMap,
    PlantMap,
    PlantMap,
) {
    let seed_split: Vec<&str> = almanac_components[0].split(": ").collect();
    let seeds: Vec<&str> = seed_split[1].split(" ").collect();
    let seed_to_soil_split: Vec<&str> = almanac_components[1].split(":\n").collect();
    let seed_to_soil: PlantMap = PlantMap::new(seed_to_soil_split[1].to_string());
    let soil_to_fertilizer_split: Vec<&str> = almanac_components[2].split(":\n").collect();
    let soil_to_fertilizer: PlantMap = PlantMap::new(soil_to_fertilizer_split[1].to_string());
    let fertilizer_to_water_split: Vec<&str> = almanac_components[3].split(":\n").collect();
    let fertilizer_to_water: PlantMap = PlantMap::new(fertilizer_to_water_split[1].to_string());
    let water_to_light_split: Vec<&str> = almanac_components[4].split(":\n").collect();
    let water_to_light: PlantMap = PlantMap::new(water_to_light_split[1].to_string());
    let light_to_temperature_split: Vec<&str> = almanac_components[5].split(":\n").collect();
    let light_to_temperature: PlantMap = PlantMap::new(light_to_temperature_split[1].to_string());
    let temperature_to_humidity_split: Vec<&str> = almanac_components[6].split(":\n").collect();
    let temperature_to_humidity: PlantMap =
        PlantMap::new(temperature_to_humidity_split[1].to_string());
    let humidity_to_location_split: Vec<&str> = almanac_components[7].split(":\n").collect();
    let humidity_to_location: PlantMap = PlantMap::new(humidity_to_location_split[1].to_string());
    return (
        seeds,
        seed_to_soil,
        soil_to_fertilizer,
        fertilizer_to_water,
        water_to_light,
        light_to_temperature,
        temperature_to_humidity,
        humidity_to_location,
    );
}

pub struct PlantMap {
    pub map: Vec<HashMap<String, i64>>,
}

impl PlantMap {
    pub fn new(input: String) -> PlantMap {
        let mut map: Vec<HashMap<String, i64>> = Vec::new();
        for line in input.split("\n") {
            let mut split: std::str::Split<'_, &str> = line.split(" ");
            let dest_range: i64 = split.next().unwrap().parse::<i64>().unwrap();
            let source_range: i64 = split.next().unwrap().parse::<i64>().unwrap();
            let range_len: i64 = split.next().unwrap().parse::<i64>().unwrap();
            let mut map_entry: HashMap<String, i64> = HashMap::new();
            map_entry.insert("min".to_string(), source_range);
            map_entry.insert("max".to_string(), source_range + range_len);
            map_entry.insert("dest".to_string(), dest_range);
            map.push(map_entry);
        }
        PlantMap { map }
    }

    pub fn convert(&self, input: i64) -> i64 {
        for map_entry in &self.map {
            let min: i64 = map_entry.get("min").unwrap().clone();
            let max: i64 = map_entry.get("max").unwrap().clone();
            let dest: i64 = map_entry.get("dest").unwrap().clone();
            if input >= min && input < max {
                return dest + (input - min);
            }
        }
        return input;
    }
}
