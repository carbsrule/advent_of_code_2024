mod day01;

pub fn run(day: u8, part: u8) {
    match (day, part) {
        (1, 1) => day01::part1(),
        _ => println!("Implementation not available"),
    }
}
