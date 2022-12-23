use crate::read_lines;
use std::collections::HashMap;
use num::pow;

static ALPHABET: [char; 52] = [
    'a', 'b', 'c', 'd', 'e', 
    'f', 'g', 'h', 'i', 'j', 
    'k', 'l', 'm', 'n', 'o',
    'p', 'q', 'r', 's', 't', 
    'u', 'v', 'w', 'x', 'y', 
    'z', 'A', 'B', 'C', 'D', 
    'E', 'F', 'G', 'H', 'I',
    'J', 'K', 'L', 'M', 'N', 
    'O','P', 'Q', 'R', 'S', 
    'T', 'U', 'V', 'W', 'X', 
    'Y', 'Z',
];

fn get_item_priority(item: char) -> u32 {
    return u32::try_from(ALPHABET.iter().position(|&letter | letter == item).unwrap()).unwrap() + 1;
}


/// --- Day 3: Rucksack Reorganization ---
pub fn solve(input_path: String) {
    let mut common_item: char = '_';
    let mut final_sum: u32 = 0;

    for line in read_lines(&input_path) {
        if let Ok(line) = line {
            let compartments: (&str, &str) = line.split_at(line.len() / 2);
            for i in compartments.0.chars() {
                for j in compartments.1.chars() {
                    if i == j && common_item == '_' {
                        common_item = i;
                        final_sum += get_item_priority(common_item);
                    }
                }
            }
            common_item = '_';
        }
    }
    println!("Final sum of item priorities: {final_sum}");
    solve_two(&input_path);
}

/// Create hashmap for each elf group; value holds occurences of items
pub fn solve_two(input_path: &String) {
    let mut elf_group: HashMap<char, u32> = HashMap::new();
    let mut final_priority: u32 = 0;

    for (index, line) in read_lines(input_path).enumerate() {
        if let Ok(line) = line {
            for item in line.chars() {
                elf_group.insert(item, u32::try_from(pow(10, index%3)).unwrap() + elf_group.get(&item).unwrap_or(&0));
            }

            if index%3 == 2 {
                // common item looks something like 111, 311, 323 etc.
                let common_item = elf_group.iter().find(|&(_, value)| value.to_string().len() == 3 && !value.to_string().contains("0")).unwrap();
                let item_priority = get_item_priority(*common_item.0);
                final_priority += item_priority;

                elf_group.clear();
            }
        }
    }
    println!("Final sum of group priorities: {final_priority}");
}