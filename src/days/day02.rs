use regex::Regex;

fn report_is_safe(report: &str) -> (bool, String) {
    let pattern = Regex::new(r"\s+").expect("Invalid regex");
    let mut last_level_opt: Option<i32> = None;
    let mut direction: char = '?';
    let mut pos = 0;
    for level in pattern.split(report) {
        let level: i32 = level.parse().expect("Not a number");
        if last_level_opt.is_some() {
            let last_level = last_level_opt.unwrap();
            let change = level - last_level;
            let new_direction = if change > 0 {
                '+'
            } else {
                '-'
            };
            if direction != new_direction && direction != '?' {
                return (false, format!("[{pos}] direction mismatch: {direction} {new_direction}"));
            }
            direction = new_direction;
            let abs_change = change.abs();
            if abs_change == 0 || abs_change > 3 {
                return (false, format!("[{pos}] change: {change}"));
            }
        }
        pos += 1;
        last_level_opt = Some(level);
    }
    return (true, String::from(""));
}

pub fn part1(lines: Vec<String>) {
    let mut safe_reports = 0;
    for report in lines {
        let (safe, reason) = report_is_safe(&report);
        if safe {
            println!("{report}: SAFE");
            safe_reports += 1;
        } else {
            println!("{report}: UNSAFE ({reason})");
        }
    }
    println!("Total: {safe_reports}");
}

pub fn part2(lines: Vec<String>) {
    let mut safe_reports = 0;
    for report in lines {
        let (safe, reason) = report_is_safe(&report);
        if safe {
            println!("{report}: SAFE");
            safe_reports += 1;
        } else {
            let pattern = Regex::new(r"\s+").expect("Invalid regex");
            let mut parts = pattern.split(&report);
            let num_parts = parts.count();
            let mut dampened_is_safe = false;
            for remove in 0..num_parts {
                let mut dampened: Vec<String> = vec![];
                let mut idx = 0;
                let mut removed_element = "";

                // gets consumed each time
                parts = pattern.split(&report);
                for part in parts {
                    let skip_el = idx == remove;
                    idx += 1;
                    if skip_el {
                        removed_element = part;
                        continue;
                    }
                    dampened.push(part.to_string());
                }
                let dampened_report = dampened.join(" ");
                (dampened_is_safe, _) = report_is_safe(&dampened_report);
                if dampened_is_safe {
                    safe_reports += 1;
                    println!("{report}: SAFE (remove value {removed_element} at index {remove})");
                    break;
                }
            }
            if !dampened_is_safe {
                println!("{report}: UNSAFE ({reason})");
            }
        }
    }
    println!("Total: {safe_reports}");
}
