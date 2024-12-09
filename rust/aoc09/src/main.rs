use std::io::{self, Read};

type Result<T> = ::std::result::Result<T, Box<dyn::std::error::Error>>;

fn main() -> Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    part1(&input)?;
    part2(&input)?;

    Ok(())
}

fn part1(input: &str) -> Result<()> {
    let mut blocks: Vec<i64> = Vec::new();
    let lines = input.trim().lines().collect::<Vec<_>>();
    let first_line = lines[0];
    let mut id = 0;
    for (i, c) in first_line.chars().enumerate() {
        if i % 2 == 0 {
            for _ in 0..c.to_digit(10).unwrap() {
                blocks.push(id);
            }
            id += 1;
        } else {
            for _ in 0..c.to_digit(10).unwrap() {
                blocks.push(-1);
            }
        }
    }

    let mut free_ptr = next_free(&blocks);
    let mut file_ptr = next_file(&blocks);

    loop {
        if free_ptr >= blocks.len() {
            break;
        }

        if file_ptr < 0 {
            break;
        }

        if blocks[free_ptr] == -1 {
            blocks[free_ptr] = blocks[file_ptr as usize];
            blocks[file_ptr as usize] = -1;
            file_ptr = next_file(&blocks);
            free_ptr = next_free(&blocks);
        }

        if free_ptr > file_ptr as usize {
            break;
        }
    }

    println!("checksum: {}", checksum(&blocks));

    Ok(())
}

type Block = Option<usize>;

fn parse_disk_map(disk_map_str: &str) -> Vec<Block> {
    let chars: Vec<char> = disk_map_str.trim().chars().collect();
    let mut i = 0;
    let length = chars.len();
    let mut blocks = Vec::new();
    let mut file_id = 0;

    while i < length {
        let f = chars[i].to_digit(10).unwrap() as usize;
        i += 1;
        let s = if i < length {
            let val = chars[i].to_digit(10).unwrap() as usize;
            i += 1;
            val
        } else {
            0
        };

        // Add f file blocks
        for _ in 0..f {
            blocks.push(Some(file_id));
        }
        file_id += 1;

        // Add s free blocks
        for _ in 0..s {
            blocks.push(None);
        }
    }

    blocks
}

fn part2(input: &str) -> Result<()> {
    // let mut blocks = Vec::new();
    let mut blocks = parse_disk_map(input);

    // Identify the number of files
    let max_file_id = blocks.iter().filter_map(|b| *b).max().unwrap_or(0);
    let total_files = max_file_id + 1;

    // For each file, find its starting position (min index) and count how many blocks it has.
    let mut min_index_for_file = vec![usize::MAX; total_files];
    let mut file_block_count = vec![0; total_files];

    for (idx, blk) in blocks.iter().enumerate() {
        if let Some(f) = blk {
            if idx < min_index_for_file[*f] {
                min_index_for_file[*f] = idx;
            }
            file_block_count[*f] += 1;
        }
    }

    // Move files from highest ID to lowest ID
    for current_file in (0..total_files).rev() {
        let file_start = min_index_for_file[current_file];
        let file_length = file_block_count[current_file];
        if file_length == 0 {
            // No blocks for this file - should not happen normally
            continue;
        }

        // Find a contiguous free-space run to the left of file_start that can hold `file_length`.
        let mut suitable_position = None;
        let mut gap_start = None;
        let mut gap_length = 0;

        for pos in 0..file_start {
            if blocks[pos].is_none() {
                if gap_start.is_none() {
                    gap_start = Some(pos);
                    gap_length = 1;
                } else {
                    gap_length += 1;
                }

                if gap_length == file_length {
                    suitable_position = gap_start;
                    break;
                }
            } else {
                // Not free, reset gap
                gap_start = None;
                gap_length = 0;
            }
        }

        if let Some(sp) = suitable_position {
            // Move the file
            // Collect current positions of this file's blocks:
            let mut file_positions_list = Vec::with_capacity(file_length);
            let mut found = 0;
            for pos in file_start..blocks.len() {
                if found == file_length {
                    break;
                }
                if blocks[pos] == Some(current_file) {
                    file_positions_list.push(pos);
                    found += 1;
                }
            }

            // Place file blocks at sp..sp+file_length-1
            for offset in 0..file_length {
                blocks[sp + offset] = Some(current_file);
            }

            // Mark old positions as free
            for old_pos in file_positions_list {
                blocks[old_pos] = None;
            }

            // Update file's new start index
            min_index_for_file[current_file] = sp;
        } else {
            // No suitable gap found, file stays put
        }
    }

    // Compute the checksum after rearranging
    let checksum = compute_checksum(&blocks);

    println!("{}", checksum);

    Ok(())
}

fn next_free(blocks: &[i64]) -> usize {
    let mut free_ptr = 0;
    loop {
        if blocks[free_ptr] == -1 {
            break;
        }
        free_ptr += 1;
    }

    free_ptr
}

fn next_file(blocks: &[i64]) -> i64 {
    let mut file_ptr = blocks.len() as i64 - 1;
    loop {
        if blocks[file_ptr as usize] != -1 {
            break;
        }
        file_ptr -= 1;
    }

    file_ptr
}

fn checksum(blocks: &[i64]) -> i64 {
    let mut checksum = 0;
    for (i, block) in blocks.iter().enumerate() {
        if *block != -1 {
            checksum += i as i64 * *block;
        }
    }

    checksum
}

fn compute_checksum(blocks: &[Block]) -> usize {
    let mut checksum = 0;
    for (pos, block) in blocks.iter().enumerate() {
        if let Some(f) = block {
            checksum += pos * f;
        }
    }
    checksum
}
