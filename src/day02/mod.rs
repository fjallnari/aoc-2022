use crate::read_lines;

/// --- Day 2: Rock Paper Scissors ---
pub fn solve(input_path: String) {
    let scenarios = [
        (["C X", "A Y", "B Z"], 6, "Z"),
        (["A X", "B Y", "C Z"], 3, "Y"),
        (["B X", "C Y", "A Z"], 0, "X")
    ];

    let mut total_score: u32 = 0;
    let mut total_score_elf: u32 = 0;

    // not ideal, should be possible to refactor it to be more DRY
    for line in read_lines(&input_path) {
        if let Ok(line) = line {
            for scenario in scenarios {
                let shape_value = scenario.0.iter().position(|&shapes| shapes == line);

                if shape_value != None {
                    total_score += scenario.1 + u32::try_from(shape_value.unwrap()).unwrap() + 1;
                }

                if line.contains(scenario.2) {
                    let shape_type = line.chars().next().unwrap().to_string();

                    let shape_value_elf = scenario.0.iter().position(|&shapes | shapes.starts_with(&shape_type));
                    
                    if shape_value_elf != None {
                        total_score_elf += scenario.1 + u32::try_from(shape_value_elf.unwrap()).unwrap() + 1; 
                    }
                }

            }
        }
    }
    println!("Your strategy guide total score: {total_score}\nElf's strategy guide total score: {total_score_elf}");
    
}