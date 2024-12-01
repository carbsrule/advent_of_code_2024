use regex::Regex;

pub fn part1(lines: Vec<String>) {
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
