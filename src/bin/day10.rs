use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;


fn load_adapters(filename: &str) -> Vec<i64> {
    let file = File::open(filename).expect("Cannot open file.");
    let reader = BufReader::new(file);

    let mut adapters: Vec<i64> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(joltage) = line.parse() {
                adapters.push(joltage);
            }
        }
    }

    adapters
}


fn part1(adapters: &Vec<i64>) -> u64 {
    let mut differences: [u64;3] = [0;3];

    let mut current = 0;
    for joltage in adapters {
        let difference = joltage - current;
        if difference >= 1 && difference <= 3 {
            current = *joltage;

            differences[(difference - 1) as usize] += 1;
        }
        else {
            panic!("Difference to next adapter it too high: {} - {} = {}",
                joltage, current, difference);
        }
    }

    // current += 3;  // Device's internal adapter.
    differences[2] += 1;

    for i in 0..differences.len() {
        println!("{:?} differences of {} joltage", differences[i], i + 1);
    }

    differences[0] * differences[2]
}


/// Counts the number of sub-paths given a list of adapters.
fn count_paths_away_from(adapters: &[i64]) -> u64 {
    fn count_paths_from(
        adapters: &[i64],
        cache: &mut HashMap<usize, u64>
    ) -> u64 {
        if adapters.len() == 1 {
            return 1
        }
        if cache.contains_key(&adapters.len()) {
            return cache[&adapters.len()];
        }

        let mut paths = 0;

        let start = adapters[0];
        for (i, joltage) in adapters[1..].iter().enumerate() {
            let difference = joltage - start;
            if difference <= 3 {
                paths += count_paths_from(&adapters[i + 1..], cache);
            }
            else {
                break;
            }
        }

        cache.insert(adapters.len(), paths);
        // println!("{:?} paths for {:?}", paths, adapters);

        paths
    }


    let mut cache = HashMap::new();
    count_paths_from(&adapters, &mut cache)
}


fn part2(adapters: &Vec<i64>) -> u64 {
    let mut adapters: Vec<i64> = adapters.clone();
    adapters.insert(0, 0);
    adapters.push(adapters.iter().max().unwrap() + 3);

    count_paths_away_from(&adapters)
}


fn main() {
    // let filename = "test_inputs/10_02.txt";
    let filename = "inputs/10.txt";
    let mut adapters = load_adapters(filename);

    adapters.sort();

    println!("Part 1: {}", part1(&adapters));
    println!("Part 2: {}", part2(&adapters));
}


#[cfg(test)]
mod tests10 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/10_01.txt";
        let mut adapters = load_adapters(filename);
        adapters.sort();
        assert_eq!(part1(&adapters), 35);
        assert_eq!(part2(&adapters), 8);
    }

    #[test]
    fn test02() {
        let filename = "test_inputs/10_02.txt";
        let mut adapters = load_adapters(filename);
        adapters.sort();
        assert_eq!(part1(&adapters), 22 * 10);
        assert_eq!(part2(&adapters), 19208);
    }
}