mod args;
mod days;
mod lines;

fn main() {
    let (day, part) = args::get_day_part();
    days::run(day, part);
}
