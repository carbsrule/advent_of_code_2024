fn has_match(grid: &Vec<Vec<char>>, start_row: usize, start_col: usize, right_move: i32, down_move: i32) -> i32 {
    let mut row = start_row;
    let mut col = start_col;

    let expected = ['M', 'A', 'S'];

    for i in 0..3 {
        row = (row as i32 + down_move) as usize;
        col = (col as i32 + right_move) as usize;
        if grid[row][col] != expected[i] {
            return 0;
        }
    }
    return 1;
}

fn count_in_grid(grid: &Vec<Vec<char>>) -> i32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let last_right = num_cols - 4;
    let last_down = num_rows - 4;
    let mut total = 0;
    for row in 0..num_rows {
        for col in 0..num_cols {
            if grid[row][col] != 'X' {
                continue;
            }
            if col >= 3 {
                total += has_match(grid, row, col, -1, 0); // left
                if row <= last_down {
                    total += has_match(grid, row, col, -1, 1); // left-down
                }
                if row >= 3 {
                    total += has_match(grid, row, col, -1, -1); // left-up
                }
            }
            if col <= last_right {
                total += has_match(grid, row, col, 1, 0); // right
                if row <= last_down {
                    total += has_match(grid, row, col, 1, 1); // right-down
                }
                if row >= 3 {
                    total += has_match(grid, row, col, 1, -1); // right-up
                }
            }
            if row >= 3 {
                total += has_match(grid, row, col, 0, -1); // up
            }
            if row <= last_down {
                total += has_match(grid, row, col, 0, 1); // down
            }
        }
    }
    return total;
}

pub fn part1(lines: Vec<String>) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut row = 0;
    for line in lines {
        grid.push(vec![]);
        for char in line.chars() {
            grid[row].push(char);
        }
        row += 1;
    }
    let count = count_in_grid(&grid);
    println!("Total: {count}");
}

fn has_mas_cross(grid: &Vec<Vec<char>>, row: usize, col: usize) -> i32 {
    let down_right = vec![grid[row][col], grid[row+1][col+1], grid[row+2][col+2]];
    let down_left = vec![grid[row][col+2], grid[row+1][col+1], grid[row+2][col]];

    let mut matches = 0;
    let expect = [['M', 'A', 'S'], ['S', 'A', 'M']];
    for match_diagonal in [down_right, down_left] {
        for option in expect {
            let mut all_match = true;
            let mut idx = 0;
            for expected_char in option {
                if match_diagonal[idx] != expected_char {
                    all_match = false;
                    break;
                }
                idx += 1;
            }
            if all_match {
                matches += 1;
                break;
            }
        }
    }

    // Need both directions (top-left to bottom-right, and top-right to bottom-left) to have found a MAS (or SAM)
    return if matches == 2 {
        1
    } else {
        0
    };
}

fn count_mas_crosses_in_grid(grid: &Vec<Vec<char>>) -> i32 {
    let num_rows = grid.len();
    let num_cols = grid[0].len();
    let last_right = num_cols - 2;
    let last_down = num_rows - 2;
    let mut total = 0;

    for row in 0..last_down {
        for col in 0..last_right {
            total += has_mas_cross(&grid, row, col);
        }
    }
    return total;
}

pub fn part2(lines: Vec<String>) {
    let mut grid: Vec<Vec<char>> = vec![];
    let mut row = 0;
    for line in lines {
        grid.push(vec![]);
        for char in line.chars() {
            grid[row].push(char);
        }
        row += 1;
    }
    let count = count_mas_crosses_in_grid(&grid);
    println!("Total: {count}");
}
