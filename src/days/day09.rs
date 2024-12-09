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

pub fn part1(lines: Vec<String>) {
    let mut blocks = read_blocks(lines);
    println!("Read blocks; reorganising...");
    move_blocks(&mut blocks);

    println!("Calculating checksum...");
    let mut checksum: u64 = 0;
    let mut block_num = 0;
    for file_id in blocks {
        if file_id == EMPTY_BLOCK {
            break;
        }
        checksum += block_num as u64 * file_id as u64;
        block_num += 1;
    }
    println!("Checksum: {checksum}");
}
