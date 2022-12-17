use crate::read_lines;

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


/// --- Day 3: Rucksack Reorganization ---
pub fn solve(input_path: String) {
    let mut common_item: char = '_';
    let mut final_sum: u32 = 0;

    for line in read_lines(input_path) {
        if let Ok(line) = line {
            let compartments: (&str, &str) = line.split_at(line.len() / 2);
            for i in compartments.0.chars() {
                for j in compartments.1.chars() {
                    if i == j && common_item == '_' {
                        common_item = i;
                        let item_priority = ALPHABET.iter().position(|&letter | letter == common_item).unwrap();
                        final_sum += u32::try_from(item_priority).unwrap() + 1;
                    }
                }
            }
            println!("{common_item} {:?}", compartments);
            common_item = '_';
        }
    }
    println!("Final sum of item priorities: {final_sum}");
}