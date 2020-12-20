use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Rule {
    name: String,
    ranges: Vec<(u32, u32)>,
}

impl std::str::FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(": ");

        let name = String::from(parts.next().unwrap());

        let mut ranges = Vec::new();
        let ranges_parts = parts.next().unwrap().split(" or ");
        for part in ranges_parts {
            let boundaries: Vec<u32> = part.split('-')
                .map(|s| s.parse().unwrap())
                .collect();

            if boundaries.len() != 2 {
                return Err(());
            }

            ranges.push((boundaries[0], boundaries[1]));
        }


        Ok(Rule{
            name,
            ranges,
        })
    }
}

impl Rule {
    fn check(&self, v: &u32) -> bool {
        let mut valid = false;
        for (a, b) in &self.ranges {
            valid |= *a <= *v && *v <= *b;
        }

        valid
    }
}

type Ticket = Vec<u32>;

fn load_input(filename: &str) -> (Vec<Rule>, Ticket, Vec<Ticket>) {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    enum Section {
        Rules = 0, MyTicket, OtherTickets,
    };

    let mut rules = Vec::new();
    let mut my_ticket = Ticket::new();
    let mut other_tickets = Vec::new();

    let mut state = Section::Rules;
    let mut skip_next_line = false;
    for line in reader.lines() {
        if let Ok(line) = line {
            // An empty line starts a new section
            if line.len() == 0 {
                state = match state {
                    Section::Rules => Section::MyTicket,
                    Section::MyTicket => Section::OtherTickets,
                    Section::OtherTickets => break,
                };
                skip_next_line = true;
                continue;
            }

            // Skip the first lines of "my ticket" and "nearby tickets"
            if skip_next_line {
                skip_next_line = false;
                continue;
            }

            match state {
                Section::Rules => {
                    rules.push(line.parse().unwrap())
                },
                Section::MyTicket => {
                    my_ticket = line.split(',').map(|s| s.parse().unwrap()).collect();
                },
                Section::OtherTickets => {
                    let ticket: Ticket = line.split(',').map(|s| s.parse().unwrap()).collect();
                    if ticket.len() != my_ticket.len() {
                        panic!("Tickets do not have the same length!")
                    }
                    other_tickets.push(ticket);
                }
            };
        }
    }

    (rules, my_ticket, other_tickets)
}

fn part1(rules: &Vec<Rule>, tickets: &Vec<Ticket>) -> u32 {
    let mut ticket_scanning_error_rate: u32 = 0;

    for ticket in tickets {
        // println!("Ticket: {:?}", tickets);
        for field in ticket {
            let may_be_valid = rules.iter().any(|rule| rule.check(field));
            // println!("  {:?} is valid: {:?}", field, may_be_valid);
            if !may_be_valid {
                ticket_scanning_error_rate += field;
            }
        }
    }

    ticket_scanning_error_rate
}

fn part2(rules: &Vec<Rule>, my_ticket: &Ticket, tickets: &Vec<Ticket>) -> u64 {
    // Remove tickets that are definetly invalid.
    let tickets = tickets.iter().filter(|ticket| {
        // A ticket is valid if all fields are valid.
        ticket.iter().all(|field| {
            // A field might be valid if it satisfies any rule.
            rules.iter().any(|rule| rule.check(field))
        })
    });

    let mut field_rules_map: HashMap<usize, HashSet<usize>> = HashMap::new();
    for i in 0..my_ticket.len() {
        let mut set = HashSet::new();
        for j in 0..rules.len() {
            set.insert(j);
        }
        field_rules_map.insert(i, set);
    }

    for ticket in tickets {
        for (i, field) in ticket.iter().enumerate() {
            for (j, rule) in rules.iter().enumerate() {
                if !rule.check(field) {
                    if let Some(set) = field_rules_map.get_mut(&i) {
                        set.remove(&j);
                    }
                }
            }
        }
    }

    let mut field_map: HashMap<usize, usize> = HashMap::new();

    let mut changed = true;
    while changed {
        changed = false;

        let mut field_no: usize = 0;
        let mut rule_no: usize = 0;

        for (i, rule_set) in &field_rules_map {
            if !field_map.contains_key(i) && rule_set.len() == 1 {
                field_no = *i;
                rule_no = *(rule_set.iter().nth(0).unwrap());
                field_map.insert(*i, rule_no);
                changed = true;
                break;
            }
        }

        if changed {
            for (i, rule_set) in field_rules_map.iter_mut() {
                if *i == field_no || field_map.contains_key(i) {
                    continue;
                }

                rule_set.remove(&rule_no);
            }
        }
    }

    let mut result: u64 = 1;
    for (field_idx, rule_idx) in field_map.iter() {
        let name = &rules[*rule_idx].name;
        let value = my_ticket[*field_idx];
        // println!("{}: {}", name, value);

        if name.starts_with("departure") {
            result *= value as u64;
        }
    }
    result
}

fn main() {
    // let filename = "test_inputs/16_01.txt";
    // let filename = "test_inputs/16_02.txt";
    let filename = "inputs/16.txt";
    let (rules, my_ticket, other_tickets) = load_input(filename);

    // println!("{:?}, {:?}, {:?}", rules, my_ticket, other_tickets);
    println!("Part 1: {}", part1(&rules, &other_tickets));
    println!("Part 2: {}", part2(&rules, &my_ticket, &other_tickets));
}

#[cfg(test)]
mod tests16 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/16_01.txt";
        let (rules, _, other_tickets) = load_input(filename);
        assert_eq!(part1(&rules, &other_tickets), 71)
    }
}