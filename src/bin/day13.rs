use std::fs::File;
use std::io::{BufRead, BufReader};


fn load_input(filename: &str) -> (i64, Vec<i64>) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    let start: i64 = lines.next().unwrap().unwrap().parse().unwrap();

    let busses: Vec<i64> = lines.next()
        .unwrap().unwrap()
        .split(',')
        .map(|s| s.parse())
        .filter(|r| r.is_ok())
        .map(|r| r.unwrap())
        .collect();

    (start, busses)
}


fn part1(start: i64, busses: &Vec<i64>) -> i64 {
    let mut next_bus: i64 = 0;
    let mut next_bus_departure: i64 = 0;
    let mut first = true;

    for bus in busses {
        let departures = start / bus;
        let next_departure = (departures + 1) * bus;

        if first || next_departure < next_bus_departure {
            next_bus = *bus;
            next_bus_departure = next_departure;
            first = false;
        }
    }

    next_bus * (next_bus_departure - start)
}

fn eek(n1: i64, n2: i64) -> (i64, i64, i64) {
    let mut a = 0;
    let mut b = n1;
    let mut u = 0;
    let mut s = 1;
    let mut v = 1;
    let mut t = 0;
    // println!("{}, {}, {}, {}, {}, {}, {}", a, b, 0, u, s, v, t);

    let mut first = true;
    while b != 0 {
        let q = a / b;
        let r = a % b;

        a = b;
        if first {
            b = n2;
        }
        else {
            b = r;
        }
        first = false;

        let u_new = s;
        let v_new = t;

        s = u - q * s;
        t = v - q * t;

        u = u_new;
        v = v_new;

        // println!("{}, {}, {}, {}, {}, {}, {}", a, b, q, u, s, v, t);
    }
    println!("{} * {} + {} * {} = {}", u, n1, v, n2, a);

    (a, u, v)
}

fn load_input2(filename: &str) -> Vec<(i64, i64)> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();
    lines.next();

    let busses: Vec<(i64, i64)> = lines.next()
        .unwrap().unwrap()
        .split(',')
        .enumerate()
        .map(|(i, s)| (i as i64, s.parse()))
        .filter(|(_, r)| r.is_ok())
        .map(|(i, r)| (i, r.unwrap()))
        .collect();

    busses
}

fn part2(busses: &Vec<(i64, i64)>) -> i64 {
    // use the "Chinesischer Restsatz"
    let m_ = busses.iter().fold(1i64, |prod, (_, bus)| prod * *bus);
    println!("M: {}", m_);
    let mut x: i64 = busses.iter().map(|(i, bus)| {
        let mi_ = m_ / *bus;
        let (_ggt, _ri, si) = eek(*bus, mi_);
        let ei = si * mi_;
        let ai = *bus - i;
        println!("Mi: {}", mi_);

        ai * ei
    }).sum();

    if x < 0 {
        x += m_ * (x.abs() / m_ + 1);
    }
    x - m_ * (x / m_)
}


fn main() {
    // let filename = "test_inputs/13_01.txt";
    let filename = "inputs/13.txt";
    let (start, busses) = load_input(filename);

    println!("{:?}, {:?}", start, busses);
    println!("Part1: {:?}", part1(start, &busses));

    let busses = load_input2(filename);
    println!("Part2: {:?}", part2(&busses));
}

#[cfg(test)]
mod tests13 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/13_01.txt";
        let (start, busses) = load_input(filename);
        assert_eq!(part1(start, &busses), 295);
    }

    #[test]
    fn test02() {
        let results: [i64; 6] = [1068781, 3417, 754018, 779210, 1261476, 1202161486];
        for i in 0..=5 {
            let filename = format!("test_inputs/13_{:02}.txt", i + 1);
            let busses = load_input2(&filename);
            let x = part2(&busses);
            assert_eq!(x, results[i]);
        }
    }
}