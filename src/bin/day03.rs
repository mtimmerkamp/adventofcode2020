use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::iter::FromIterator;
use std::fmt;

#[derive(Debug)]
enum Tile {
    Empty,
    Tree,
}

impl Tile {
    fn from(s: char) -> Tile {
        match s {
            '.' => Tile::Empty,
            '#' => Tile::Tree,
            _ => panic!("Unknown tile {}", s)
        }
    }
}

impl fmt::Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Tile::Empty => '.',
            Tile::Tree => 'X',
        };
        write!(f, "{}", s)
    }
}


fn read_lines(filename: &str) -> Vec<Vec<Tile>> {
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut lines: Vec<Vec<Tile>> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            lines.push(Vec::from_iter(line.chars().map(Tile::from)));
        }
    }

    lines
}


fn count_trees(lines: &Vec<Vec<Tile>>, slope: (i32, i32)) -> i32 {
    let slope_x = slope.0 as usize;
    let slope_y = slope.1 as usize;
    let line_length = lines[0].len();

    let mut trees = 0;

    let mut x = 0;
    let mut y = 0;
    while y < lines.len() {
        match lines[y][x % line_length] {
            Tile::Tree => trees += 1,
            _ => {},
        }

        x += slope_x;
        y += slope_y;
    }

    trees
}

fn part1(lines: &Vec<Vec<Tile>>) -> i32 {
    count_trees(&lines, (3, 1))
}

fn part2(lines: &Vec<Vec<Tile>>) -> i64 {
    let slopes = [
        (1, 1), (3, 1), (5, 1), (7, 1), (1, 2)
    ];

    let mut result: i64 = 1;
    for slope in slopes.iter() {
        result *= count_trees(&lines, *slope) as i64;
    }

    result
}


fn main() {
    let filename = "inputs/03.txt";
    let lines = read_lines(filename);

    let trees = part1(&lines);
    println!("Part1: Tree count {}", trees);

    let trees_product = part2(&lines);
    println!("Part2: Product of tree counts: {}", trees_product);
}


#[cfg(test)]
mod tests03 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/03_01.txt";
        let lines = read_lines(filename);

        assert_eq!(part1(&lines), 7);
    }

    #[test]
    fn test02() {
        let filename = "test_inputs/03_01.txt";
        let lines = read_lines(filename);

        assert_eq!(part2(&lines), 336);
    }
}

