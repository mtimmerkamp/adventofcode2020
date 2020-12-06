use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;


type Groups = Vec<Vec<String>>;


fn read_groups(filename: &str) -> Groups {
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut groups: Groups = Vec::new();
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        let mut save_lines = false;

        if let Ok(line) = line {
            if line.len() > 0 {
                lines.push(line);
            }
            else {
                save_lines = true;
            }
        }

        if save_lines {
            groups.push(lines);
            lines = Vec::new();
        }
    }
    if lines.len() > 0 {
        groups.push(lines);
    }

    groups
}


fn count_positives(groups: &Groups) -> u32 {
    let mut total_count = 0;
    for group in groups {
        let chars: std::collections::HashSet<char> = group
            .iter()
            .map(|s| s.chars())
            .flatten()
            .collect();
        total_count += chars.len() as u32;
    }
    total_count
}

fn count_common_positives(groups: &Groups) -> u32 {
    let mut total_count = 0;
    for group in groups {
        let mut answer_counts = std::collections::HashMap::<char, u32>::new();
        for answer_set in group {
            let chars: std::collections::HashSet<char> = answer_set.chars().collect();
            for c in chars {
                match &answer_counts.get(&c) {
                    Some(&v) => answer_counts.insert(c, v + 1u32),
                    None => answer_counts.insert(c, 1),
                };
            }
        }

        for (&_, &count) in answer_counts.iter() {
            if count == (group.len() as u32) {
                total_count += 1;
            }
        }
    }
    total_count
}


fn main() {
    let filename = "inputs/06.txt";
    let groups = read_groups(filename);

    // println!("{:?}", groups);
    println!("Part 1: {:?}", count_positives(&groups));
    println!("Part 2: {:?}", count_common_positives(&groups));
}

#[cfg(test)]
mod tests06 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/06_01.txt";
        let groups = read_groups(filename);
        let positives = count_positives(&groups);

        assert_eq!(positives, 11);
    }

    #[test]
    fn test02() {
        let filename = "test_inputs/06_01.txt";
        let groups = read_groups(filename);
        let positives = count_common_positives(&groups);

        assert_eq!(positives, 6);
    }
}