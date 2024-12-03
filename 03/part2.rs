use std::fs;
use regex::Regex;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let re_mul = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut instructions: Vec<(usize, bool, Option<(i32, i32)>)> = vec![];

    for mat in re_mul.find_iter(&content) {
        if let Some(caps) = re_mul.captures(mat.as_str()) {
            let num_a = caps[1].parse::<i32>().unwrap();
            let num_b = caps[2].parse::<i32>().unwrap();
            instructions.push((mat.start(), false, Some((num_a, num_b))));
        }
    }
    for mat in re_do.find_iter(&content) {
        instructions.push((mat.start(), true, None));
    }
    for mat in re_dont.find_iter(&content) {
        instructions.push((mat.start(), false, None));
    }

    instructions.sort_by_key(|&(pos, _, _)| pos);

    let mut mul_enabled = true;
    let mut sum = 0;

    for (_, is_do, mul_data) in instructions {
        if let Some((num_a, num_b)) = mul_data {
            if mul_enabled {
                sum += num_a * num_b;
            }
        } else {
            mul_enabled = is_do;
        }
    }

    println!("answer: {}", sum);
}
