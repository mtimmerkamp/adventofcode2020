use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;
use std::iter::FromIterator;

#[derive(Debug, Clone, PartialEq)]
enum TileState {
    Empty,
    Occupied,
    Floor,
}

#[derive(Debug, PartialEq)]
struct State(Vec<Vec<TileState>>);

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let state = &self.0;
        for (i, row) in state.iter().enumerate() {
            if i > 0 {
                f.write_str("\n")?;
            }
            let line = String::from_iter(row.iter().map(|t| match t {
                TileState::Empty => 'L',
                TileState::Occupied => '#',
                TileState::Floor => '.',
            }));
            f.write_str(&line)?;
        };
        Ok(())
    }
}

impl State {
    fn count_adjacent_occupied_seats(&self, row: usize, column: usize) -> u32 {
        let state = &self.0;
        let rows = state.len();
        if rows == 0 {
            panic!("State has not a single row.");
        }

        let columns = state[0].len();
        if row >= rows || column >= columns {
            panic!("Index out of range");
        }

        let mut count = 0;
        for &di in &[-1, 0, 1] {
            for &dj in &[-1, 0, 1] {
                if     di < 0 && row == 0
                    || di > 0 && row == rows - 1
                    || dj < 0 && column == 0
                    || dj > 0 && column == columns - 1
                    || dj == 0 && di == 0

                {
                    continue;
                }
                let i = (row as i32 + di) as usize;
                let j = (column as i32 + dj) as usize;
                let tile = &state[i][j];
                count += match tile {
                    TileState::Occupied => 1,
                    _ => 0,
                };
            }
        }

        count
    }

    fn count_visible_occupied_seats(&self, row: usize, column: usize) -> u32 {
        let state = &self.0;
        let rows = state.len();
        if rows == 0 {
            panic!("State has not a single row.");
        }

        let columns = state[0].len();
        if row >= rows || column >= columns {
            panic!("Index out of range");
        }

        // println!("Checking {} {}", row, column);

        let mut count = 0;
        for &di in &[-1, 0, 1] {
            for &dj in &[-1, 0, 1] {
                if di == 0 && dj == 0 {
                    continue;
                }

                // println!("  di, dj = {}, {}", di, dj);
                let mut i = row as i32 + di;
                let mut j = column as i32 + dj;
                while 0 <= i && i < rows as i32 && 0 <= j && j < columns as i32 {
                    // println!("    i, j = {:?}, {:?}", i, j);
                    let tile = &state[i as usize][j as usize];
                    match tile {
                        TileState::Occupied => {
                            // println!("      Occupied!");
                            count += 1;
                            break;
                        },
                        TileState::Empty => {
                            break;
                        }
                        TileState::Floor => {},
                    }

                    i += di;
                    j += dj;
                }
            }
        }

        count
    }

    fn count(&self, tile: TileState) -> usize {
        let state = &self.0;
        state.iter().flatten().filter(|t| **t == tile).count()
    }

    fn evolve(&self) -> State {
        let state = &self.0;
        let mut new: Vec<Vec<TileState>> = Vec::new();

        for (i, row) in state.iter().enumerate() {
            let mut new_row = Vec::new();
            for (j, tile) in row.iter().enumerate() {
                let new_tile = match tile {
                    TileState::Floor => TileState::Floor,
                    TileState::Empty => {
                        match self.count_adjacent_occupied_seats(i, j) {
                            0 => TileState::Occupied,
                            _ => TileState::Empty,
                        }
                    },
                    TileState::Occupied => {
                        match self.count_adjacent_occupied_seats(i, j) {
                            0..=3 => TileState::Occupied,
                            4..=u32::MAX => TileState::Empty,
                        }
                    },
                };
                new_row.push(new_tile);
            }
            new.push(new_row);
        }

        State(new)
    }

    fn evolve2(&self) -> State {
        let state = &self.0;
        let mut new: Vec<Vec<TileState>> = Vec::new();

        for (i, row) in state.iter().enumerate() {
            let mut new_row = Vec::new();
            for (j, tile) in row.iter().enumerate() {
                let new_tile = match tile {
                    TileState::Floor => TileState::Floor,
                    TileState::Empty => {
                        match self.count_visible_occupied_seats(i, j) {
                            0 => TileState::Occupied,
                            _ => TileState::Empty,
                        }
                    },
                    TileState::Occupied => {
                        match self.count_visible_occupied_seats(i, j) {
                            0..=4 => TileState::Occupied,
                            5..=u32::MAX => TileState::Empty,
                        }
                    },
                };
                new_row.push(new_tile);
            }
            new.push(new_row);
        }

        State(new)
    }
}


fn parse_line(line: &str) -> Option<Vec<TileState>> {
    let mut tiles = Vec::new();
    for c in line.chars() {
        let tile = match c {
            'L' => TileState::Empty,
            '.' => TileState::Floor,
            '#' => TileState::Occupied,
            _ => return None,
        };
        tiles.push(tile);
    }

    Some(tiles)
}

fn load_state(filename: &str) -> State {
    let mut state = Vec::new();

    let file = File::open(filename).expect("Cannot open file.");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            if let Some(tiles) = parse_line(&line) {
                state.push(tiles);
            }
            else {
                panic!("Cannot read line {}", line);
            }
        }
    }

    State(state)
}

fn part1(start_state: State) -> usize {
    let mut last = start_state;
    let mut current = last.evolve();

    // let mut iterations = 1;
    while last != current {
        // println!("\nState  {}:\n{}", iterations, current);
        last = current;
        current = last.evolve();
        // iterations += 1;
    }
    // println!("\nFinal state:\n{}", current);

    current.count(TileState::Occupied)
}

fn part2(start_state: State) -> usize {
    let mut last = start_state;
    let mut current = last.evolve2();

    // let mut iterations = 1;
    while last != current {
    // for _ in 0..=1 {
        // println!("\nState  {}:\n{}", iterations, current);
        last = current;
        current = last.evolve2();
        // iterations += 1;
    }
    // println!("\nFinal state:\n{}", current);

    current.count(TileState::Occupied)
}


fn main() {
    let filename = "inputs/11.txt";
    // let filename = "test_inputs/11_01.txt";

    let state = load_state(filename);
    println!("Part1: Occpuied seats: {}", part1(state));

    let state = load_state(filename);
    println!("Part2: Occpuied seats: {}", part2(state));
}


#[cfg(test)]
mod tests11 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/11_01.txt";
        let mut state = load_state(filename);

        for i in 0..=4 {
            state = state.evolve();

            let filename = format!("test_inputs/11_01_{:02}.txt", i);
            let expectation = load_state(&filename);

            assert_eq!(expectation, state);
        }
    }

    #[test]
    fn test02() {
        let filename = "test_inputs/11_01.txt";
        let state = load_state(filename);
        assert_eq!(part1(state), 37);
    }

    #[test]
    fn test03() {
        for &(filename, (row, col), occ_seats) in [
            ("02", (4, 3), 8),
            ("03", (1, 1), 0),
            ("04", (3, 3), 0),
        ].iter() {
            let state = load_state(&format!("test_inputs/11_{}.txt", filename));
            assert_eq!(state.count_visible_occupied_seats(row, col), occ_seats);

        }
    }

    #[test]
    fn test04() {
        let filename = "test_inputs/11_05_00.txt";
        let mut state = load_state(filename);

        for i in 1..=6 {
            state = state.evolve2();

            let filename = format!("test_inputs/11_05_{:02}.txt", i);
            let expectation = load_state(&filename);

            assert_eq!(expectation, state);
        }
    }
}