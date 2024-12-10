use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "input.txt";

    if let Ok(lines) = read_lines(file_path) {
        let mut map = vec![];
        let mut guard_position = (0, 0);
        let mut guard_direction = (0, -1);

        for (y, line) in lines.enumerate() {
            if let Ok(content) = line {
                let mut row = Vec::with_capacity(content.len());
                for (x, cell) in content.bytes().enumerate() {
                    row.push(cell as char);
                    if "^v<>".contains(cell as char) {
                        guard_position = (x as isize, y as isize);
                        guard_direction = match cell as char {
                            '^' => (0, -1),
                            'v' => (0, 1),
                            '<' => (-1, 0),
                            '>' => (1, 0),
                            _ => (0, 0),
                        };
                    }
                }
                map.push(row);
            }
        }

        let width = map[0].len() as isize;
        let height = map.len() as isize;

        let mut valid_positions = Vec::new();

        for y in 0..height {
            for x in 0..width {
                if map[y as usize][x as usize] == '#' || (x, y) == guard_position {
                    continue;
                }

                let mut modified_map = map.clone();
                modified_map[y as usize][x as usize] = '#';

                if causes_loop(guard_position, guard_direction, &modified_map, width, height) {
                    valid_positions.push((x, y));
                }
            }
        }
        println!("Answer: {}", valid_positions.len());
    }
}

fn causes_loop(
    start_position: (isize, isize),
    start_direction: (isize, isize),
    map: &[Vec<char>],
    width: isize,
    height: isize,
) -> bool {
    let mut visited_states = HashSet::new();
    let mut guard_position = start_position;
    let mut guard_direction = start_direction;

    while is_inside(guard_position, width, height) {
        if !visited_states.insert((guard_position, guard_direction)) {
            return true;
        }

        let next_position = (
            guard_position.0 + guard_direction.0,
            guard_position.1 + guard_direction.1,
        );

        if is_inside(next_position, width, height) {
            if map[next_position.1 as usize][next_position.0 as usize] != '#' {
                guard_position = next_position;
            } else {
                guard_direction = turn_right(guard_direction);
            }
        } else {
            return false;
        }
    }

    false
}

fn turn_right(direction: (isize, isize)) -> (isize, isize) {
    match direction {
        (0, -1) => (1, 0),
        (1, 0) => (0, 1),
        (0, 1) => (-1, 0),
        (-1, 0) => (0, -1),
        _ => (0, 0),
    }
}

fn is_inside(position: (isize, isize), width: isize, height: isize) -> bool {
    position.0 >= 0 && position.0 < width && position.1 >= 0 && position.1 < height
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
