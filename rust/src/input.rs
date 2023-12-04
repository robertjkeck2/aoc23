use std::fs;

pub struct InputReader;

impl InputReader {
    pub fn read_file_input(day_num: u32) -> String {
        let filepath: String = format!("src/data/day{}/input.txt", day_num);
        println!("Reading input from {}", filepath);
        let contents: String =
            fs::read_to_string(filepath).expect("Something went wrong reading the file");
        contents
    }
}
