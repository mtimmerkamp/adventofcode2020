use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


fn check_validity(numbers: &Vec<u64>, preamble_length: usize) -> Option<usize> {
    'outer: for (i, &n) in numbers[preamble_length..].iter().enumerate() {
        // check all pairs
        for (j, &a) in numbers[i..i+preamble_length].iter().enumerate() {
            for (k, &b) in numbers[i..i+preamble_length].iter().enumerate() {
                if k == j {continue}
                if a + b == n {
                    continue 'outer;
                }
            }
        }
        return Some(i + preamble_length);
    }
    None
}


// fn find_continguous_set(numbers: &Vec<u64>, )


fn read_numbers(filename: &str) -> Vec<u64> {
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut numbers: Vec<u64> = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            if let Ok(n) = line.parse() {
                numbers.push(n);
            }
        }
    }

    numbers
}


fn main() {
    let filename = "inputs/09.txt";
    let numbers = read_numbers(filename);

    if let Some(i) = check_validity(&numbers, 25) {
        println!("Part1: Valid until {}: {}", i, numbers[i]);
    }

}

#[cfg(test)]
mod tests09 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/09_01.txt";
        let numbers = read_numbers(filename);
        let i = check_validity(&numbers, 5);
        assert_eq!(i, Some(14));
        if let Some(i) = i {
            assert_eq!(numbers[i], 127);
        }
    }
}
