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

pub fn part2(lines: Vec<String>) {
    let pattern = Regex::new(r"(mul)\(([0-9]+),([0-9]+)\)|(do)\(()()\)|(don't)\(()()\)")
        .expect("Invalid regex");
    let mut total: i64 = 0;
    let mut enabled = true;

    for line in lines {
        for function_call in pattern.captures_iter(&line) {
            let (_, [function_name, arg1, arg2]) = function_call.extract();
            if function_name != "mul" {
                println!("{function_name}()");
            }
            match function_name {
                "mul" => {
                    let int1: i64 = arg1.parse().expect("must be a number");
                    let int2: i64 = arg2.parse().expect("must be a number");
                    if !enabled {
                        print!("IGNORED -- ");
                    }
                    println!("mul({int1},{int2})");
                    if enabled {
                        total += int1 * int2;
                    }
                },
                "do" => enabled = true,
                "don't" => enabled = false,
                _ => panic!("Unrecognised function {function_name}"),
            }
        }
    }
    println!("Total: {total}");
}
