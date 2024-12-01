use std::env;
use std::io;

pub fn get_part() -> u32 {
    let part: u32 = env::args().nth(1)
        .unwrap_or("1".to_string())
        .parse()
        .expect("Failed to parse part arg");
    if part != 2 {
        return 1;
    }
    return part;
}

pub fn parse_int(num: &str) -> i32 {
    return num.parse().expect("Failed to parse number");
}

pub fn char_parse_int(ch: &char) -> i32 {
    let mut num = String::new();
    num.push(*ch);
    return parse_int(&num);
}
