use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "input.txt";
    let mut grid = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(content) = line {
                grid.push(content.chars().collect::<Vec<char>>());
            }
        }

        let directions = [
            (0, 1),
            (1, 0),
            (1, 1),
            (1, -1),
            (0, -1),
            (-1, 0),
            (-1, -1),
            (-1, 1),
        ];
        let word_chars = ['X', 'M', 'A', 'S'];
        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;

        for row in 0..rows {
            for col in 0..cols {
                for &(dr, dc) in &directions {
                    if is_word_at(&grid, &word_chars, row, col, dr, dc, rows, cols) {
                        count += 1;
                    }
                }
            }
        }
        println!("answer: {}", count);
    }
}

fn is_word_at(
    grid: &Vec<Vec<char>>,
    word: &[char; 4],
    row: usize,
    col: usize,
    dr: isize,
    dc: isize,
    rows: usize,
    cols: usize,
) -> bool {
    let word_len = 4;
    for i in 0..word_len {
        let nr = row as isize + dr * i as isize;
        let nc = col as isize + dc * i as isize;
        if nr < 0 || nr >= rows as isize || nc < 0 || nc >= cols as isize {
            return false;
        }
        if grid[nr as usize][nc as usize] != word[i] {
            return false;
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
