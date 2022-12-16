use std::env;
use std::collections::HashMap;

mod day01;


fn main() {
    let aoc_days = HashMap::from([
        (String::from("01"), day01::calorie_count)
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

struct Config {
    day: String,
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