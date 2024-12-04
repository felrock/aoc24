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

        let rows = grid.len();
        let cols = grid[0].len();
        let mut count = 0;

        for row in 1..rows - 1 {
            for col in 1..cols - 1 {
                if is_x_mas(&grid, row, col) {
                    count += 1;
                }
            }
        }

        println!("answer: {}", count);
    }
}

fn is_x_mas(grid: &Vec<Vec<char>>, row: usize, col: usize) -> bool {
    if grid[row][col] != 'A' {
        return false;
    }

    let top_left_char = grid[row - 1][col - 1];
    let top_right_char = grid[row - 1][col + 1];
    let bottom_left_char = grid[row + 1][col - 1];
    let bottom_right_char = grid[row + 1][col + 1];

    let left_diagonal_mas = top_left_char == 'M' && bottom_right_char == 'S';
    let left_diagonal_sam = top_left_char == 'S' && bottom_right_char == 'M';

    let right_diagonal_mas = top_right_char == 'M' && bottom_left_char == 'S';
    let right_diagonal_sam = top_right_char == 'S' && bottom_left_char == 'M';

    (left_diagonal_mas || left_diagonal_sam) && (right_diagonal_mas || right_diagonal_sam)
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
