mod lines;

fn main() {
    let (bytes, line) = lines::read_line();
    if bytes > 0 {
        println!("{line}");
    } else {
        println!("Hello, world!");
    }
}
