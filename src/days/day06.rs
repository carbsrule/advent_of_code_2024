#[derive(Debug)]
struct Pos {
    row: i32,
    col: i32,
}

#[derive(Debug)]
struct Guard {
    pos: Pos,
    dir: char,
}

impl Guard {
    fn SetPos(&mut self, row: i32, col: i32) {
        self.pos.row = row;
        self.pos.col = col;
    }
    fn Move(&self, vert: i32, horiz: i32) -> Guard {
        let guard = Guard{
            pos: Pos {
                row: self.pos.row + vert,
                col: self.pos.col + horiz,
            },
            dir: self.dir,
        };
        return guard;
    }
    fn Turn(&mut self) {
        self.dir = match self.dir {
            '^' => '>',
            '>' => 'v',
            'v' => '<',
            '<' => '^',
            _ => panic!("Turn the beat around... but which way?"),
        };
    }
}

struct Tile {
    tile: char,
    visited: bool,
}

struct Map {
    grid: Vec<Vec<Tile>>,
    guard: Guard,
    visited: u32,
}

impl Map {
    fn tile(&self, pos: &Pos) -> char {
        return self.grid[pos.row as usize][pos.col as usize].tile;
    }
    fn visit(&mut self) {
        let pos = &self.guard.pos;
        let tile = &mut self.grid[pos.row as usize][pos.col as usize];
        if !tile.visited {
            tile.visited = true;
            self.visited += 1;
        }
    }
}

fn read_map(lines: Vec<String>) -> Map {
    let mut map = Map {
        grid: vec![],
        guard: Guard {
            pos: Pos {
                row: 0,
                col: 0,
            },
            dir: '\0',
        },
        visited: 0,
    };
    let mut row: usize = 0;
    for line in lines {
        map.grid.push(vec![]);
        let mut col = 0;
        for char in line.chars() {
            let mut tile = Tile {
                tile: char,
                visited: false,
            };
            if ['<', '^', '>', 'v'].contains(&char) {
                map.guard.SetPos(row as i32, col as i32);
                map.guard.dir = char;
                tile.visited = true;
                tile.tile = '.';
                map.visited += 1;
            }
            map.grid[row].push(tile);
            col += 1;
        }
        row += 1;
    }
    return map;
}

fn move_guard(mut map: Map) -> (Map, bool) {
    let before_guard = match map.guard.dir {
        '^' => map.guard.Move(-1, 0),
        '>' => map.guard.Move(0, 1),
        'v' => map.guard.Move(1, 0),
        '<' => map.guard.Move(0, -1),
        _ => panic!("What is guard? Baby don't hurt me"),
    };

    let cols = map.grid[0].len() as i32;
    let rows = map.grid.len() as i32;
    if before_guard.pos.col < 0 || before_guard.pos.col >= cols {
        print!("Exit horizontal from: {:?} ", map.guard.pos);
        map.guard = before_guard;
        return (map, false);
    }
    if before_guard.pos.row < 0 || before_guard.pos.row >= rows {
        print!("Exit vertical from: {:?} ", map.guard.pos);
        map.guard = before_guard;
        return (map, false);
    }

    print!("In front of guard: {:?} {}   ", before_guard.pos, map.tile(&before_guard.pos));
    if map.tile(&before_guard.pos) == '#' {
        // turn right
        map.guard.Turn();
        println!("Turn at {:?} to {}", map.guard.pos, map.guard.dir);
        return move_guard(map);
    }

    map.guard = before_guard;
    map.visit();

    println!("Move {} to ({:?})", map.guard.dir, map.guard.pos);

    return (map, true);
}

pub fn part1(lines: Vec<String>) {
    let mut map = read_map(lines);

    for row in &map.grid {
        println!("[{}]", row.iter()
            .map(|n| n.tile.to_string())
            .collect::<Vec<String>>()
            .join(""));
    }
    println!("{:?}", map.guard);

    let mut in_map;
    loop {
        (map, in_map) = move_guard(map);
        if !in_map {
            break;
        }
    }
    println!("Visited tiles: {}", map.visited);
}
