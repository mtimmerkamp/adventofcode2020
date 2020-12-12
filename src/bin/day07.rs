use std::fs::File;
use std::io::{BufReader, BufRead};
use std::collections::{HashMap, HashSet};


type BagContents = Vec<(i32, String)>;


fn read_line(line: &str) ->
    Result<(String, BagContents), std::num::ParseIntError>
{
    // let len = line.len();
    let parts: Vec<&str> = line.split(' ').collect();
    let content_count = (parts.len() - 4) / 4;

    let super_color = format!("{} {}", parts[0], parts[1]);
    // if content_count == 0 {
    //     return Ok((super_color, Vec::new()));
    // }

    let mut bags: Vec<(i32, String)> = Vec::new();
    for i in 1..=content_count  {
        let j = i * 4;

        let number: i32 = match parts[j + 0] {
            "no" => return Ok((super_color, Vec::new())),
            number => match number.parse() {
                Ok(number) => number,
                Err(e) => return Err(e),
            },
        };

        let color = format!("{} {}", parts[j + 1], parts[j + 2]);
        bags.push((number, color));
    }

    Ok((super_color, bags))
}


fn read_bags(filename: &str) -> HashMap<String, BagContents> {
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut map: HashMap<String, BagContents> = HashMap::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            match read_line(&line) {
                Ok((bag, contents)) => {
                    // println!("{:?}: {:?}", &bag, &contents);
                    map.insert(bag, contents);
                },
                Err(e) => panic!("{:?}", e),
            }
        }
    }

    map
}


fn find_containing_bags(map: &HashMap<String, BagContents>, color: &str)
 -> HashSet<String> {
    let mut containers: HashSet<String> = HashSet::new();

    for super_color in map.keys() {
        let contents = &map[super_color];
        for (_, bag_color) in contents {
            if bag_color == color {
                containers.insert(String::from(super_color));
                for container in find_containing_bags(map, super_color) {
                    containers.insert(container);
                }
            }
        }
    }

    containers
 }


fn count_containing_bags(map: &HashMap<String, BagContents>, color: &str)
    -> usize
{
    let containers = find_containing_bags(map, color);
    // println!("{:?}", containers);
    containers.len()
}


fn count_contained_bags(map: &HashMap<String, BagContents>, color: &str)
    -> usize
{
    let bags = &map[color];

    let mut count: usize = 0;
    for (bag_count, bag_color) in bags {
        let bag_count = *bag_count as usize;
        count += bag_count;
        count += bag_count * count_contained_bags(map, &bag_color);
    }

    count
}


fn main() {
    let filename = "inputs/07.txt";
    let map = read_bags(filename);

    println!("Part1: {:?}", count_containing_bags(&map, "shiny gold"));
    println!("Part2: {:?}", count_contained_bags(&map, "shiny gold"));
}

#[cfg(test)]
mod tests07 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/07_01.txt";
        let map = read_bags(filename);
        let container_count = count_containing_bags(&map, "shiny gold");
        assert_eq!(container_count, 4);
    }

    #[test]
    fn test02() {
        let filename = "test_inputs/07_01.txt";
        let map = read_bags(filename);
        let container_count = count_contained_bags(&map, "shiny gold");
        assert_eq!(container_count, 32);
    }

    #[test]
    fn test03() {
        let filename = "test_inputs/07_02.txt";
        let map = read_bags(filename);
        let container_count = count_contained_bags(&map, "shiny gold");
        assert_eq!(container_count, 126);
    }
}
