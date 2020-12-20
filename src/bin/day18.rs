use std::fs::File;
use std::io::{BufRead, BufReader};
use std::fmt;


#[derive(Debug)]
struct Tape {
    chars: Vec<char>,
    current: usize,
}
#[derive(Debug)]
struct TapeState(usize);

impl Tape {
    fn new(chars: Vec<char>) -> Tape {
        Tape {
            chars, current: 0,
        }
    }
    fn from(s: &str) -> Tape {
        Tape::new(s.chars().collect())
    }

    fn peek(&self) -> Option<&char> {
        self.chars.get(self.current)
    }
    fn consume(&mut self) -> Option<&char> {
        let c = self.chars.get(self.current);
        self.current += 1;
        c
    }

    fn store(&self) -> TapeState {
        TapeState(self.current)
    }
    fn restore(&mut self, state: &TapeState) {
        self.current = state.0;
    }
}

#[derive(Debug)]
enum Expr {
    Number(i64),
    Addition(Box<Expr>, Box<Expr>),
    Multiplication(Box<Expr>, Box<Expr>),
    Parenthesis(Box<Expr>),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Number(n) =>
                f.write_fmt(format_args!("{}", n)),
            Expr::Parenthesis(e) =>
                f.write_fmt(format_args!("[{}]", e)),
            Expr::Addition(e1, e2) =>
                f.write_fmt(format_args!("({} + {})", e1, e2)),
            Expr::Multiplication(e1, e2) =>
                f.write_fmt(format_args!("({} * {})", e1, e2)),
        }
    }
}

#[derive(Debug)]
enum ExprParseError {
    Invalid,
    InvalidCharacter(char),
    MissingOpeningParenthesis,
    MissingClosingParenthesis,
}

type ExprResult = Result<Expr, ExprParseError>;

fn parse_parenthesis(tape: &mut Tape) -> ExprResult {
    let state = tape.store();

    // require "("
    match tape.consume() {
        Some(&'(') => {},
        _ => {
            tape.restore(&state);
            return Err(ExprParseError::MissingOpeningParenthesis);
        },
    }

    match parse_expression(tape) {
        Ok(expr) => {
            // require ")"
            match tape.consume() {
                Some(&')') => return Ok(
                    Expr::Parenthesis(Box::new(expr))
                ),
                _ => {
                    tape.restore(&state);
                    return Err(ExprParseError::MissingClosingParenthesis)
                }
            }
        },
        Err(e) => {
            tape.restore(&state);
            return Err(e);
        }
    }
}

fn parse_number(tape: &mut Tape) -> ExprResult {
    let state = tape.store();
    let mut digits: Vec<char> = Vec::new();

    loop {
        let local_state = tape.store();
        let c = tape.consume();
        match c {
            Some(&c @ '0'..='9') => digits.push(c),
            _ => {
                tape.restore(&local_state);
                break;
            },
        }
    }

    if digits.len() == 0 {
        tape.restore(&state);
        Err(ExprParseError::InvalidCharacter(*tape.peek().unwrap()))
    }
    else {
        let number: String = digits.iter().collect();
        let number = number.parse().unwrap();
        Ok(Expr::Number(number))
    }
}

fn parse_parenthesis_or_number(tape: &mut Tape) -> ExprResult {
    match parse_parenthesis(tape) {
        Ok(expression) => Ok(expression),
        Err(_) => {
            match parse_number(tape) {
                Ok(number) => Ok(number),
                Err(e2) => return Err(e2),
            }
        },
    }
}

