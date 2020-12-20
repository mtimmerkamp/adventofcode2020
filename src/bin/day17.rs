use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;
use std::collections::HashSet;

#[derive(Debug)]
enum Tile {
    Active, Inactive,
}

#[derive(Debug)]
enum TileParseError {
    InvalidCharacter(char),
}

impl Tile {
    fn from_char(c: char) -> Result<Self, TileParseError> {
        match c {
            '.' => Ok(Tile::Inactive),
            '#' => Ok(Tile::Active),
            _ => Err(TileParseError::InvalidCharacter(c)),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Active => '#',
            Tile::Inactive => '.',
        }
    }
}

fn increment(c: Coord, v: i32) -> Coord {
    let mut c = c.clone();
    for i in 0..c.len() {
        c[i] += v
    }
    c
}

struct World {
    active_tiles: HashSet<Coord>,
}

impl World {
    fn new() -> World {
        World {
            active_tiles: HashSet::new(),
        }
    }

    fn set(&mut self, coord: Coord, tile: Tile) {
        match tile {
            Tile::Active => {
                self.active_tiles.insert(coord)
            },
            Tile::Inactive => {
                self.active_tiles.remove(&coord)
            },
        };
    }

    fn get(&self, coord: Coord) -> Tile {
        if self.active_tiles.contains(&coord) {
            Tile::Active
        }
        else {
            Tile::Inactive
        }
    }

    fn from_lines(lines: &Vec<String>) -> World {
        let mut world = World::new();

        let z = 0;
        for (y, line) in lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let tile = Tile::from_char(c).unwrap();
                let mut coord = Coord::default();
                coord[0] = x as i32;
                coord[1] = y as i32;
                world.set(coord, tile);
            }
        }

        world
    }

    fn bounding_rect(&self) -> (Coord, Coord) {
        let mut min = Coord::default();
        let mut max = Coord::default();

        for coord in &self.active_tiles {
            for i in 0..min.len() {
                min[i] = min[i].min(coord[i]);
                max[i] = max[i].max(coord[i]);
            }
        }

        (min, max)
    }

    fn count_neighbors(&self, coord: Coord) -> u32 {
        let mut neighbors = 0;

        let start = increment(coord.clone(), -1);
        let end = increment(coord.clone(), 1);

        for neighbor_coord in iter_coords(start, end) {
            if coord == neighbor_coord {
                continue;
            }

            match self.get(neighbor_coord) {
                Tile::Active => neighbors += 1,
                _ => {},
            }
        }

        neighbors
    }

    fn evolve(&self) -> World {
        let (min, max) = self.bounding_rect();
        let min = increment(min.clone(), -1);
        let max = increment(max.clone(), 1);

        let mut new_active_tiles = HashSet::new();
        for coord in iter_coords(min, max) {
            let neighbors = self.count_neighbors(coord);

            let new_state = match self.get(coord) {
                Tile::Active => {
                    if neighbors == 2 || neighbors == 3 {
                        Tile::Active
                    } else {Tile::Inactive}
                },
                Tile::Inactive => {
                    if neighbors == 3 {Tile::Active} else {Tile::Inactive}
                }
            };

            match new_state {
                Tile::Active => {
                    new_active_tiles.insert(coord);
                },
                Tile::Inactive => {},
            }
        }

        World {
            active_tiles: new_active_tiles,
        }
    }

    fn count_active_tiles(&self) -> u32 {
        self.active_tiles.len() as u32
    }
}

struct CoordIter {
    start: Coord,
    end: Coord,
    last: Option<Coord>,
}

impl Iterator for CoordIter {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        match self.last {
            None => {
                let current = self.start.clone();
                self.last = Some(current);
                return Some(current.clone());
            },
            Some(mut current) => {
                let mut correct = false;
                for i in (0..current.len()).rev() {
                    current[i] += 1;
                    if current[i] <= self.end[i] {
                        correct = true;
                        break;
                    } else {
                        current[i] = self.start[i];
                    }
                }
                if !correct {
                    return None;
                }

                self.last = Some(current);
                return Some(current.clone());
            }
        };
    }
}

fn iter_coords(start: Coord, end: Coord) -> CoordIter {
    CoordIter {
        start, end, last: None,
    }
}

fn iter_coord_along(start: Coord, end: Coord, axis: usize) -> CoordIter {
    let mut new_end = start.clone();
    new_end[axis] = end[axis];

    CoordIter {
        start, end: new_end, last: None,
    }
}

impl World {
    fn format_dim(
        &self, f: &mut fmt::Formatter<'_>, prefix: &String,
        (min, max): (Coord, Coord), axis: usize
    ) -> fmt::Result {
        if axis == 0 {
            panic!("Format for axis 0 is not implemented.");
        }
        else if axis == 1 {
            f.write_fmt(format_args!("{}\n", &prefix))?;
            for y in iter_coord_along(min, max, 1) {
                let mut chars = Vec::new();
                for x in iter_coord_along(y, max, 0) {
                    chars.push(self.get(x).to_char());
                }
                f.write_str(&chars.iter().collect::<String>())?;
                f.write_str("\n")?;
            }
            f.write_str("\n")?;
        }
        else {
            for xi in iter_coord_along(min, max, axis) {
                let prefix = match prefix.len() {
                    0 => format!("x{{{}}} = {}", axis, xi[axis]),
                    _ => format!("{}, x{{{}}} = {}", prefix, axis, xi[axis]),
                };
                self.format_dim(f, &prefix, (xi, max), axis - 1)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for World {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = self.bounding_rect();

        self.format_dim(f, &String::new(), (min, max), min.len()-1)
    }
}

fn load_world(filename: &str) -> World {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    World::from_lines(&lines)
}

fn part1(world: &World, rounds: u32, display: bool) -> u32 {
    let start_world = world;

    let mut world: World = World::new();
    let mut first_evolution = true;

    if display {
        println!("Before and cycles:");
        println!("{}", start_world);
    }
    for cycle in 1..=rounds {
        if first_evolution {
            world = start_world.evolve();
            first_evolution = false;
        }
        else {
            world = world.evolve();
        }
        if display {
            println!("After {} cycle(s):", cycle);
            println!("{}", world);
        }

    }

    world.count_active_tiles()
}


type Coord = [i32; 3];  // for part 1
// type Coord = [i32; 4];  // for part 2

fn main() {
    // let filename = "test_inputs/17_01.txt";
    let filename = "inputs/17.txt";
    let world = load_world(filename);

    println!("Part 1: Active tiles: {}", part1(&world, 6, false));
    // For part 2, change Coord above.
}

#[cfg(test)]
mod tests17 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/17_01.txt";
        let world = load_world(filename);
        assert_eq!(part1(&world, 6, false), 112);
    }
}