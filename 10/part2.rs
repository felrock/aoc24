use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "input.txt";

    if let Ok(lines) = read_lines(file_path) {
        let mut map: Vec<Vec<u8>> = vec![];

        for line in lines {
            if let Ok(content) = line {
                map.push(content.chars().map(|c| c.to_digit(10).unwrap() as u8).collect());
            }
        }

        let answer = calculate_trailhead_ratings(&map);
        println!("answer: {}", answer);
    }
}

fn calculate_trailhead_ratings(map: &Vec<Vec<u8>>) -> usize {
    let mut total_rating = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 0 {
                total_rating += trailhead_rating(map, row, col);
            }
        }
    }

    total_rating
}

fn trailhead_rating(map: &Vec<Vec<u8>>, start_row: usize, start_col: usize) -> usize {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut visited = vec![vec![false; map[0].len()]; map.len()];
    let mut stack = vec![(start_row, start_col, 0)];
    let mut rating = 0;

    while let Some((row, col, height)) = stack.pop() {
        if row >= map.len() || col >= map[0].len() || visited[row][col] || map[row][col] != height {
            continue;
        }

        visited[row][col] = true;

        if height == 9 {
            rating += 1;
            visited[row][col] = false;
            continue;
        }

        for &(dr, dc) in &directions {
            let new_row = row as isize + dr;
            let new_col = col as isize + dc;

            if new_row >= 0 && new_row < map.len() as isize && new_col >= 0 && new_col < map[0].len() as isize {
                stack.push((new_row as usize, new_col as usize, height + 1));
            }
        }

        visited[row][col] = false;
    }

    rating
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
