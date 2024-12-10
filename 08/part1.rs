use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "input.txt";

    if let Ok(lines) = read_lines(file_path) {
        let map: Vec<Vec<char>> = lines
            .filter_map(|line| line.ok())
            .map(|line| line.chars().collect())
            .collect();

        let mut antennas: HashMap<char, Vec<(usize, usize)>> = HashMap::new();

        for (row, line) in map.iter().enumerate() {
            for (col, &ch) in line.iter().enumerate() {
                if ch != '.' {
                    antennas.entry(ch).or_default().push((row, col));
                }
            }
        }

        let mut antinodes = HashSet::new();

        for (_, positions) in &antennas {
            let n = positions.len();
            for i in 0..n {
                for j in i + 1..n {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];

                    let dx = x2 as isize - x1 as isize;
                    let dy = y2 as isize - y1 as isize;

                    // First antinode (closer to the first antenna)
                    let ax1 = x1 as isize - dx;
                    let ay1 = y1 as isize - dy;
                    if is_within_bounds(ax1, ay1, &map) {
                        antinodes.insert((ax1 as usize, ay1 as usize));
                    }

                    // Second antinode (further away from the second antenna)
                    let ax2 = x2 as isize + dx;
                    let ay2 = y2 as isize + dy;
                    if is_within_bounds(ax2, ay2, &map) {
                        antinodes.insert((ax2 as usize, ay2 as usize));
                    }
                }
            }
        }

        let mut final_grid = map.clone();
        for &(x, y) in &antinodes {
            if final_grid[x][y] == '.' {
                final_grid[x][y] = '#';
            }
        }
        println!("answer: {}", antinodes.len());
    }
}

fn is_within_bounds(x: isize, y: isize, map: &[Vec<char>]) -> bool {
    x >= 0 && y >= 0 && (x as usize) < map.len() && (y as usize) < map[0].len()
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
