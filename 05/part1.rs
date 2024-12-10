use std::collections::{HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "input.txt";

    if let Ok(lines) = read_lines(file_path) {
        let mut rules = vec![];
        let mut updates = vec![];
        let mut parsing_rules = true;

        // Read the input into rules and updates
        for line in lines {
            if let Ok(content) = line {
                if content.is_empty() {
                    parsing_rules = false;
                    continue;
                }
                if parsing_rules {
                    let parts: Vec<&str> = content.split('|').collect();
                    if parts.len() == 2 {
                        let x = parts[0].parse::<u32>().unwrap();
                        let y = parts[1].parse::<u32>().unwrap();
                        rules.push((x, y));
                    }
                } else {
                    let update: Vec<u32> = content.split(',')
                        .map(|x| x.parse::<u32>().unwrap())
                        .collect();
                    updates.push(update);
                }
            }
        }

        // Check each update for correct order
        let mut valid_middle_sum = 0;

        for update in &updates {
            if is_update_order_valid(update, &rules) {
                // Find the middle page of the update
                let middle = update[update.len() / 2];
                valid_middle_sum += middle;
            }
        }

        println!("answer: {}", valid_middle_sum);
    }
}

fn is_update_order_valid(update: &[u32], rules: &[(u32, u32)]) -> bool {
    let mut index_map = HashMap::new();
    for (i, &page) in update.iter().enumerate() {
        index_map.insert(page, i);
    }

    for &(x, y) in rules {
        if let (Some(&i_x), Some(&i_y)) = (index_map.get(&x), index_map.get(&y)) {
            if i_x > i_y {
                return false;
            }
        }
    }

    true
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
