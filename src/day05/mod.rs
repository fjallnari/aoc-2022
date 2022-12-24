use regex::Regex;
use crate::read_lines;

/// --- Day 5: Supply Stacks ---
pub fn solve(input_path: String) {
    let mut stacks: Vec<Vec<char>> = Default::default();    
    for (index_line, line) in read_lines(&input_path).enumerate() {
        if let Ok(line) = line {
            if index_line == 0 {
                let stack_count = line.len()/4 + 1;
                stacks.resize_with(stack_count, Default::default);
            }

            if line.trim() != "" && line.trim().starts_with("[") {
                // parse input crates
                for (index, character) in line.chars().enumerate() {
                    if index%4 == 1 && character != ' ' {
                        stacks[index/4].insert(0, character);
                    }
                }
            }
            else if line.starts_with("move") {
                // parse instructions
                let re = Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
                let captures = re.captures(&line).unwrap();
                let params: Vec<&str> = captures.iter().map(|x| x.unwrap().as_str()).collect();
                let params = &params[1..4];

                let crates_count: i32 = params[0].parse::<i32>().unwrap();
                let origin_index:usize = params[1].parse::<usize>().unwrap() - 1;
                let destination_index:usize = params[2].parse::<usize>().unwrap() - 1;

                let mut crates_drain_index:i32 = stacks[origin_index].len() as i32 - crates_count;
                if crates_drain_index < 0 {
                    crates_drain_index = 0;
                }

                let mut origin_crate: Vec<char> = std::mem::take(&mut stacks[origin_index]);

                let crates_to_move = origin_crate.drain(crates_drain_index as usize..);
                
                // if you remove the rev() call, it solves the second part of the day 5 problem
                stacks[destination_index].extend(crates_to_move.rev());
                stacks[origin_index] = origin_crate;
            }
        }
    }

    let stack_tops: Vec<String> = stacks.iter().map(|stack| stack.last().unwrap().to_string()).collect();

    println!("The following crates end up on top: {}", stack_tops.join(""));
}