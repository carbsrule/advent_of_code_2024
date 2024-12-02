use crate::lines;

mod day01;
mod day02;

pub fn run(day: u8, part: u8) {
    let mut lines: Vec<String> = Vec::new();
    loop {
        let (bytes, line) = lines::read_line();
        if bytes < 1 {
            break;
        }
        lines.push(line)
    }

    match (day, part) {
        (1, 1) => day01::part1(lines),
        (1, 2) => day01::part2(lines),
        (2, 1) => day02::part1(lines),
        _ => println!("Implementation not available"),
    }
}
