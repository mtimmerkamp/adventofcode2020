use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
enum Rule {
    NonTerminal(Vec<Vec<u32>>),
    Terminal(char)
}

impl Rule {
    fn from(s: &str) -> Result<(u32, Rule), ()> {
        let mut parts = s.split(": ");

        let id: u32 = parts.next().unwrap().parse().unwrap();
        let rule = parts.next().unwrap();

        let rule = if rule.contains("\"") {
            let rule = rule.trim_matches('"');
            Rule::Terminal(rule.chars().next().unwrap())
        }
        else {
            let mut options: Vec<Vec<u32>> = Vec::new();
            let option_strs = rule.split(" | ");
            for option in option_strs {
                let ids: Vec<u32> = option.split(' ')
                    .map(|s| s.parse().unwrap())
                    .collect();
                options.push(ids);
            }
            Rule::NonTerminal(options)
        };

        Ok((id, rule))
    }
}

fn load_input(filename: &str) -> (HashMap<u32, Rule>, Vec<String>) {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    enum State {
        Rules, Messages,
    }

    let mut messages = Vec::new();
    let mut rules = HashMap::new();

    let mut state = State::Rules;
    for line in reader.lines() {
        if let Ok(line) = line {
            if line.len() == 0 {
                state = State::Messages;
                continue;
            }

            match state {
                State::Rules => {
                    let (rule_id, rule) = Rule::from(&line).expect("Invalid rule");
                    rules.insert(rule_id, rule);
                },
                State::Messages => {
                    messages.push(line);
                },
            }
        }
    }

    (rules, messages)
}

fn _validate<'a>(s: &'a str, rules: &HashMap<u32, Rule>, start: u32, level: usize) -> (bool, &'a str) {
    let rule = &rules[&start];

    let (valid, remainder) = match rule {
        Rule::Terminal(expectation) => {
            let mut chars = s.chars();

            if let Some(c) = chars.next() {
                (*expectation == c, chars.as_str())
            } else {
                (false, chars.as_str())
            }
        },
        Rule::NonTerminal(options) => {
            let mut chars = s.chars();

            let mut any_valid = false;

            // Check if any option matches.
            for option in options.iter() {
                let mut option_valid = true;
                chars = s.chars();

                // An option matches if all rules inside match.
                for id in option {
                    let (valid, remainder) = _validate(
                        chars.as_str(), rules, *id, level + 1);
                    option_valid &= valid;
                    if valid {
                        chars = remainder.chars();
                    } else {
                        break;
                    }

                }

                if option_valid {
                    any_valid = true;
                    break;
                }
            }

            if any_valid {
                (any_valid, chars.as_str())
            } else {
                (any_valid, s)
            }

        }
    };

    (valid, remainder)
}

fn validate(s: &str, rules: &HashMap<u32, Rule>) -> bool {
    let (valid, remainder) = _validate(s, rules, 0, 0);
    if remainder.len() == 0 {
        valid
    } else {
        false
    }
}

fn part1(messages: &Vec<String>, rules: &HashMap<u32, Rule>) -> usize {
    messages.iter().filter(|m| validate(m, rules)).count()
}

fn _validate2<'a>(s: &'a str, rules: &HashMap<u32, Rule>, start: u32, level: usize) -> (bool, &'a str) {
    // special handing for the starting rule 0 to handle recursive rules 8
    // and 11, which used solely by rule 0.
    if start == 0 {
        let s_orig = s;
        // handle rules:  0: 8 11;  8: 42 | 42 8;  11: 42 31 | 42 11 31
        // expect n + m times rule 42 and m times rule 31.
        let mut s;

        // Test all possible combinations of n and m such that the input is
        // completely consumed.
        let mut m;
        let mut n = 0;  // n must be at least 1.
        'n: loop {
            n += 1;

            m = 0;
            'm: loop {
                m += 1; // m must be at least 1.

                let mut valid = true;
                s = s_orig;

                // n + m times rule 42
                for i in 1..=(n + m) {
                    let (v, r) = _validate2(s,  rules, 42, level + 1);
                    valid &= v;
                    if v {
                        s = r;
                    } else if i <= n {
                        // Failure, even though we did not find rule 42 n
                        // times. => n is too large. Since we checked all
                        // smaller n, this input must be invalid.
                        break 'n;
                    } else {
                        // We failed to find rule 42 n + m times (e.g., the
                        // input might already be consumed). => m is too large.
                        // Increment n and try again.
                        continue 'n;
                    }
                }

                // m times rule 31
                for _ in 1..=m {
                    let (v, r) = _validate2(s,  rules, 31, level + 1);
                    valid &= v;
                    if v {
                        s = r;
                    } else if s.len() == 0 {
                        // We failed and the input is consumed. => m is too
                        // large. Increment n and try again.
                        break 'm;
                    } else {
                        // We failed but the input is not yet consumed.
                        // => m might be too small. Increment m and try again.
                        continue 'm;
                    }
                }

                if valid && s.len() == 0 {
                    return (true, s);
                }
            }
        }

        return (false, s_orig);
    }

    // Rule 0 is not used by any other rule, so we can use the validator of
    // part 1 for all other checks.
    let (valid, remainder) = _validate(s, rules, start, level);

    (valid, remainder)
}

fn validate2(s: &str, rules: &HashMap<u32, Rule>) -> bool {
    let (valid, remainder) = _validate2(s, rules, 0, 0);

    if remainder.len() == 0 {
        // println!("valid:   {}", s);
        valid
    } else {
        // println!("invalid: {}", s);
        false
    }
}

fn part2(messages: &Vec<String>, rules: &HashMap<u32, Rule>) -> usize {
    messages.iter().filter(|m| validate2(m, rules)).count()
}

fn main() {
    // let filename = "test_inputs/19_01.txt";
    // let filename = "test_inputs/19_02.txt";
    let filename = "inputs/19.txt";
    let (rules, messages) = load_input(filename);

    println!("Part 1: {} rules valid", part1(&messages, &rules));

    // transform_rules_for_part2(&mut rules);
    println!("Part 2: {} rules valid", part2(&messages, &rules));
}