// use std::io;
use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn main() -> std::io::Result<()> {
    part1()?;
    part2()?;

    Ok(())
}

fn part1() -> std::io::Result<()> {
    // let f = File::open("test_inputs/01_01.txt")?;
    let f = File::open("inputs/01.txt")?;
    let reader = BufReader::new(f);

    let mut v: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let i: u64 = line?.parse().unwrap();
        v.push(i);
    }

    let target_sum = 2020;

    // let mut product: u64 = 0;
    'outer: for i in 0..v.len() {
        let x = v[i];
        for j in i+1..v.len() {
            let y = v[j];
            if x + y == target_sum {
                println!("Found: {} + {} = {}", x, y, x + y);
                let product = x * y;
                println!("Product x*y = {}", product);
                break 'outer;
            }
        }
    }
    // assert_eq!(product, 514579);

    Ok(())
}

fn part2() -> std::io::Result<()> {
    // let f = File::open("test_inputs/01_01.txt")?;
    let f = File::open("inputs/01.txt")?;
    let reader = BufReader::new(f);

    let mut v: Vec<u64> = Vec::new();
    for line in reader.lines() {
        let i: u64 = line?.parse().unwrap();
        v.push(i);
    }

    let target_sum = 2020;

    // let mut product: u64 = 0;
    'outer: for i in 0..v.len() {
        let x = v[i];
        for j in i+1..v.len() {
            let y = v[j];
            for k in j+1..v.len() {
                let z = v[k];

                if x + y + z == target_sum {
                    println!("Found: {} + {} + {}= {}", x, y, z, x + y + z);
                    let product = x * y * z;
                    println!("Product x * y * z = {}", product);
                    break 'outer;
                }
            }
        }
    }
    // assert_eq!(product, 241861950);

    Ok(())
}
