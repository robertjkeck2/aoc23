use std::fs;

pub struct InputReader;

impl InputReader {
    pub fn read_file_input(day_num: u32) -> String {
        let filename = format!("src/data/day{}/input.txt", day_num);
        println!("Reading input from {}", filename);
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        contents
    }
}