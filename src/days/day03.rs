use regex::Regex;

pub fn part1(lines: Vec<String>) {
    let pattern = Regex::new(r"mul\(([0-9]+),([0-9]+)\)")
        .expect("Invalid regex");
    let mut total: i64 = 0;
    for line in lines {
        for multiply in pattern.captures_iter(&line) {
            let (_, [arg1, arg2]) = multiply.extract();
            let int1: i64 = arg1.parse().expect("must be a number");
            let int2: i64 = arg2.parse().expect("must be a number");
            total += int1 * int2;
        }
    }
    println!("Total: {total}");
}
