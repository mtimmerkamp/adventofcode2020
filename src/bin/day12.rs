use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turn(&self, a: &Action) -> Direction {
        match a {
            Action::Left(angle) => self.rotate(*angle),
            Action::Right(angle) => self.rotate(-angle),
            _ => self.clone(),
        }
    }

    fn rotate(&self, angle: i32) -> Direction {
        let rotations = angle.abs() / 90;
        let rotate_right = angle.is_negative();

        let mut direction = *self;
        for _ in 1..=rotations {
            if rotate_right {
                direction = direction.right();
            }
            else {
                direction = direction.left();
            }
        }

        direction
    }

    fn left(&self) -> Direction {
        match self {
            Self::North => Self::West,
            Self::South => Self::East,
            Self::West => Self::South,
            Self::East => Self::North,
        }
    }

    fn right(&self) -> Direction {
        match self {
            Self::North => Self::East,
            Self::South => Self::West,
            Self::West => Self::North,
            Self::East => Self::South,
        }
    }
}

#[derive(Debug)]
enum Action {
    North(i32),
    South(i32),
    West(i32),
    East(i32),
    Left(i32),
    Right(i32),
    Forward(i32),
}


#[derive(Debug)]
enum ActionErrorKind {
    TooShort,
    UnknownAction,
    InvalidDistance,
}

#[derive(Debug)]
struct ActionParseError{
    kind: ActionErrorKind,
}

impl std::str::FromStr for Action {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Action, ActionParseError> {
        if s.len() < 2 {
            return Err(ActionParseError {
                kind: ActionErrorKind::TooShort,
            })
        }
        let mut chars = s.chars();

        let action = match chars.next() {
            Some(a) => a,
            None => return Err(ActionParseError {
                kind: ActionErrorKind::UnknownAction,
            })
        };

        let argument = match chars.as_str().parse() {
            Ok(argument) => argument,
            Err(_) => return Err(ActionParseError {
                kind: ActionErrorKind::InvalidDistance,
            }),
        };

        match action {
            'N' => Ok(Action::North(argument)),
            'S' => Ok(Action::South(argument)),
            'E' => Ok(Action::East(argument)),
            'W' => Ok(Action::West(argument)),
            'L' => Ok(Action::Left(argument)),
            'R' => Ok(Action::Right(argument)),
            'F' => Ok(Action::Forward(argument)),
            _ => Err(ActionParseError {
                kind: ActionErrorKind::UnknownAction
            }),
        }
    }
}


#[derive(Debug)]
struct Ship {
    direction: Direction,
    position: (i32, i32),
}

impl Ship {
    fn new() -> Ship {
        Ship {
            direction: Direction::East,
            position: (0, 0),
        }
    }

    fn drive(&mut self, action: &Action) {
        let new_direction = self.direction.turn(&action);
        let (move_direction, distance) = match action {
            Action::North(d) => (Direction::North, *d),
            Action::South(d) => (Direction::South, *d),
            Action::West(d) => (Direction::West, *d),
            Action::East(d) => (Direction::East, *d),
            Action::Forward(d) => (self.direction, *d),
            _ => (new_direction, 0),
        };

        let (dx, dy) = match move_direction {
            Direction::North => (0, distance),
            Direction::South => (0, -distance),
            Direction::West => (-distance, 0),
            Direction::East => (distance, 0),
        };

        self.direction = new_direction;
        self.position = (self.position.0 + dx, self.position.1 + dy);
    }
}


fn load_instructions(filename: &str)
    -> Result<Vec<Action>, ActionParseError>
{
    let file = File::open(filename).expect("Cannot open file.");
    let reader = BufReader::new(file);

    let mut instructions: Vec<Action> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            match line.parse() {
                Ok(action) => instructions.push(action),
                Err(e) => return Err(e),
            }
        }
    }

    Ok(instructions)
}

fn part1(filename: &str) -> i32 {
    let instructions = load_instructions(filename)
        .expect("Cannot load instructions.");

    let mut ship = Ship::new();
    // println!("{:?}", ship);
    for action in instructions {
        ship.drive(&action);
        // println!("{:?} -> {:?}", action, ship);
    }

    ship.position.0.abs() + ship.position.1.abs()
}

#[derive(Debug)]
struct Ship2 {
    position: (i32, i32),
    waypoint: (i32, i32),
}

impl Ship2 {
    fn new() -> Ship2 {
        Ship2 {
            position: (0, 0),
            waypoint: (10, 1),
        }
    }

    fn rotate_waypoint(&mut self, angle: i32) {
        let rotations = angle.abs() / 90;
        let rotate_right = angle.is_negative();

        let mut waypoint = self.waypoint;
        for _ in 1..=rotations {
            if rotate_right {
                waypoint = (waypoint.1, -waypoint.0);
            }
            else {
                waypoint = (-waypoint.1, waypoint.0);
            }
        }
        self.waypoint = waypoint;
    }

    fn drive(&mut self, action: &Action) {
        match action {
            Action::North(d) => self.waypoint.1 += d,
            Action::South(d) => self.waypoint.1 -= d,
            Action::West(d) => self.waypoint.0 -= d,
            Action::East(d) => self.waypoint.0 += d,
            Action::Left(angle) => self.rotate_waypoint(*angle),
            Action::Right(angle) => self.rotate_waypoint(-angle),
            Action::Forward(times) => self.position = (
                self.position.0 + times * self.waypoint.0,
                self.position.1 + times * self.waypoint.1
            ),
        }
    }
}

fn part2(filename: &str) -> i32 {
    let instructions = load_instructions(filename)
        .expect("Cannot load instructions.");

    let mut ship = Ship2::new();
    // println!("{:?}", ship);
    for action in instructions {
        ship.drive(&action);
        // println!("{:?} -> {:?}", action, ship);
    }

    ship.position.0.abs() + ship.position.1.abs()
}

fn main() {
    // let filename = "test_inputs/12_01.txt";
    let filename = "inputs/12.txt";

    println!("Part1: Distance from starting position: {}", part1(filename));
    println!("Part2: Distance from starting position: {}", part2(filename));
}

#[cfg(test)]
mod tests12 {
    use super::*;

    #[test]
    fn test_direction() {
        let direction = Direction::North;
        assert_eq!(direction.left(), Direction::West);

        assert_eq!(direction.rotate(90), Direction::West);
        assert_eq!(direction.rotate(-90), Direction::East);
    }

    #[test]
    fn test01() {
        assert_eq!(part1("test_inputs/12_01.txt"), 25);
    }

    #[test]
    fn test02() {
        assert_eq!(part1("test_inputs/12_02.txt"), 0);
    }

    #[test]
    fn test03() {
        assert_eq!(part1("test_inputs/12_03.txt"), 0);
    }

    #[test]
    fn test04() {
        assert_eq!(part2("test_inputs/12_01.txt"), 286);
    }
}