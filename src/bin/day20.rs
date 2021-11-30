use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

const TILE_SIZE: usize = 10;

enum Position {
    Top,
    Right,
    Bottom,
    Left,
}

fn get_offset(pos: Position) -> (isize, isize) {
    match pos {
        Position::Top => (0, -1),
        Position::Right => (1, 0),
        Position::Bottom => (0, 1),
        Position::Left => (0, -1),
    }
}

type Border = [bool; TILE_SIZE];

#[derive(Debug)]
struct Tile {
    id: u32,
    data: [Border; TILE_SIZE],
}

impl Tile {
    fn new(id: u32, data: [Border; TILE_SIZE]) -> Tile {
        Tile { id, data }
    }

    fn borders(&self) -> [Border; 4] {
        let mut border_left: Border = Border::default();
        let mut border_right: Border = Border::default();

        for i in 0..TILE_SIZE {
            border_left[i] = self.data[i][0];
            border_right[i] = self.data[i][TILE_SIZE - 1];
        }

        [
            self.data[0],
            border_right,
            self.data[TILE_SIZE - 1],
            border_left,
        ]
    }

    fn flip_horizontal(&mut self) {
        for i in 0..TILE_SIZE {
            for j in 0..TILE_SIZE / 2 {
                let temp = self.data[i][j];
                self.data[i][j] = self.data[i][TILE_SIZE - 1 - j];
                self.data[i][TILE_SIZE - 1 - j] = temp;
            }
        }
    }

    fn flip_vertical(&mut self) {
        for i in 0..TILE_SIZE / 2 {
            for j in 0..TILE_SIZE {
                let temp = self.data[i][j];
                self.data[i][j] = self.data[TILE_SIZE - 1 - i][j];
                self.data[TILE_SIZE - 1 - i][j] = temp;
            }
        }
    }

    fn rotate(&mut self, angle: u8) {
        let angle = ((angle % 4) + 4) % 4;

        match angle {
            0 => {}
            1 => {
                for i in 0..TILE_SIZE {
                    for j in i..TILE_SIZE - i - 1 {
                        let temp = self.data[i][j];
                        self.data[i][j] = self.data[TILE_SIZE - 1 - j][i];
                        self.data[TILE_SIZE - 1 - j][i] =
                            self.data[TILE_SIZE - 1 - i][TILE_SIZE - 1 - j];
                        self.data[TILE_SIZE - 1 - i][TILE_SIZE - 1 - j] =
                            self.data[j][TILE_SIZE - 1 - i];
                        self.data[j][TILE_SIZE - 1 - i] = temp;
                    }
                }
            }
            2 => {
                for i in 0..TILE_SIZE {
                    for j in i..TILE_SIZE - i - 1 {
                        let temp = self.data[i][j];
                        self.data[i][j] = self.data[TILE_SIZE - 1 - i][TILE_SIZE - 1 - j];
                        self.data[TILE_SIZE - 1 - i][TILE_SIZE - 1 - j] = temp;

                        let temp = self.data[j][TILE_SIZE - 1 - i];
                        self.data[j][TILE_SIZE - 1 - i] = self.data[TILE_SIZE - 1 - j][i];
                        self.data[TILE_SIZE - 1 - j][i] = temp;
                    }
                }
            }
            3 => {
                for i in 0..TILE_SIZE {
                    for j in i..TILE_SIZE - i - 1 {
                        let temp = self.data[i][j];
                        self.data[i][j] = self.data[j][TILE_SIZE - 1 - i];
                        self.data[j][TILE_SIZE - 1 - i] =
                            self.data[TILE_SIZE - 1 - i][TILE_SIZE - 1 - j];
                        self.data[TILE_SIZE - 1 - i][TILE_SIZE - 1 - j] =
                            self.data[TILE_SIZE - 1 - j][i];
                        self.data[TILE_SIZE - 1 - j][i] = temp;
                    }
                }
            }
            _ => {
                panic!("Invalid angle")
            }
        }
    }

    fn has_common_border_with(&self, other: &Tile) -> bool {
        for border in other.borders() {
            for self_border in self.borders() {
                let mut reversed_border: Border = Border::default();
                for i in 0..TILE_SIZE {
                    reversed_border[TILE_SIZE - 1 - i] = self_border[i];
                }

                if border == self_border || border == reversed_border {
                    return true;
                }
            }
        }
        return false;
    }

    fn touches(&self, other: &Tile) -> Option<Position> {
        let borders1 = self.borders();
        let borders2 = other.borders();

        if borders1[0] == borders2[2] {
            Some(Position::Top)
        } else if borders1[1] == borders2[3] {
            Some(Position::Right)
        } else if borders1[2] == borders2[0] {
            Some(Position::Bottom)
        } else if borders1[3] == borders2[1] {
            Some(Position::Left)
        } else {
            None
        }
    }
}

