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
        let rows = map.len();
        let cols = map[0].len();

        for (_, positions) in &antennas {
            let n = positions.len();

            for &(x, y) in positions {
                antinodes.insert((x, y));
            }

            for i in 0..n {
                for j in i + 1..n {
                    let (x1, y1) = positions[i];
                    let (x2, y2) = positions[j];

                    let dx = x2 as isize - x1 as isize;
                    let dy = y2 as isize - y1 as isize;
                    let gcd = gcd(dx, dy);

                    let step_x = dx / gcd;
                    let step_y = dy / gcd;

                    for &(start_x, start_y) in &[(x1, y1), (x2, y2)] {
                        let mut x = start_x as isize;
                        let mut y = start_y as isize;
                        while is_within_bounds(x, y, rows, cols) {
                            antinodes.insert((x as usize, y as usize));
                            x += step_x;
                            y += step_y;
                        }

                        let mut x = start_x as isize - step_x;
                        let mut y = start_y as isize - step_y;
                        while is_within_bounds(x, y, rows, cols) {
                            antinodes.insert((x as usize, y as usize));
                            x -= step_x;
                            y -= step_y;
                        }
                    }
                }
            }
        }
        println!("answer: {}", antinodes.len());
    }
}

fn gcd(a: isize, b: isize) -> isize {
    if b == 0 {
        a.abs()
    } else {
        gcd(b, a % b)
    }
}

fn is_within_bounds(x: isize, y: isize, rows: usize, cols: usize) -> bool {
    x >= 0 && y >= 0 && (x as usize) < rows && (y as usize) < cols
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
