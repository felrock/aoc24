use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "input.txt";

    let mut answer = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(content) = line {
                let parts: Vec<&str> = content.split(':').collect();
                let target: i64 = parts[0].trim().parse().unwrap();
                let numbers: Vec<i64> = parts[1]
                    .trim()
                    .split_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect();

                if can_form_target(&numbers, target) {
                    answer += target;
                }
            }
        }

        println!("answer: {}", answer);
    }
}

fn can_form_target(numbers: &[i64], target: i64) -> bool {
    let n = numbers.len();
    if n == 0 {
        return false;
    }

    let ops_count = n - 1;
    let max_ways = 1 << (ops_count * 2);

    for mask in 0..max_ways {
        let mut result = numbers[0];
        let mut valid = true;

        for i in 0..ops_count {
            let op = (mask >> (i * 2)) & 3;

            if op == 0 {
                result += numbers[i + 1];
            } else if op == 1 {
                result *= numbers[i + 1];
            } else if op == 2 {
                result = concatenate(result, numbers[i + 1]);
            } else {
                valid = false;
                break;
            }
        }

        if valid && result == target {
            return true;
        }
    }

    false
}

fn concatenate(left: i64, right: i64) -> i64 {
    let mut right_part = right;
    let mut multiplier = 1;

    while right_part > 0 {
        multiplier *= 10;
        right_part /= 10;
    }

    left * multiplier + right
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
