use regex::Regex;

fn get_left_and_right(lines: Vec<String>) -> (Vec<i32>, Vec<i32>) {
    let pattern = Regex::new(r"\s+").expect("Invalid regex");
    let mut left_side: Vec<i32> = Vec::new();
    let mut right_side: Vec<i32> = Vec::new();
    for line in lines {
        let mut sides = pattern.split(&line);
        let left_val: i32 = sides.next().unwrap().parse()
            .expect("left side must be number");
        let right_val: i32 = sides.next().unwrap().parse()
            .expect("right side must be number");
        left_side.push(left_val);
        right_side.push(right_val);
    }

    return (left_side, right_side);
}

pub fn part1(lines: Vec<String>) {
    let (mut left_side, mut right_side) = get_left_and_right(lines);

    left_side.sort();
    right_side.sort();

    let mut total_distance: i32 = 0;
    let mut idx: usize = 0;
    for left_val in left_side {
        let right_val = right_side[idx];
        idx += 1;
        let diff = (left_val - right_val).abs();
        // println!("diff({left_val}, {right_val}): {diff}");
        total_distance += diff;
    }
    println!("Total: {total_distance}");
}

pub fn part2(lines: Vec<String>) {
    let (left_side, right_side) = get_left_and_right(lines);


    let mut total_similarity = 0;
    for left_el in left_side {
        let mut appearances = 0;
        for right_el in &right_side {
            if left_el == *right_el {
                appearances += 1;
            }
        }
        let similarity = left_el * appearances;
        println!("similarity({left_el}, {appearances}): {similarity}");
        total_similarity += similarity;
    }
    println!("Total similarity: {total_similarity}");
}
