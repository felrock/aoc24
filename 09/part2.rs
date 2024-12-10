use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let file_path = "input.txt";

    if let Ok(lines) = read_lines(file_path) {
        for line in lines {
            if let Ok(disk_map) = line {
                let checksum = compact_disk(&disk_map);
                println!("Filesystem checksum: {}", checksum);
            }
        }
    }
}

fn compact_disk(disk_map: &str) -> usize {
    let mut blocks = parse_disk_map(disk_map);

    let files = collect_files(&blocks);

    for file in files.iter().rev() {

        let free_spans = collect_free_spans(&blocks);

        // Check if the file is already as left-aligned as possible
        let is_in_leftmost_position = free_spans
            .iter()
            .any(|&(start, end)| end - start + 1 >= file.len && start < file.start);

        if !is_in_leftmost_position {
            continue;
        }

        if let Some((span_start, _)) = find_free_span(&free_spans, file.len) {
            // Move the file into the free span
            for i in span_start..span_start + file.len {
                blocks[i] = Some(file.id);
            }

            // Clear the old file position
            for i in file.start..file.start + file.len {
                blocks[i] = None;
            }

        } else {
        }
    }

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
            id += 1;
        } else {
            blocks.extend(vec![None; length]);
        }
        is_file = !is_file;
    }

    blocks
}

fn collect_free_spans(blocks: &[Option<usize>]) -> Vec<(usize, usize)> {
    let mut spans = Vec::new();
    let mut start = None;

    for (i, block) in blocks.iter().enumerate() {
        match (start, block) {
            (None, None) => start = Some(i),
            (Some(s), Some(_)) => {
                spans.push((s, i - 1));
                start = None;
            }
            _ => {}
        }
    }

    if let Some(s) = start {
        spans.push((s, blocks.len() - 1));
    }

    spans
}

fn find_free_span(free_spans: &[(usize, usize)], file_len: usize) -> Option<(usize, usize)> {
    free_spans
        .iter()
        .find(|&&(start, end)| end - start + 1 >= file_len)
        .map(|&(start, _)| (start, start + file_len - 1))
}

fn collect_files(blocks: &[Option<usize>]) -> Vec<FileBlock> {
    let mut files = Vec::new();
    let mut current_id = None;
    let mut current_start = 0;
    let mut current_len = 0;

    for (i, &block) in blocks.iter().enumerate() {
        match (current_id, block) {
            (Some(id), Some(next_id)) if id == next_id => current_len += 1,
            (Some(id), _) => {
                files.push(FileBlock {
                    id,
                    start: current_start,
                    len: current_len,
                });
                current_id = block;
                current_start = i;
                current_len = 1;
            }
            (None, Some(id)) => {
                current_id = Some(id);
                current_start = i;
                current_len = 1;
            }
            _ => {}
        }
    }

    if let Some(id) = current_id {
        files.push(FileBlock {
            id,
            start: current_start,
            len: current_len,
        });
    }

    files
}

fn calculate_checksum(blocks: &[Option<usize>]) -> usize {
    let checksum: usize = blocks
        .iter()
        .enumerate()
        .filter_map(|(pos, block)| block.map(|id| pos * id))
        .sum();

    checksum
}

#[derive(Debug)]
struct FileBlock {
    id: usize,
    start: usize,
    len: usize,
}

fn read_lines<P>(filename: P) -> io::Result<std::io::Lines<std::io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
