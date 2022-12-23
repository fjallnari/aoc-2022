use std::env;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

mod day01;
mod day02;
mod day03;

struct Config {
    day: String,
}

fn main() {
    let aoc_days = HashMap::from([
        (String::from("01"), day01::solve as fn(String)),
        (String::from("02"), day02::solve as fn(String)),
        (String::from("03"), day03::solve as fn(String))
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