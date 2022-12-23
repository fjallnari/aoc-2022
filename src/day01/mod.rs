use crate::read_lines;

/// --- Day 1: Calorie Counting ---
pub fn solve(input_path: String) {
    println!("Loading input... {}", input_path);

    const TOP_N: usize = 3;

    let mut current_calories: i32 = 0;
    let mut last_calories: i32;
    let mut top_calories: [i32; TOP_N] = [0;TOP_N];

    for line in read_lines(&input_path) {
        if let Ok(line) = line {
            if line.trim() == "" {
                for index in (0..TOP_N).rev() {
                    if current_calories > top_calories[index] {
                        last_calories = top_calories[index];
                        top_calories[index] = current_calories;
                        current_calories = last_calories;
                    }
                }
                current_calories = 0;
                
            } else {
                current_calories += line.parse::<i32>().unwrap();
            }
        }
    }
    let top_calories_sum: i32 = top_calories.iter().sum();
    println!("Top elf is carrying {} calories, top {} are carrying {} calories.", top_calories[top_calories.len()-1], TOP_N, top_calories_sum);
}