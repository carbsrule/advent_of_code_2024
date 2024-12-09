const EMPTY_BLOCK: i32 = -1;

fn read_disk_map(lines: Vec<String>) -> Vec<u8> {
    const RADIX: u32 = 10;
    let mut disk_map = vec![];
    for line in lines {
        for num in line.chars().flat_map(|ch| ch.to_digit(RADIX)) {
            disk_map.push(num as u8);
        }
    }
    return disk_map;
}

fn read_blocks(lines: Vec<String>) -> Vec<i32> {
    let disk_map = read_disk_map(lines);
    let mut is_block = true;
    let mut file_id = 0;
    let mut blocks = vec![];
    for num in disk_map {
        if is_block {
            for _ in 0..num {
                blocks.push(file_id);
            }
            file_id += 1;
            is_block = false;
        } else {
            for _ in 0..num {
                blocks.push(EMPTY_BLOCK);
            }
            is_block = true;
        }
    }
    return blocks;
}

fn move_blocks(blocks: &mut Vec<i32>) {
    let mut percent_done = 0;
    let num_blocks = blocks.len();
    let ten_percent = num_blocks / 10;
    for i in (0..num_blocks).rev() {
        let file_id = blocks[i];
        if file_id != EMPTY_BLOCK {
            // println!("{file_id} at pos {i}");
            for j in 0..i {
                if blocks[j] == EMPTY_BLOCK {
                    // println!("Move to empty block at pos {i}");
                    blocks[j] = file_id;
                    blocks[i] = EMPTY_BLOCK;
                    break;
                }
            }
        }
        if i > 0 && i % ten_percent == 0 {
            percent_done += 10;
            println!("{percent_done}% done");
        }
    }
}

fn calc_checksum(blocks: Vec<i32>) -> u64 {
    println!("Calculating checksum...");
    let mut checksum: u64 = 0;
    let mut block_num: u64 = 0;
    for file_id in blocks {
        if file_id != EMPTY_BLOCK {
            checksum += block_num * file_id as u64;
        }
        block_num += 1;
    }
    return checksum;
}

pub fn part1(lines: Vec<String>) {
    let mut blocks = read_blocks(lines);
    println!("Read blocks; reorganising...");
    move_blocks(&mut blocks);

    let checksum = calc_checksum(blocks);
    println!("Checksum: {checksum}");
}

// Returns true if file was moved
fn move_file(blocks: &mut Vec<i32>, file_start: usize, file_length: usize) -> bool {
    let file_id = blocks[file_start];
    for i in 0..file_start {
        let mut all_free = true;
        for j in 0..file_length {
            if  blocks[i+j] != EMPTY_BLOCK {
                all_free = false;
                break;
            }
        }
        if !all_free {
            continue;
        }

        // move blocks into free space
        let mut file_block = file_start;
        for j in i..i+file_length {
            blocks[j] = blocks[file_block];
            blocks[file_block] = EMPTY_BLOCK;
            file_block += 1;
        }
        // println!("Moving file {file_id} of length {file_length} from {file_start} to {i}");
        return true;
    }
    // println!("No space found to move file {file_id} of length {file_length} starting at {file_start}");
    return false;
}

fn defrag_blocks(blocks: &mut Vec<i32>) {
    let num_blocks = blocks.len();
    let mut percent_done = 0;
    let mut last_file_id = EMPTY_BLOCK;
    let mut file_end = num_blocks;
    let mut read_position = num_blocks - 1;
    while read_position > 0 {
        let file_id = blocks[read_position];
        // println!("Read @ {read_position}, file_id: {file_id}, last: {last_file_id}, end: {file_end}");
        let mut keep_moving = true;
        if file_id != last_file_id {
            if last_file_id == EMPTY_BLOCK {
                file_end = read_position + 1;
            } else {
                let file_start = read_position + 1;
                let file_length = file_end - file_start;
                // println!("File {last_file_id} starts at {file_start}, with length {file_length} from end {file_end}");
                move_file(blocks, file_start, file_length);
                keep_moving = false;
                file_end = file_start;
            }
            last_file_id = file_id;
        }
        if keep_moving {
            read_position -= 1;
        }

        let new_percent_done = 100 - (read_position * 100 / num_blocks);
        if new_percent_done % 10 == 0 && new_percent_done != percent_done {
            percent_done = new_percent_done;
            println!("{percent_done}% done");
        }
    }
}

pub fn part2(lines: Vec<String>) {
    let mut blocks = read_blocks(lines);

    println!("Read blocks; reorganising...");
    defrag_blocks(&mut blocks);

    let checksum = calc_checksum(blocks);
    println!("Checksum: {checksum}");
}
