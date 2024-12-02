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

                let is_ascending = numbers
                    .windows(2)
                    .all(|pair| pair[0] + 1 == pair[1] || pair[0] + 2 == pair[1]
                        || pair[0] + 3 == pair[1]);

                let is_descending = numbers
                    .windows(2)
                    .all(|pair| pair[1] + 1 == pair[0] || pair[1] + 2 == pair[0]
                        || pair[1] + 3 == pair[0]);

                if is_ascending || is_descending {
                    safe += 1;
                }
            }
        }

        // Print the output
        println!("answer: {}", safe);
    }
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