impl std::fmt::Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut map = String::with_capacity(TILE_SIZE * (TILE_SIZE + 1));
        for row in self.data {
            for square in row {
                map.push(match square {
                    true => '#',
                    false => '.',
                });
            }
            map.push('\n');
        }
        write!(f, "{}", map)
    }
}

fn load_input(filename: &str) -> Vec<Tile> {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut tiles: Vec<Tile> = Vec::new();

    let mut lines = reader.lines();

    let mut id: u32 = 0;
    let mut j = 0;
    let mut data: [Border; TILE_SIZE] = <_>::default();

    loop {
        if let Some(Ok(line)) = lines.next() {
            // println!("line: {}, j: {}, id: {}", line, j, id);
            if line.starts_with("Tile ") {
                id = (&line[5..=8]).parse().unwrap();
                j = 0;
            } else if j < TILE_SIZE {
                for (i, d) in (&line[0..TILE_SIZE])
                    .chars()
                    .map(|c| match c {
                        '#' => true,
                        _ => false,
                    })
                    .enumerate()
                {
                    data[j][i] = d;
                }
                j = j + 1;
            } else if id != 0 && j == TILE_SIZE {
                // println!("{} {:?}", id, data);
                // println!("Adding {}", id);
                let tile = Tile::new(id, data);
                tiles.push(tile);
                id = 0;
            } else {
                println!("Unknown line: {}", line);
            }
        } else {
            if id != 0 && j == TILE_SIZE {
                // println!("{} {:?}", id, data);
                // println!("Adding {}", id);
                let tile = Tile::new(id, data);
                tiles.push(tile);
                id = 0;
            }
            break;
        }
    }

    tiles
}

fn find_neighbors(tiles: &Vec<Tile>) -> HashMap<u32, HashSet<u32>> {
    let mut neighbors: HashMap<u32, HashSet<u32>> = HashMap::new();
    for tile in tiles {
        neighbors.insert(tile.id, HashSet::new());
    }

    for tile1 in tiles {
        for tile2 in tiles {
            if tile1.id != tile2.id && tile1.has_common_border_with(&tile2) {
                neighbors.get_mut(&tile1.id).unwrap().insert(tile2.id);
                neighbors.get_mut(&tile2.id).unwrap().insert(tile1.id);
            }
        }
    }

    neighbors
}

fn part1(filename: &str) -> u64 {
    let tiles = load_input(filename);
    let neighbors = find_neighbors(&tiles);

    let edge_product = neighbors
        .iter()
        .filter(|(_id, neighbor_set)| neighbor_set.len() == 2)
        // .for_each(|(id, ns)| println!("{}: {:?}", id, ns));
        .fold(1u64, |product, (id, _neighbor_set)| product * (*id as u64));
    edge_product
}

fn part2(filename: &str) -> usize {
    let tiles = load_input(filename);
    let neighbors = find_neighbors(&tiles);
    let mut tiles: HashMap<u32, &Tile> = HashMap::from_iter(tiles.iter().map(|t| (t.id, t)));
    let mut map: HashMap<(isize, isize), u32> = HashMap::new();
    let mut tile_positions: HashMap<u32, (isize, isize)> = HashMap::new();

    let mut unplaced_tiles: Vec<u32> = Vec::new();
    // Build a map with the tiles.

    unplaced_tiles.push(*(tiles.keys().next().unwrap()));
    while unplaced_tiles.len() > 0 {
        let id = unplaced_tiles.pop().unwrap();
        let tile = tiles.get_mut(&id).unwrap();

        println!("{:?}", tile);
        let pos = if map.len() == 0 {
            // If map is empty: place first tile at center.
            (0, 0)
        } else {
            // Find a position for the current tile
            panic!("Not implemented");
            (-1, -1)
        };
        map.insert(pos, id);
        tile_positions.insert(id, pos);

        for neighbor_id in neighbors.get(&id).unwrap() {
            if !tile_positions.contains_key(neighbor_id) {
                unplaced_tiles.push(*neighbor_id);
            }
        }
    }

    0
}

fn main() {
    let edge_product = part1("inputs/20.txt");
    println!("Part 1: {}", edge_product);

    let roughness = part2("inputs/20.txt");
    println!("Part 2: {}", roughness);
}

#[cfg(test)]
mod tests20 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/20_01.txt";
        let edge_product = part1(filename);

        assert_eq!(edge_product, 20899048083289);
    }

    #[test]
    fn test01_sol() {
        let filename = "inputs/20.txt";
        let edge_product = part1(filename);

        assert_eq!(edge_product, 5966506063747);
    }

    #[test]
    fn test02() {
        let filename = "test_inputs/20_01.txt";
        assert_eq!(part2(filename), 273);
    }
}
