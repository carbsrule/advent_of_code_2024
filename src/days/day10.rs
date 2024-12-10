const UNREACHABLE_MARKER: char  = '.';
const UNREACHABLE_TILE: i8 = -10;
const SUMMIT: i8 = 9;

#[derive(Clone,Debug)]
struct Pos {
    row: i32,
    col: i32,
    path: Vec<Pos>,
}

impl Pos {
    fn new(row: i32, col: i32) -> Pos {
        return Pos {
            row,
            col,
            path: vec![],
        }
    }

    // Get a clone of a Pos, without its path history to avoid unnecessary recursion
    fn plain_clone(&self) -> Pos {
        return Pos::new(self.row, self.col);
    }

    // Add two points (i.e. a point and a distance vector)
    // Keep track of the path taken
    // Ensure that the resultant point is within bounds
    fn add(&self, dist: &Pos, bounds: &Pos) -> Option<Pos> {
        let mut res = Pos::new(
            self.row + dist.row,
            self.col + dist.col,
        );
        if res.row < 0 || res.col < 0 {
            return None;
        }
        if res.row >= bounds.row || res.col >= bounds.col {
            return None;
        }
        let mut path = self.path.clone();
        path.push(self.plain_clone());
        res.path = path;

        return Some(res);
    }

    fn format_path(&self) -> String {
        let mut steps = vec![];
        for el in &self.path {
            steps.push(format!("({}, {})", el.row, el.col));
        }
        return format!("[{}]", steps.join(", "));
    }

    fn coords(&self) -> String {
        return format!("({}, {})", self.row, self.col);
    }
}

struct Map {
    grid: Vec<Vec<i8>>,
    trailheads: Vec<Pos>,
}

impl Map {
    fn height(&self, pos: &Pos) -> i8 {
        return self.grid[pos.row as usize][pos.col as usize];
    }

    fn print(&self) {
        println!("Map: ");
        for row in &self.grid {
            for col in row {
                if *col == UNREACHABLE_TILE {
                    print!("{}", UNREACHABLE_MARKER);
                } else {
                    print!("{}", col);
                }
            }
            println!("");
        }

        println!("Trailheads (x{}): ", self.trailheads.len());
        for trailhead in &self.trailheads {
            println!("{}", trailhead.coords());
        }
    }

    fn reached_goal(&self, frontier: &Vec<Pos>) -> bool {
        for tile in frontier {
            if self.grid[tile.row as usize][tile.col as usize] != SUMMIT {
                return false;
            }
        }
        return true;
    }

    fn get_summit_paths(&self) -> Vec<Vec<Pos>> {
        let bounds = Pos::new(self.grid.len() as i32, self.grid[0].len() as i32);

        let directions = [
            Pos::new(-1, 0), // north
            Pos::new(0,1),   // east
            Pos::new(1, 0),  // south
            Pos::new(0, -1), // west
        ];

        // let mut scores = vec![];
        let mut summit_paths = vec![];
        for trailhead in &self.trailheads {
            let mut frontier: Vec<Pos> = vec![trailhead.clone()];
            while !self.reached_goal(&frontier) {
                let mut new_frontier = vec![];
                for tile in &frontier {
                    let current_height = self.height(tile);
                    for dir in &directions {
                        let destination = tile.add(&dir, &bounds);
                        match destination {
                            Some(dest_pos) => {
                                if self.height(&dest_pos) == current_height + 1
                                    && !pos_in_list(&dest_pos, &new_frontier)
                                {
                                    new_frontier.push(dest_pos);
                                }
                            },
                            None => (),
                        }
                    }
                }
                frontier = new_frontier;
            }
            summit_paths.push(frontier);
        }
        return summit_paths;
    }
}

fn pos_in_list(pos: &Pos, list: &Vec<Pos>) -> bool {
    for item in list {
        if item.row == pos.row && item.col == pos.col {
            // println!("Duplicate destination: {}", pos.coords());
            return true;
        }
    }
    return false;
}

fn read_map(lines: Vec<String>) -> Map {
    let mut map = Map {
        grid: vec![],
        trailheads: vec![],
    };

    let mut row = 0;
    for line in lines {
        map.grid.push(vec![]);
        let mut col = 0;
        for ch in line.chars() {
            let height = match ch {
                UNREACHABLE_MARKER => UNREACHABLE_TILE,
                _ => ch.to_digit(10).unwrap() as i8,
            };
            map.grid[row].push(height);
            if height == 0 {
                map.trailheads.push(Pos::new(row as i32, col));
            }
            col += 1;
        }
        row += 1;
    }
    return map;
}

pub fn part1(lines: Vec<String>) {
    let map = read_map(lines);
    println!("Read map\n");

    // map.print();

    let summit_paths = map.get_summit_paths();
    let mut total_scores = 0;
    for paths_from_trailhead in summit_paths {
        println!(
            "Trailhead {}:",
            paths_from_trailhead[0].path[0].coords(),
        );
        total_scores += paths_from_trailhead.len();
        for summit in paths_from_trailhead {
            println!("    {} via {}", summit.coords(), summit.format_path());
        }
    }
    println!("Sum of scores: {total_scores}");
}
