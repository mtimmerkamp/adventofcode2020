use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::convert::TryFrom;
use std::collections::HashSet;


#[derive(Debug)]
struct SeatRange {
    min_row: u32,
    max_row: u32,
    min_col: u32,
    max_col: u32,
}

#[derive(Debug)]
struct SeatRoutingError {
    direction: Direction,
}


impl SeatRange {
    fn new(width: u32, height: u32) -> SeatRange{
        SeatRange {
            min_row: 0, max_row: height - 1,
            min_col: 0, max_col: width - 1,
        }
    }

    fn front(&self) -> Result<SeatRange, SeatRoutingError> {
        if self.min_row == self.max_row {
            Err(SeatRoutingError {direction: Direction::FRONT})
        }
        else {
            Ok(SeatRange {
                max_row: self.min_row + (self.max_row - self.min_row + 1) / 2 - 1,
                ..*self
            })
        }
    }
    fn back(&self) -> Result<SeatRange, SeatRoutingError> {
        if self.min_row == self.max_row {
            Err(SeatRoutingError {direction: Direction::BACK})
        }
        else {
            Ok(SeatRange {
                min_row: self.min_row + (self.max_row - self.min_row + 1) / 2,
                ..*self
            })
        }
    }
    fn left(&self) -> Result<SeatRange, SeatRoutingError> {
        if self.min_col == self.max_col {
            Err(SeatRoutingError {direction: Direction::LEFT})
        }
        else {
            Ok(SeatRange {
                max_col: self.min_col + (self.max_col - self.min_col + 1) / 2 - 1,
                ..*self
            })
        }
    }
    fn right(&self) -> Result<SeatRange, SeatRoutingError> {
        if self.min_col == self.max_col {
            Err(SeatRoutingError {direction: Direction::RIGHT})
        }
        else {
            Ok(SeatRange {
                min_col: self.min_col + (self.max_col - self.min_col + 1) / 2,
                ..*self
            })
        }
    }

    fn is_seat(&self) -> bool {
        self.min_row == self.max_row && self.min_col == self.max_col
    }
    fn seat(&self) -> Result<Seat, Error> {
        if self.is_seat() {
            Ok(Seat {row: self.min_row, col: self.min_col})
        }
        else {
            Err(Error("Cannot convert range to seat."))
        }
    }
}

#[derive(Debug)]
struct Seat {
    row: u32,
    col: u32,
}

impl Seat {
    fn id(&self) -> u32 {
        self.row * 8 + self.col
    }
}


#[derive(Debug)]
enum Direction {
    FRONT, BACK, LEFT, RIGHT
}


type BoardingPass = [Direction; 10];

fn parse_boarding_pass(s: &str) -> Result<BoardingPass, Error> {
    let mut pass: Vec<Direction> = Vec::new();
    for c in s.chars() {
        match c {
            'F' => pass.push(Direction::FRONT),
            'B' => pass.push(Direction::BACK),
            'L' => pass.push(Direction::LEFT),
            'R' => pass.push(Direction::RIGHT),
            _ => return Err(Error("Invalid letter"))
        }
    }
    if let Ok(pass) = BoardingPass::try_from(pass) {
        Ok(pass)
    }
    else {
        Err(Error("Cannot convert to boarding pass."))
    }
}


#[derive(Debug)]
struct Error (&'static str);


fn load_boading_passes(filename: &str) -> Result<Vec<BoardingPass>, Error> {
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut passes: Vec<BoardingPass> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(pass) = parse_boarding_pass(&line) {
                passes.push(pass);
            }
            else {
                return Err(Error("Cannot parse boarding pass."));
            }
        }
    }

    Ok(passes)
}


fn find_seat(pass: &BoardingPass) -> Option<Seat> {
    let mut seat_range = SeatRange::new(8, 128);

    for direction in pass.iter() {
        // println!("{:?}", seat_range);

        let result = match direction {
            Direction::FRONT => seat_range.front(),
            Direction::BACK => seat_range.back(),
            Direction::LEFT => seat_range.left(),
            Direction::RIGHT => seat_range.right(),
        };

        seat_range = match result {
            Ok(s) => s,
            Err(e) => panic!(e),
        }
    }

    if let Ok(seat) = seat_range.seat() {
        Some(seat)
    }
    else {
        None
    }
}


fn main() {
    let filename = "inputs/05.txt";
    let boarding_passes = load_boading_passes(filename).unwrap();

    // println!("{:?}", boarding_passes);
    let mut max_id = 0;
    let mut ids: HashSet<u32> = std::iter::successors(
        Some(0u32), |&i| if i < 127*8 + 8 {Some(i + 1)} else {None}).collect();
    // let mut seats: Vec<Seat> = Vec::new();
    for pass in boarding_passes {
        match find_seat(&pass) {
            Some(seat) => {
                let id = seat.id();
                max_id = std::cmp::max(max_id, id);
                // println!("{:?}, id: {}", seat, id);
                ids.remove(&id);
                // seats.push(seat);
            },
            None => println!("Cannot find seat for {:?}", &pass),
        }
    }

    println!("Part1: Max seat ID: {}", max_id);

    // println!("Part2: missing seat IDs: {:?}", ids);
    for id in &ids {
        if !ids.contains(&(id + 1)) && !ids.contains(&(id - 1)) {
            println!("Part2: My seat ID: {:?}", id);
        }
    }
}


#[cfg(test)]
mod tests05 {
    use super::*;

    #[test]
    fn test01() {
        let passes = [
            "FBFBBFFRLR", "BFFFBBFRRR", "FFFBBBFRRR", "BBFFBBFRLL"
        ];
        let row_cols: [(u32, u32); 4] = [
            (44, 5), (70, 7), (14, 7), (102, 4)
        ];

        for (pass, (row, col)) in passes.iter().zip(row_cols.iter()) {
            let pass = parse_boarding_pass(pass).unwrap();
            let seat = find_seat(&pass).unwrap();

            assert_eq!((seat.row, seat.col), (*row, *col));
        }
    }
}
