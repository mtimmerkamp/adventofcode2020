use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;
use std::fmt;
use std::collections::{HashMap, HashSet};

#[macro_use]
extern crate lazy_static;


#[derive(Debug)]
struct Passport{
    birth_year: i32,
    issue_year: i32,
    expiration_year: i32,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: Option<String>,
}

struct ParsePassportError {
    kind: PassportErrorKind,
}

impl fmt::Display for ParsePassportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            PassportErrorKind::MalformedField =>
                "Malformed field data.",
            PassportErrorKind::MissingRequiredField => "Passport is missing a field.",
        }.fmt(f)
    }
}

enum PassportErrorKind {
    MalformedField,
    MissingRequiredField,
}

impl FromStr for Passport {
    type Err = ParsePassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fields = HashMap::new();
        for piece in s.split(' ') {
            let mut iter = piece.split(':');

            let id = iter.next();
            let value = iter.next();

            match (id, value) {
                (Some(id), Some(value)) => fields.insert(id, value),
                (Some(_), None) =>
                    return Err(ParsePassportError {kind: PassportErrorKind::MalformedField}),
                _ => continue,
            };
        }

        if let (
            Some(birth_year),
            Some(issue_year),
            Some(expiration_year),
            Some(height),
            Some(hair_color),
            Some(eye_color),
            Some(passport_id),
            country_id,  // nobody would mind to handle county id as optional...
        ) = (
            fields.remove("byr").map(|s| s.parse().ok()).flatten(),
            fields.remove("iyr").map(|s| s.parse().ok()).flatten(),
            fields.remove("eyr").map(|s| s.parse().ok()).flatten(),
            fields.remove("hgt").map(String::from),
            fields.remove("hcl").map(String::from),
            fields.remove("ecl").map(String::from),
            fields.remove("pid").map(String::from),
            fields.remove("cid").map(String::from),
        ) {
            Ok(Passport {
                birth_year, issue_year, expiration_year, height, hair_color,
                eye_color, passport_id, country_id
            })
        }
        else {
            Err(ParsePassportError { kind: PassportErrorKind::MissingRequiredField })
        }
    }
}


fn load_passports(filename: &str) -> Vec<Passport> {
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut passports: Vec<Passport> = Vec::new();
    let mut passport_data = String::new();

    for line in reader.lines() {
        let line = line.expect("Cannot read line.");

        if line.len() == 0 {
            // println!("PassportData data: {}", passport_data);
            match passport_data.parse() {
                Ok(passport) => passports.push(passport),
                Err(e) => println!("Error on \"{}\": {}", passport_data, e),
            }
            passport_data = String::new();
        }
        else {
            if passport_data.len() > 0
            {
                passport_data.push(' ');
            }
            passport_data.push_str(&line);
        }
    }
    if passport_data.len() > 0 {
        match passport_data.parse() {
            Ok(passport) => passports.push(passport),
            Err(e) => println!("Error on \"{}\": {}", passport_data, e),
        }
    }

    passports
}


fn part1(passports: &Vec<Passport>) -> i32 {
    passports.len() as i32
}

lazy_static! {
    static ref ALLOWED_EYE_COLORS: HashSet<&'static str> = [
        "amb", "blu", "brn", "gry", "grn", "hzl", "oth"
    ].iter().cloned().collect();
}

fn is_valid_hair_color(hcl: &str) -> bool {
    let mut chars = hcl.chars();
    chars.next() == Some('#')
        && chars.all(|c| ('a'..='f').contains(&c) || ('0'..='9').contains(&c))
}

fn is_valid_height(hgt: &str) -> bool {
    let len = hgt.len();

    if len > 2 && (hgt.ends_with("cm") || hgt.ends_with("in")) {
        if let Ok(height) = hgt[0..len-2].parse() {
            if hgt.ends_with("cm") {
                150 <= height && height <= 193
            }
            else {
                59 <= height && height <= 76
            }
        }
        else {
            false
        }
    }
    else {
        false
    }
}

fn is_valid_passport2(p: &Passport) -> bool {
    1920 <= p.birth_year && p.birth_year <= 2002
    && 2010 <= p.issue_year && p.issue_year <= 2020
    && 2020 <= p.expiration_year && p.expiration_year <= 2030
    && is_valid_height(&p.height)
    && is_valid_hair_color(&p.hair_color)
    && ALLOWED_EYE_COLORS.contains(p.eye_color.as_str())
    && p.passport_id.len() == 9 && p.passport_id.parse::<i32>().is_ok()
}


fn part2(passports: &Vec<Passport>) -> i32 {
    return passports
        .iter()
        .filter(|p| is_valid_passport2(p))
        .count() as i32;
}


fn main() {
    let filename = "inputs/04.txt";
    let passports = load_passports(filename);

    // println!("{:#?}", &passports);
    println!("Part1: {} valid passports!", part1(&passports));
    println!("Part2: {} valid passports!", part2(&passports));
}


#[cfg(test)]
mod tests04 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/04_01.txt";
        let passports = load_passports(filename);

        assert_eq!(part1(&passports), 2);
    }

    #[test]
    fn test02_invalid() {
        let filename = "test_inputs/04_02_invalid.txt";
        let passports = load_passports(filename);

        assert_eq!(part2(&passports), 0);
    }

    #[test]
    fn test02_valid() {
        let filename = "test_inputs/04_02_valid.txt";
        let passports = load_passports(filename);

        assert_eq!(part2(&passports), 4);
    }
}