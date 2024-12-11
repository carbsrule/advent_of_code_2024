use regex::Regex;

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
