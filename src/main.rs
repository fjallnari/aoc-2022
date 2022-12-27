use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::io::prelude::*;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

struct Config {
    day: String,
}

fn main() {
    let aoc_days = HashMap::from([
        (String::from("01"), day01::solve as fn(String)),
        (String::from("02"), day02::solve as fn(String)),
        (String::from("03"), day03::solve as fn(String)),
        (String::from("04"), day04::solve as fn(String)),
        (String::from("05"), day05::solve as fn(String)),
        (String::from("06"), day06::solve as fn(String)),
    ]);

    let args: Vec<String> = env::args().collect();

    let config: Config = parse_config(&args);

    println!("Running AoC2022 day {}", config.day);

    let input_path: String = format!("src/_input/{}.txt", config.day);

    let day_func = aoc_days.get(&config.day);

    match day_func {
        Some(func) => func(input_path),
        None => {
            println!("invalid op");
        }
    };

}

fn parse_config(args: &[String]) -> Config {
    let day: String;

    if args.len() == 1 {
        day = String::from("01");
    } else {
        day = args[1].clone();
    }

    Config { day }
}

pub fn read_lines(input_path: &String) -> io::Lines<io::BufReader<File>> {
    let file = File::open(input_path).expect("Could not load file.");
    return io::BufReader::new(file).lines();
}

pub fn load_file_to_string(input_path: String) -> String {
    let mut file = match File::open(input_path) {
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };

    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_) => {},
        Err(error) => panic!("Couldn't load file to string: {:?}", error),
    };
    
    return buffer;
}