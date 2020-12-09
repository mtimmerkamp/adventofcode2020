use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;


#[derive(Debug, Clone, Copy)]
enum Opcode {
    ACC,
    JMP,
    NOP
}


#[derive(Debug)]
struct Instruction {
    opcode: Opcode,
    argument: i64,
}


type Program = Vec<Instruction>;


#[derive(Debug)]
struct Processor {
    accumulator: i64,
    instruction_pointer: i64,
}


#[derive(Debug)]
enum InstructionErrorKind {
    EmptyInstruction,
    MissingArgument,
    InvalidArgument,
    UnknownOpcode,
}


#[derive(Debug)]
struct InstructionError {
    kind: InstructionErrorKind,
}


impl Instruction {
    fn read(s: &str) -> Result<Instruction, InstructionError> {
        let parts: Vec<&str> = s.split(' ').collect();

        let opcode = match parts.get(0) {
            Some(&v) => v,
            _ => return Err(InstructionError{
                kind: InstructionErrorKind::EmptyInstruction})
        };
        let opcode = match opcode {
            "acc" => Opcode::ACC,
            "jmp" => Opcode::JMP,
            "nop" => Opcode::NOP,
            _ => return Err(InstructionError{
                kind: InstructionErrorKind::UnknownOpcode})
        };

        let argument = match parts.get(1) {
            Some(&v) => v,
            None => return Err(InstructionError{
                kind: InstructionErrorKind::MissingArgument})
        };

        let argument: i64 = match argument.parse() {
            Ok(v) => v,
            _ => return Err(InstructionError{
                kind: InstructionErrorKind::InvalidArgument})
        };

        Ok(Instruction {opcode, argument})
    }
}

#[derive(Debug)]
enum ProcessorErrorKind {
    InvalidInstructionPointer,
}

#[derive(Debug)]
struct ProcessorError {
    kind: ProcessorErrorKind,
}

impl Processor {
    fn new() -> Processor {
        Processor {
            accumulator: 0,
            instruction_pointer: 0,
        }
    }

    fn run_instruction(&mut self, instruction: &Instruction) {
        match instruction.opcode {
            Opcode::ACC => self.accumulator += instruction.argument,
            Opcode::JMP => {
                self.instruction_pointer += instruction.argument;
                self.instruction_pointer -= 1;
            },
            Opcode::NOP => {},
        };
        self.instruction_pointer += 1;
    }

    fn step(&mut self, program: &Program) -> Result<(), ProcessorError> {
        if self.instruction_pointer < 0
           || self.instruction_pointer > program.len() as i64
        {
            return Err(ProcessorError {
                kind: ProcessorErrorKind::InvalidInstructionPointer
            })
        }

        let instruction = match program.get(self.instruction_pointer as usize) {
            Some(instruction) => instruction,
            None => return Err(ProcessorError {
                kind: ProcessorErrorKind::InvalidInstructionPointer}),
        };
        self.run_instruction(instruction);

        Ok(())
    }
}


fn load_program(filename: &str) -> Result<Program, InstructionError> {
    let f = File::open(filename).expect("Cannot open file");
    let reader = BufReader::new(f);

    let mut program = Vec::new();
    for line in reader.lines() {
        if let Ok(line) = line {
            match Instruction::read(&line) {
                Ok(instruction) => program.push(instruction),
                Err(e) => return Err(e),
            }
        }
    }

    Ok(program)
}


fn part1(filename: &str) -> i64 {
    let program = load_program(filename).expect("Cannot load program.");

    let mut processor = Processor::new();
    let mut ips: HashSet<i64> = HashSet::new();

    println!("{:?}", processor);
    while !ips.contains(&processor.instruction_pointer) {
        ips.insert(processor.instruction_pointer);
        match processor.step(&program) {
            Ok(_) => {},
            Err(e) => panic!("Error executing program! {:?}", e)
        }
        println!("{:?}", processor);

        if processor.instruction_pointer == program.len() as i64 {
            println!("Program halted.");
            break;
        }
    }

    processor.accumulator
}


// fn is_looping(program: &Program) -> bool {
//     is_looping_helper(program, &mut Processor::new())
// }

fn is_looping_helper(program: &Program, processor: &mut Processor) -> bool {
    let mut ips: HashSet<i64> = HashSet::new();

    while !ips.contains(&processor.instruction_pointer) {
        ips.insert(processor.instruction_pointer);
        match processor.step(&program) {
            Ok(_) => {},
            Err(e) => panic!("Error executing program! {:?}", e)
        }

        if processor.instruction_pointer == program.len() as i64 {
            return false;
        }
    }
    true
}


struct ProgramGenerator {
    program: Program,
    current_line: i64,
}

impl ProgramGenerator {
    fn new(program: Program) -> Self {
        ProgramGenerator {
            program,
            current_line: 0,
        }
    }
}

impl Iterator for ProgramGenerator {
    type Item = Program;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_line == (self.program.len() as i64) - 1 {
            return None
        }

        let mut new = Program::new();
        let mut must_modify = true;
        for (i, instr) in self.program.iter().enumerate() {
            if must_modify && i > self.current_line as usize {
                let opcode = match instr.opcode {
                    Opcode::JMP => {
                        must_modify = false;
                        Opcode::NOP
                    },
                    Opcode::NOP => {
                        must_modify = false;
                        Opcode::JMP
                    },
                    opcode => opcode,
                };
                if !must_modify {
                    self.current_line = i as i64;
                }


                new.push(Instruction{
                    opcode,
                    ..*instr
                })
            }
            else {
                new.push(Instruction{
                    ..*instr
                })
            }
        }

        Some(new)
    }
}



fn part2(filename: &str) -> Option<i64> {
    let program = load_program(filename).expect("Cannot load program.");
    let program_generator = ProgramGenerator::new(program);

    for program in program_generator {
        let mut processor = Processor::new();
        if is_looping_helper(&program, &mut processor) {
            println!("Program loops!");
        }
        else {
            println!("{:?}", program);
            return Some(processor.accumulator);
        }
    }

    None
}


fn main() {
    let filename = "inputs/08.txt";

    let acc = part1(filename);
    println!("Part1: accumulator = {}", acc);

    if let Some(acc) = part2(filename) {
        println!("Part2: accumulator = {}", acc);
    }
    else {
        println!("Part2: No non-looping program found. :(")
    }
}

#[cfg(test)]
mod tests08 {
    use super::*;

    #[test]
    fn test01() {
        let filename = "test_inputs/08_01.txt";
        let acc = part1(filename);
        assert_eq!(acc, 5);
    }

    #[test]
    fn test02() {
        let filename = "test_inputs/08_01.txt";
        let acc = part2(filename);
        assert_eq!(acc, Some(8));
    }
}