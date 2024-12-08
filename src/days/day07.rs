use std::collections::HashMap;
use regex::Regex;

fn generate_permutations(operators: Vec<char>, len: usize) -> Vec<Vec<char>> {
    let mut all_perms: Vec<Vec<char>> = vec![];
    all_perms.push(vec![]);
    for _ in 0..len {
        let mut new_perms = vec![];
        for op in &operators {
            for perm in &all_perms {
                let mut new_perm = perm.clone();
                new_perm.push(*op);
                new_perms.push(new_perm);
            }
        }
        all_perms = new_perms;
    }
    return all_perms;
}

fn get_permutations(map: &mut HashMap<usize, Vec<Vec<char>>>, allowed_ops: Vec<char>, len: usize) -> &Vec<Vec<char>> {
    return map.entry(len).or_insert(generate_permutations(allowed_ops, len));
}

fn parse_line(line: String, splitter: &Regex) -> (i64, i64, Vec<i64>) {
    let mut total = 0;
    let mut first_number = 0;
    let mut subsequent_numbers = vec![];
    let mut i = 0;
    for val in splitter.split(&line) {
        let number = val.parse().expect("number");
        if i == 0 {
            total = number;
        } else if i == 1 {
            first_number = number;
        } else {
            subsequent_numbers.push(number);
        }
        i += 1;
    }
    return (total, first_number, subsequent_numbers);
}

fn handle_operation(total: &mut i64, operand: &i64, operator: char) {
    match operator {
        '+' => *total += operand,
        '*' => *total *= operand,
        _ => panic!("invalid operator"),
    }
}

fn calibrate(lines: Vec<String>, allowed_ops: Vec<char>) -> i64 {
    let mut perms_by_length = HashMap::new();
    let pattern = Regex::new(r":?\s+").expect("Invalid regex");
    let mut total_matches = 0;
    for line in lines {
        let (expected_total, first_num, operands) = parse_line(line, &pattern);
        let perms = get_permutations(&mut perms_by_length, allowed_ops.clone(), operands.len() + 1);
        for operators in perms {
            let mut total = first_num;
            for i in 0..operands.len() {
                handle_operation(&mut total, &operands[i], operators[i]);
            }
            if total == expected_total {
                println!("Match: {:?}, {:?}", operators, [vec![first_num], operands].concat());
                total_matches += total;
                break;
            }
        }
    }
    return total_matches;
}

pub fn part1(lines: Vec<String>) {
    let calibration_result = calibrate(lines, vec!['+', '*']);
    println!("Total: {calibration_result}");
}
