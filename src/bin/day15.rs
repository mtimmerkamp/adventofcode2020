use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

fn load_start_numbers(filename: &str) -> Vec<u32> {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        if let Ok(line) = line {
            return line.split(',').map(|s| s.parse().unwrap()).collect()
        }
    }

    Vec::new()
}

fn part1(start_numbers: &Vec<u32>, end: usize) -> u32 {
    let mut numbers: HashMap<u32, usize> = HashMap::new();

    for (i, number) in start_numbers.iter().enumerate() {
        let entry = match numbers.get(number) {
            None => i,
            Some(_) => i,
        };
        numbers.insert(*number, entry);
        // println!("{}", number);
    }

    let mut last_number = start_numbers[start_numbers.len()-1];
    for i in start_numbers.len()-1..end {
        let entry = numbers.get(&last_number);
        let (entry, new_number) = match entry {
            None => (i, 0),
            Some(i0) => (i, (i - *i0) as u32),
        };
        numbers.insert(last_number, entry);

        last_number = new_number;
        // println!("{}", last_number);
    }

    last_number
}

fn main() {
    // let filename = "test_inputs/15_01.txt";
    let filename = "inputs/15.txt";
    let start_numbers = load_start_numbers(filename);

    println!("Part 1: 2020th number is {}", part1(&start_numbers, 2020 - 1));
    println!("Part 2: 30000000th number is {}", part1(&start_numbers, 30000000 - 1));
}

#[cfg(test)]
mod tests15 {
    use super::*;

    #[test]
    fn test01() {
        let results = [436, 1, 10, 27, 78, 438, 1836];

        for (i, result) in results.iter().enumerate() {
            let filename = format!("test_inputs/15_{:02}.txt", i + 1);
            let start_numbers = load_start_numbers(&filename);
            assert_eq!(part1(&start_numbers, 2020 - 1), *result);
        }
    }

    #[test]
    fn test01() {
        let results = [436, 1, 10, 27, 78, 438, 1836];

        for (i, result) in results.iter().enumerate() {
            let filename = format!("test_inputs/15_{:02}.txt", i + 1);
            let start_numbers = load_start_numbers(&filename);
            assert_eq!(part1(&start_numbers, 2020 - 1), *result);
        }
    }
}
