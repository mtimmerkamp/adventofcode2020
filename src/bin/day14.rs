use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;
use std::collections::HashMap;

struct Mask {
    and_mask: u64,
    or_mask: u64,
}

impl fmt::Debug for Mask {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Mask")
            .field("and", &format_args!("{:036b}", self.and_mask))
            .field("or", &format_args!("{:036b}", self.or_mask))
            .finish()
    }
}

impl Mask {
    const LEN: u32 = 36;

    fn new() -> Mask {
        Mask {and_mask: 0, or_mask: 0}
    }

    fn apply_to(&self, v: u64) -> u64 {
        v & self.and_mask | self.or_mask
    }
}

#[derive(Debug)]
enum MaskParseError {
    InvalidCharacter(char),
}

impl std::str::FromStr for Mask {
    type Err = MaskParseError;

    fn from_str(s: &str) -> Result<Mask, Self::Err> {
        let chars = s.chars();

        let mut and_mask = !0;
        let mut or_mask = 0;
        for (i, c) in chars.rev().enumerate() {
            match c {
                '1' => or_mask |= 1 << i,
                '0' => and_mask &= !(1 << i),
                'X' => {},
                _ => return Err(MaskParseError::InvalidCharacter(c)),
            }
        }

        Ok(Mask {and_mask, or_mask})
    }
}

#[derive(Debug)]
enum Step {
    ChangeMask(Mask),
    Write(u64, u64),
}

#[derive(Debug)]
enum StepParseError {
    Invalid,
    MissingEqualSign,
}

impl std::str::FromStr for Step {
    type Err = StepParseError;

    fn from_str(line: &str) -> Result<Step, Self::Err> {
        let mut parts = line.split(" = ");

        let left = parts.next().unwrap();
        let right = match parts.next() {
            Some(s) => s,
            None => return Err(StepParseError::MissingEqualSign),
        };

        if left.starts_with("mask") {
            let mask = right.parse().expect("Invalid mask");
            Ok(Step::ChangeMask(mask))
        }
        else if left.starts_with("mem") {
            let addr = left[3..]
                .trim_matches(|p| p == '[' || p == ']')
                .parse()
                .unwrap();
            let value = right.parse().unwrap();
            Ok(Step::Write(addr, value))
        }
        else {
            Err(StepParseError::Invalid)
        }
    }
}


fn load_steps(filename: &str) -> Vec<Step> {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut steps = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            let step = line.parse().expect("Falied to parse");
            steps.push(step);
        }
    }

    steps
}


fn part1(steps: &Vec<Step>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask = &Mask::new();
    for step in steps {
        match step {
            Step::ChangeMask(m) => {
                mask = m;
            },
            Step::Write(addr, value) => {
                memory.insert(*addr, mask.apply_to(*value));
            },
        }
    }

    memory.iter().map(|(_, value)| *value).sum()
}

fn iter_addrs(addr: u64, mask: &Mask) -> Vec<u64> {
    let addr = addr | mask.or_mask;

    let mut addrs = Vec::new();
    addrs.push(addr);

    // println!("{:?}", mask);
    for i in 0..=Mask::LEN - 1 {
        let is_floating = !(
            (mask.and_mask >> i) & 1 == 0
            || (mask.or_mask >> i) & 1 == 1);

        // println!("{:?}: {}", i, is_floating);

        if is_floating {
            let mut new_addrs = Vec::new();
            for addr in &addrs {
                new_addrs.push(*addr ^ (1 << i));
            }
            // println!("Adding {:?}", &new_addrs);

            addrs.extend(new_addrs);
        }
    }

    addrs
}

fn part2(steps: &Vec<Step>) -> u64 {
    let mut memory: HashMap<u64, u64> = HashMap::new();

    let mut mask = &Mask::new();
    for step in steps {
        match step {
            Step::ChangeMask(m) => {
                mask = m;
            },
            Step::Write(addr, value) => {
                let addrs = iter_addrs(*addr, mask);
                // println!("{:?}", addrs);
                for addr in addrs {
                    memory.insert(addr, *value);
                }
            },
        }
    }

    memory.iter().map(|(_, value)| *value).sum()
}


fn main() {
    // let filename = "test_inputs/14_01.txt";
    // let filename = "test_inputs/14_02.txt";
    let filename = "inputs/14.txt";
    let steps = load_steps(filename);
    println!("Part 1: {:?}", part1(&steps));
    println!("Part 2: {:?}", part2(&steps));
}

#[cfg(test)]
mod tests14 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/14_01.txt";
        let steps = load_steps(filename);
        assert_eq!(part1(&steps), 165);
    }

    #[test]
    fn test02() {
        let filename = "test_inputs/14_02.txt";
        let steps = load_steps(filename);
        assert_eq!(part2(&steps), 208);
    }
}
