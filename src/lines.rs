use std::io;

pub fn read_line() -> (usize, String) {
    let mut line = String::new();
    let num_bytes = io::stdin()
        .read_line(&mut line)
        .expect("Failed to read line");
    if num_bytes == 0 {
        line = "".to_string();
    } else {
        line = line.trim().to_string();
    }
    return (num_bytes, line);
}
