fn read_rules_and_updates(lines: Vec<String>) -> (Vec<(u32, u32)>, Vec<Vec<u32>>) {
    let mut reading_order = true;
    let mut order_rules: Vec<(u32, u32)> = vec![];
    let mut updates: Vec<Vec<u32>> = vec![];
    for line in lines {
        if line == "" {
            reading_order = false;
            continue;
        }
        if reading_order {
            order_rules.push(read_order(&line));
        } else {
            updates.push(read_pages(&line));
        }
    }
    return (order_rules, updates);
}

fn read_order(line: &String)-> (u32, u32) {
    let mut order_tuple: (u32, u32) = (0, 0);
    let mut idx = 0;
    for el in line.split("|") {
        let num: u32 = el.parse().unwrap();
        if idx == 0 {
            order_tuple.0 = num;
        } else {
            order_tuple.1 = num;
        }
        idx += 1;
        if idx >= 2 {
            break;
        }
    }
    return order_tuple;
}

fn read_pages(line: &String) -> Vec<u32>  {
    let mut pages = vec![];
    for el in line.split(",") {
        let num: u32 = el.parse().unwrap();
        pages.push(num);
    }
    return pages;
}

fn pages_in_order(page1: u32, page2: u32, order_rules: &Vec<(u32, u32)>) -> bool {
    for (first, last) in order_rules {
        if *first == page2 && *last == page1 {
            // println!("Bad {page1}, {page2}: {first}|{last}");
            return false;
        }
        if *first == page1 && *last == page2 {
            // println!("Good {page1}, {page2}: {first}|{last}");
            return true;
        }
    }
    // No rule matched the numbers; assume it's OK
    println!("No rule found for {page1}, {page2}");
    return true;
}

pub fn part1(lines: Vec<String>) {
    let (order_rules, updates) = read_rules_and_updates(lines);
    let mut sum_middle_pages = 0;
    for update in updates {
        let mut order_ok = true;
        'check_update:
        for i in 0..update.len() {
            for j in i+1..update.len() {
                let page1 = update[i];
                let page2 = update[j];
                if !pages_in_order(page1, page2, &order_rules) {
                    order_ok = false;
                    break 'check_update;
                }
            }
        }

        print!("[{}]", update.iter()
            .map(|n| n.to_string())
            .collect::<Vec<String>>()
            .join(", "));
        if order_ok {
            let middle_page = update[update.len() / 2];
            println!(" OK -- {middle_page}");
            sum_middle_pages += middle_page;
        } else {
            println!(" BAD");
        }
    }
    println!("Total: {sum_middle_pages}");
}

fn get_bad_updates(order_rules: &Vec<(u32, u32)>, updates: Vec<Vec<u32>>) -> Vec<Vec<u32>> {
    let mut bad_updates: Vec<Vec<u32>> = vec![];
    for update in updates {
        'check_update:
        for i in 0..update.len() {
            for j in i+1..update.len() {
                let page1 = update[i];
                let page2 = update[j];
                if !pages_in_order(page1, page2, &order_rules) {
                    bad_updates.push(update);
                    break 'check_update;
                }
            }
        }
    }
    return bad_updates;
}

fn fix_update(order_rules: &Vec<(u32, u32)>, update: &mut Vec<u32>) -> bool {
    let mut num_changes = 0;
    for _ in 0..100 {
        let mut mismatched = 0;
        for i in 0..update.len() {
            for j in i+1..update.len() {
                let page1 = update[i];
                let page2 = update[j];
                if !pages_in_order(page1, page2, &order_rules) {
                    print!("FIX: {update:?} -> ");
                    (update[i], update[j]) = (update[j], update[i]);
                    println!("{update:?}");
                    mismatched += 1;
                    num_changes += 1;
                }
            }
        }
        if mismatched == 0 {
            return true;
        }
    }
    print!("FAILED TO FIX: {update:?} (changes: {num_changes})");
    return false;
}

pub fn part2(lines: Vec<String>) {
    let (order_rules, updates) = read_rules_and_updates(lines);
    let mut bad_updates = get_bad_updates(&order_rules, updates);
    for mut update in &mut bad_updates {
        let fixed = fix_update(&order_rules, &mut update);
        if !fixed {
            return;
        }
    }
    let mut sum_middle_pages = 0;
    for update in bad_updates {
        sum_middle_pages += update[update.len() / 2];
    }
    println!("Total: {sum_middle_pages}");
}
