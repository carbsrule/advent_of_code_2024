use std::collections::HashMap;
use std::fmt;

#[derive(Clone, PartialEq)]
struct Point {
    row: i32,
    col: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = format!("({},{})", self.row, self.col);
        f.write_str(&fmt)
    }
}


impl Point {
    // Calculate distance between two points
    fn dist(&self, other: &Point) -> Point {
        let dist = Point {
            row: other.row - self.row,
            col: other.col - self.col,
        };
        // println!("Distance from {self:?} to {other:?}: {dist:?}");
        return dist;
    }

    // Add two points (i.e. a point and a distance vector)
    fn add(&self, dist: &Point) -> Point {
        let res = Point {
            row: self.row + dist.row,
            col: self.col + dist.col,
        };
        // println!("Add {dist:?} to {self:?} = {res:?}");
        return res;
    }

    // Subtract two points (i.e. a point and a distance vector)
    fn sub(&self, dist: &Point) -> Point {
        let res = Point {
            row: self.row - dist.row,
            col: self.col - dist.col,
        };
        // println!("Subtract {dist:?} from {self:?} = {res:?}");
        return res;
    }

    fn in_bounds(&self, rows: i32, cols: i32) -> bool {
        if self.col < 0 || self.col >= cols {
            return false;
        }
        if self.row < 0 || self.row >= rows {
            return false;
        }
        true
    }

    fn generate_antinodes(&self, other: &Point, rows: usize, cols: usize, single: bool) -> Vec<Point> {
        let rows = rows as i32;
        let cols = cols as i32;
        let dist = self.dist(other);
        let mut possibles: Vec<Point> = vec![];
        if single {
            possibles.append(&mut vec![self.sub(&dist), other.add(&dist)]);
            possibles.retain(|antinode| antinode.in_bounds(rows, cols));
        } else {
            possibles.push(self.clone());
            possibles.push(other.clone());
            let mut antinode = self.sub(&dist);
            while antinode.in_bounds(rows, cols) {
                possibles.push(antinode.clone());
                antinode = antinode.sub(&dist);
            }
            antinode = other.add(&dist);
            while antinode.in_bounds(rows, cols) {
                possibles.push(antinode.clone());
                antinode = antinode.add(&dist);
            }
        }
        return possibles;
    }
}

struct AntennaSet {
    members: HashMap<char, Vec<Point>>,
    rows: usize,
    cols: usize,
}

impl AntennaSet {
    fn generate_antinodes(&self, single: bool) -> Vec<Point> {
        let mut all_antinodes = vec![];
        for (_, positions) in self.members.iter() {
            if positions.len() == 0 {
                continue;
            }
            for i in 0..positions.len() {
                for j in i+1..positions.len() {
                    let antinodes_for_points = positions[i].generate_antinodes(
                        &positions[j],
                        self.rows,
                        self.cols,
                        single,
                    );
                    for antinode in antinodes_for_points {
                        if all_antinodes.contains(&antinode) {
                            continue;
                        }
                        all_antinodes.push(antinode);
                    }
                }
            }
        }
        println!("All antinodes: {all_antinodes:?}");
        return all_antinodes;
    }

    fn count_antinodes(&self, single: bool) -> usize {
        return self.generate_antinodes(single).len();
    }
}

fn get_antennas(lines: Vec<String>) -> AntennaSet {
    let mut row = 0;
    let mut antennas = AntennaSet {
        members: HashMap::new(),
        rows: lines.len(),
        cols: 0,
    };

    for line in lines {
        if row == 0 {
            antennas.cols = line.len();
        }
        let mut col = 0;
        for freq in line.chars() {
            if freq != '.' {
                let pos = Point {
                    row,
                    col,
                };
                if let Some(freq_positions) = antennas.members.get_mut(&freq) {
                    freq_positions.push(pos);
                } else {
                    antennas.members.insert(freq, vec![pos]);
                }
            }
            col += 1;
        }
        row += 1;
    }
    return antennas;
}

pub fn part1(lines: Vec<String>) {
    let antennas = get_antennas(lines);

    // println!("Antennas: {:?}", antennas.members);
    let num_antinodes = antennas.count_antinodes(true);
    println!("Antinodes: {num_antinodes}");
}

pub fn part2(lines: Vec<String>) {
    let antennas = get_antennas(lines);

    // println!("Antennas: {:?}", antennas.members);
    let num_antinodes = antennas.count_antinodes(false);
    println!("Antinodes: {num_antinodes}");
}
