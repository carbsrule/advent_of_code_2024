mod args;
mod days;

fn main() {
    let (day, part) = args::get_day_part();
    days::run(day, part);
}
