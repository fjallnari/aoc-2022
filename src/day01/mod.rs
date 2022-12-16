use std::fs::File;
use std::io::{self, BufRead};

pub fn calorie_count(input_path: String) {
    println!("Loading input... {}", input_path);

    let lines = read_lines(input_path);

    let mut max_calories: i32 = 0;
    let mut current_calories: i32 = 0;

    for line in lines {
        if let Ok(line) = line {
            if line.trim() == "" {
                if current_calories >= max_calories {
                    max_calories = current_calories;
                }
                current_calories = 0;
            } else {
                current_calories += line.parse::<i32>().unwrap();
            }
        }
    }
    println!("{max_calories}");
}

fn read_lines(input_path: String) -> io::Lines<io::BufReader<File>> {
    let file = File::open(input_path).expect("Could not load file.");
    return io::BufReader::new(file).lines();
}