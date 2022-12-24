use crate::read_lines;

type ElfTask = (u32, u32);

fn is_contained(tuple_a: ElfTask, tuple_b: ElfTask) -> bool {
    return tuple_a.0 >= tuple_b.0 && tuple_a.1 <= tuple_b.1;
}

fn is_overlapped(tuple_a: ElfTask, tuple_b: ElfTask) -> bool {
    return tuple_a.0 <= tuple_b.0 && tuple_a.1 >= tuple_b.0;
}

fn test_both_ways(predicate_fce: fn(ElfTask, ElfTask) -> bool, tuple_a: ElfTask, tuple_b: ElfTask) -> bool {
    return predicate_fce(tuple_a, tuple_b) || predicate_fce(tuple_b, tuple_a);
}

/// --- Day 4: Camp Cleanup ---
pub fn solve(input_path: String) {
    let mut fully_contained_sum = 0;
    let mut overlapped_sum = 0;

    for line in read_lines(&input_path) {
        if let Ok(line) = line {
            let elf_pair: Vec<u32> = line.split(&['-', ','][..]).map(|x| x.parse::<u32>().unwrap()).collect();
            
            let elf_a: ElfTask = (elf_pair[0], elf_pair[1]);
            let elf_b: ElfTask = (elf_pair[2], elf_pair[3]);

            if test_both_ways(is_contained, elf_a, elf_b) {
                fully_contained_sum += 1;
            }

            if test_both_ways(is_overlapped, elf_a, elf_b) {
                overlapped_sum += 1
            }
        }
    }
    println!("There are {fully_contained_sum} fully contained pairs.");
    println!("There are {overlapped_sum} partly overlapping pairs.");
}