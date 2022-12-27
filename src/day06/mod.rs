use crate::load_file_to_string;

fn contains_duplicates(iter: &[char]) -> bool {
    return (1..iter.len()).any(|i| iter[i..].contains(&iter[i - 1]));
}

/// --- Day 6: Tuning Trouble ---
pub fn solve(input_path: String) {
    let buffer = load_file_to_string(input_path);
    let mut queue: Vec<char> = Vec::new(); 

    for (index, character) in buffer.chars().enumerate() {
        // change the 4 to 14 to solve the second part
        if queue.len() == 4 {
            if !contains_duplicates(&queue) {
                println!("The first marker is detected at index: {} after {:?}.", index, queue);
                return;
            }
            queue.remove(0);
        }

        queue.push(character);
    }

}