use regex::Regex;
use std::collections::HashMap;

#[derive(Debug)]
struct Stone {
    value: u64,
    blinks: u8,
}

fn blink(stones: Vec<u64>) -> Vec<u64> {
    let mut new_stones: Vec<u64> = vec![];
    for stone in stones {
        if stone == 0 {
            new_stones.push(1);
            continue;
        }
        let stone_str = stone.to_string();
        let num_digits = stone_str.len();
        if num_digits % 2 == 0 {
            let halfway = num_digits / 2;
            new_stones.push(stone_str[0..halfway].parse().expect("number"));
            new_stones.push(stone_str[halfway..].parse().expect("number"));
        } else {
            new_stones.push(stone * 2024);
        }
    }
    return new_stones;
}

fn load_stones(lines: Vec<String>) -> Vec<u64> {
    let pattern = Regex::new(r"\s+").expect("Invalid regex");
    let mut stones: Vec<u64> = vec![];
    for line in lines {
        for stone in pattern.split(&line) {
            stones.push(stone.parse().expect("number"));
        }
    }
    println!("Starting stones: {:?}", stones);
    return stones;
}

pub fn part1(lines: Vec<String>) {
    let mut stones = load_stones(lines);
    for _ in 0..25 {
        stones = blink(stones);
    }
    println!("Final number of stones: {}", stones.len())
}

fn get_known_count(value: u64, blinks_to_finish: u8, seen: &HashMap<(u64,u8), u64>) -> Option<u64> {
    let has_count = seen.get(&(value, blinks_to_finish));
    match has_count {
        Some(final_count) => {
            // println!("Pulled count from map for value {value} for final {} blinks", blinks_to_finish);
            return Some(*final_count);
        },
        _ => return None,
    }
}

fn save_known_count(value: u64, blinks_to_finish: u8, total: u64, seen: &mut HashMap<(u64,u8), u64>) {
    seen.insert((value, blinks_to_finish), total);
}

// hash map: (value, steps to finish) -> count
fn count_stones_produced(stone: Stone, total_blinks: u8, seen: &mut HashMap<(u64,u8), u64>) -> u64 {
    // println!("Checking stone: {:?}", stone);
    let value = stone.value;
    let blinks_to_finish = total_blinks - stone.blinks;
    if blinks_to_finish == 0 {
        return 1;
    }

    let known_count = get_known_count(value, blinks_to_finish, seen);
    match known_count {
        Some(known_value) => {
            return known_value;
        }
        _ => (),
    }

    // Need to count from scratch
    let mut total = 0;
    let next_blink = blink(vec![stone.value]);
    for child_value in next_blink {
        let child_stone = Stone{value: child_value, blinks: stone.blinks + 1};
        total += count_stones_produced(child_stone, total_blinks, seen);

    }

    save_known_count(value, blinks_to_finish, total, seen);

    return total;
}

// Duplicate of part 2 that uses a cachey thing
pub fn part2(lines: Vec<String>) {
    let stones_vals = load_stones(lines);
    let mut total = 0;
    let mut cached_totals: HashMap<(u64,u8), u64> = HashMap::new();
    for value in stones_vals {
        let stone = Stone{value, blinks: 0};
        total += count_stones_produced(stone, 75, &mut cached_totals);
    }
    println!("Total stones at end: {total}");
}
