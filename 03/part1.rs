use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {
    let file_path = "input.txt";
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results: Vec<i32> = vec![];

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(content) = line {
                for cap in re.captures_iter(&content) {
                    let num_a: i32 = cap[1].parse::<i32>().unwrap();
                    let num_b: i32 = cap[2].parse::<i32>().unwrap();
                    results.push(num_a * num_b);
                }
            }
        }
        let sum: i32 = results.iter().sum();

        // print the output here
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
