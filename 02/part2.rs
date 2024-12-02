use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "input.txt";
    let mut safe: i32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(content) = line {
                let numbers: Vec<i32> = content
                    .split_whitespace()
                    .filter_map(|s| s.parse::<i32>().ok())
                    .collect();

                if is_safe(&numbers) {
                    safe += 1;
                } else if can_be_safe_with_dampener(&numbers) {
                    safe += 1;
                }
            }
        }

        println!("answer: {}", safe);
    }
}

fn is_safe(numbers: &[i32]) -> bool {
    let is_ascending = numbers
        .windows(2)
        .all(|pair| pair[0] + 1 == pair[1] || pair[0] + 2 == pair[1] || pair[0] + 3 == pair[1]);
    let is_descending = numbers
        .windows(2)
        .all(|pair| pair[0] == pair[1] + 1 || pair[0] == pair[1] + 2 || pair[0] == pair[1] + 3);
    is_ascending || is_descending
}

fn can_be_safe_with_dampener(numbers: &[i32]) -> bool {
    for i in 0..numbers.len() {
        let mut modified = numbers.to_vec();
        modified.remove(i);
        if is_safe(&modified) {
            return true;
        }
    }
    false
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
