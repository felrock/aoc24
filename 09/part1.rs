use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "input.txt";

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(disk_map) = line {
                let checksum = compact_disk(&disk_map);
                println!("checksum: {}", checksum);
            }
        }
    }
}

fn compact_disk(disk_map: &str) -> usize {
    let mut blocks = parse_disk_map(disk_map);

    let mut left = 0; // Pointer for free space (None)
    let mut right = blocks.len() - 1; // Pointer for file blocks (Some(id))

    while left < right {
        // Move `left` to the next free space
        while left < right && blocks[left].is_some() {
            left += 1;
        }

        // Move `right` to the next file block
        while left < right && blocks[right].is_none() {
            right -= 1;
        }

        // Swap free space with file block
        if left < right {
            blocks.swap(left, right);
            left += 1;
            right -= 1;
        }
    }


    // Calculate checksum
    let checksum = calculate_checksum(&blocks);
    checksum
}

fn parse_disk_map(disk_map: &str) -> Vec<Option<usize>> {
    let mut blocks = Vec::new();
    let chars: Vec<char> = disk_map.chars().collect();
    let mut is_file = true;
    let mut id = 0;

    for i in 0..chars.len() {
        let length = chars[i].to_digit(10).unwrap() as usize;
        if is_file {
            blocks.extend(vec![Some(id); length]);
            id+=1;
        } else {
            blocks.extend(vec![None; length]);
        }
        is_file = !is_file;
    }

    blocks
}

fn calculate_checksum(blocks: &[Option<usize>]) -> usize {
    let checksum: usize = blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, block)| block.map(|id| {
            let value = pos * id;
            value
        }))
        .sum();

    checksum
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
