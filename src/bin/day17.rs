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

type Coord = (i32, i32, i32);

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
                world.set((x as i32, y as i32, z), tile);
            }
        }

        world
    }

    fn bounding_rect(&self) -> (Coord, Coord) {
        let mut min = (0, 0, 0);
        let mut max = (0, 0, 0);

        for coord in &self.active_tiles {
            min.0 = i32::min(min.0, coord.0);
            min.1 = i32::min(min.1, coord.1);
            min.2 = i32::min(min.2, coord.2);
            max.0 = i32::max(max.0, coord.0);
            max.1 = i32::max(max.1, coord.1);
            max.2 = i32::max(max.2, coord.2);
        }

        (min, max)
    }

    fn count_neighbors(&self, coord: Coord) -> u32 {
        let mut neighbors = 0;

        let start = (coord.0 - 1, coord.1 - 1, coord.2 - 1);
        let end = (coord.0 + 1, coord.1 + 1, coord.2 + 1);

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
        let min = (min.0 - 1, min.1 - 1, min.2 - 1);
        let max = (max.0 + 1, max.1 + 1, max.2 + 1);

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
                // Calculate next coord.
                current.2 += 1;
                if current.2 > self.end.2 {
                    current.2 = self.start.2;
                    current.1 += 1;
                    if current.1 > self.end.1 {
                        current.1 = self.start.1;
                        current.0 += 1;
                        if current.0 > self.end.0 {
                            return None;
                        }
                    }
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

impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (min, max) = self.bounding_rect();
        for z in min.2..=max.2 {
            f.write_fmt(format_args!("z = {}\n", z))?;
            for y in min.1..=max.1 {
                let mut chars = Vec::new();
                for x in min.0..=max.0 {
                    chars.push(self.get((x, y, z)).to_char());
                }
                f.write_str(&chars.iter().collect::<String>())?;
                f.write_str("\n")?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

fn load_world(filename: &str) -> World {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    World::from_lines(&lines)
}

fn part1(world: &World, display: bool) -> u32 {
    let start_world = world;

    let mut world: World = World::new();
    let mut first_evolution = true;

    if display {
        println!("Before and cycles:");
        println!("{}", start_world);
    }
    for cycle in 1..=6 {
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

fn main() {
    // let filename = "test_inputs/17_01.txt";
    let filename = "inputs/17.txt";
    let world = load_world(filename);

    println!("Part 1: Active tiles: {}", part1(&world, false));
}

#[cfg(test)]
mod tests17 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/17_01.txt";
        let world = load_world(filename);
        assert_eq!(part1(&world, false), 112);
    }
}