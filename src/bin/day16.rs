use std::fs::File;
use std::io::{BufRead, BufReader};

struct Rule {
    name: String,
    ranges: Vec<(u32, u32)>,
}

impl std::str::FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");

        let name = match parts.next() {
            None => return Err(()),
            Some(name) => name,
        };
        
        Err(())
    }
}

fn load_input(filename: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {

}

fn main() {
    
}