fn parse_expression(tape: &mut Tape) -> ExprResult {
    let state = tape.store();
    let mut expressions: Vec<Expr> = Vec::new();
    // let mut last_error;

    match parse_parenthesis_or_number(tape) {
        Ok(expr) => expressions.push(expr),
        Err(e) => {
            tape.restore(&state);
            return Err(e)
        },
    }

    enum Operation {
        Add, Mul,
    }

    loop {
        let state2 = tape.store();
        let operation = match tape.consume() {
            Some(&'+') => Operation::Add,
            Some(&'*') => Operation::Mul,
            Some(_) => {
                tape.restore(&state2);
                break;
            },
            None => break,
        };

        match parse_parenthesis_or_number(tape) {
            Ok(expr) => expressions.push(expr),
            Err(e) => {
                tape.restore(&state);
                return Err(e);
            },
        }

        let right = expressions.pop().unwrap();
        let left = expressions.pop().unwrap();
        let expr = match operation {
            Operation::Add => Expr::Addition(
                Box::new(left), Box::new(right)
            ),
            Operation::Mul => Expr::Multiplication(
                Box::new(left), Box::new(right)
            )
        };
        expressions.push(expr);
    }

    match expressions.pop() {
        Some(expr) => Ok(expr),
        None => Err(ExprParseError::Invalid),
    }
}

fn parse(s: &str) -> ExprResult {
    let s: String = s.chars().filter(|&c| c != ' ').collect();
    let mut tape = Tape::from(&s);

    parse_expression(&mut tape)
}


fn load_input(filename: &str) -> Vec<Expr> {
    let file = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut expressions = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            match parse(&line) {
                Ok(expr) => {
                    expressions.push(expr);
                },
                Err(e) => panic!("Cannot parse expression. {:?}", e),
            }
        }
    }

    expressions
}

fn eval(expr: &Expr) -> i64 {
    match expr {
        Expr::Number(n) => *n,
        Expr::Parenthesis(e) => eval(e),
        Expr::Addition(e1, e2) => eval(e1) + eval(e2),
        Expr::Multiplication(e1, e2) => eval(e1) * eval(e2),
    }
}

fn part1(expressions: &Vec<Expr>) -> i64 {
    expressions.iter().map(eval).sum()
}


/// Transforms Add(Mul(a, b), c) -> Mul(c, A(b, c))
fn transform_addition(e1: &Expr, e2: &Expr) -> Expr {
    let e1 = transform(e1);
    let e2 = transform(e2);

    match &e1 {
        Expr::Multiplication(e11, e12) => {
            let new_e1 = transform(e11);
            let new_e21 = transform(e12);
            // let new_e22 = transform(&e2);
            Expr::Multiplication(
                Box::new(new_e1),
                Box::new(Expr::Addition(Box::new(new_e21), Box::new(e2))))
        },
        _ => Expr::Addition(
            Box::new(e1),
            Box::new(e2)),
    }
}

/// Transforms evaluation order for part 1 to evaluation order for part 2.
fn transform(expr: &Expr) -> Expr {
    match expr {
        Expr::Number(n) => Expr::Number(*n),
        Expr::Parenthesis(e) => Expr::Parenthesis(Box::new(transform(e))),
        Expr::Addition(e12, e3) => transform_addition(e12, e3),
        Expr::Multiplication(e1, e2) =>
            Expr::Multiplication(
                Box::new(transform(&e1)),
                Box::new(transform(&e2))),
    }
}

fn part2(expressions: &Vec<Expr>) -> i64 {
    expressions.iter().map(|e| eval(&transform(&e))).sum()
}

fn main() {
    // let filename = "test_inputs/18_01.txt";
    let filename = "inputs/18.txt";
    let expressions = load_input(filename);

    // for e in expressions {
    //     println!("Old {} = {}", e, eval(&e));
    //     // println!("{:?}", e);

    //     let e = transform(&e);
    //     println!("Transformed: {} = {}", e, eval(&e));
    //     // println!("{:?}", e);
    // }
    println!("Part 1: {:?}", part1(&expressions));
    println!("Part 2: {:?}", part2(&expressions));
}

#[cfg(test)]
mod tests18 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/18_01.txt";
        let expressions = load_input(filename);
        assert_eq!(part1(&expressions), 26457);
    }

    #[test]
    fn test02() {
        let filename = "test_inputs/18_01.txt";
        let expressions = load_input(filename);
        assert_eq!(part2(&expressions), 694173);
    }
}