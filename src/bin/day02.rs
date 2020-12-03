use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Rule {
    min_count: i32,
    max_count: i32,
    character: char,
}

fn parse_line(line: &String) -> Result<(Rule, String), &'static str> {
    let parts: Vec<&str> = line.split(": ").collect();
    let (rule, password) = match parts[..] {
        [rule, password] => (rule, password),
        _ => return Err("Invalid format, expected RULE + \": \" PASSWORD"),
    };

    let rule_parts: Vec<&str> = rule.split(' ').collect();
    let (min_max, character) = match rule_parts[..] {
        [min_max, character] => (min_max, character),
        _ => return Err("Invalid format, expected MIN_MAX + \" \" CHARACTER"),
    };

    let min_max_parts: Vec<&str> = min_max.split('-').collect();
    if min_max_parts.len() != 2 {
        return Err("Invalid format, expected MIN + \"-\" + MAX");
    }
    let min_count: i32 = min_max_parts[0].parse().unwrap();
    let max_count: i32 = min_max_parts[1].parse().unwrap();



    let character: char = match character.chars().next() {
        Some(character) => character,
        _ => return Err("CHARACTER must have a length of at least one."),
    };
    Ok((
        Rule {character, max_count, min_count},
        String::from(password)
    ))
}

fn validate_password(password: String, rule: Rule) -> bool {
    let mut character_count = 0;
    for c in password.chars() {
        if c == rule.character {
            character_count += 1;
        }
    }
    character_count >= rule.min_count && character_count <= rule.max_count
}

fn count_correct_passwords(filename: &str) -> i32 {
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut correct_passwords = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (rule, password) = parse_line(&line).expect("Cannot convert line.");

        if validate_password(password, rule) {
            correct_passwords += 1;
        }
    }
    correct_passwords
}

fn validate_password2(password: String, rule: Rule) -> bool {
    if !password.contains(rule.character) {
        return false;
    }

    let char1 = password.chars().nth((rule.min_count - 1) as usize);
    let char2 = password.chars().nth((rule.max_count - 1) as usize);
    return (char1 == Some(rule.character)) ^ (char2 == Some(rule.character));
}

fn count_correct_passwords2(filename: &str) -> i32 {
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut correct_passwords = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (rule, password) = parse_line(&line).expect("Cannot convert line.");

        if validate_password2(password, rule) {
            correct_passwords += 1;
        }
    }
    correct_passwords
}

fn main() {
    let correct_passwords = count_correct_passwords("inputs/02.txt");
    println!("Part1: There are {} correct passwords.", correct_passwords);
    let correct_passwords = count_correct_passwords2("inputs/02.txt");
    println!("Part2: There are {} correct passwords.", correct_passwords);
}

#[cfg(test)]
mod tests02 {
    use super::*;

    #[test]
    fn test_count_correct_passwords() {
        let correct_passwords = count_correct_passwords("test_inputs/02_01.txt");
        assert_eq!(correct_passwords, 2);
    }

    #[test]
    fn test_count_correct_passwords2() {
        let correct_passwords = count_correct_passwords2("test_inputs/02_01.txt");
        assert_eq!(correct_passwords, 1);
    }
}