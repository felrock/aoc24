use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

fn main() {
    let file_path = "input.txt";
    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(content) = line {
                let parts: Vec<i32> = content
                    .split("   ")
                    .filter_map(|part| part.trim().parse::<i32>().ok())
                    .collect();

                if parts.len() == 2 {
                    left_list.push(parts[0]);
                    right_list.push(parts[1]);
                }
            }
        }

        left_list.sort();
        right_list.sort();

        let mut right_map: HashMap<i32, i32> = HashMap::new();
        for &item in &right_list {
            *right_map.entry(item).or_insert(0) += 1;
        }

        let sum: i32 = left_list
            .iter()
            .map(|&item| item * right_map.get(&item).unwrap_or(&0))
            .sum();

        println!("answer: {}", sum);
    }
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
