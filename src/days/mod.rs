use crate::lines;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;

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
        (2, 2) => day02::part2(lines),
        (3, 1) => day03::part1(lines),
        (3, 2) => day03::part2(lines),
        (4, 1) => day04::part1(lines),
        (4, 2) => day04::part2(lines),
        (5, 1) => day05::part1(lines),
        _ => println!("Implementation not available"),
    }
}
