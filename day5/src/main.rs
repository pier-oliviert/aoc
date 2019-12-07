use std::io::{self, BufRead};
use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum OpCode {
    Addition,
    Multiplication,
    Input,
    Output,
    JumpIf(bool),
    Comparison(Comparison),
    Exit,
    Invalid,
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Comparison {
    LesserThan,
    Equality
}

#[derive(Debug)]
struct Instruction {
    operation: OpCode,
    parameters: Vec<Parameter>,
    address: usize,
    origin: i32
}

impl Instruction {
    fn construct(codes: &[i32], address: usize) -> Instruction {
        let (op_code, size, modes) = Instruction::modes(codes, &address);
        let mut parameters = Vec::new();

        for x in 1..=size {
            parameters.push(Parameter {
                value: codes[address + x] as isize,
                mode: modes[x - 1]
            })
        }

        Instruction {
            address: address,
            origin: codes[address],
            operation: op_code,
            parameters: parameters
        }
    }

    fn modes(codes: &[i32], address: &usize) -> (OpCode, usize, [Mode; 3]) {
        let pointer = codes[*address];
        let mut modes = [Mode::Position; 3];
        for x in 1..=3 {
            let mode = pointer % 10_i32.pow(6_u32 - x as u32) / 10_i32.pow(5_u32 - x as u32);
            modes[x - 1] = match mode {
                0 => Mode::Position,
                1 => Mode::Immediate,
                _ => Mode::Invalid
            };
        }

        modes.reverse();

        let op_code = match pointer % 100 {
            1 => OpCode::Addition,
            2 => OpCode::Multiplication,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIf(true),
            6 => OpCode::JumpIf(false),
            7 => OpCode::Comparison(Comparison::LesserThan),
            8 => OpCode::Comparison(Comparison::Equality),
            99 => OpCode::Exit,
            _ => OpCode::Invalid
        };

        let size = match op_code {
            OpCode::Addition | OpCode::Multiplication | OpCode::Comparison(_) => 3,
            OpCode::JumpIf(_) => 2,
            OpCode::Input | OpCode::Output => 1,
            OpCode::Exit | OpCode::Invalid => 0
        };

        return (op_code, size, modes);
    }

    fn execute(&self, codes: &mut Vec<i32>) -> (bool, usize) {
        let position = match self.operation {
            OpCode::Addition => self.add(codes),
            OpCode::Multiplication => self.multiply(codes),
            OpCode::Input => self.input(codes),
            OpCode::Output => self.output(codes),
            OpCode::JumpIf(condition) => self.jump(codes, &condition),
            OpCode::Comparison(condition) => self.compare(codes, &condition),
            OpCode::Invalid => {
                println!("Invalid Instruction! {:?}", self);
                1
            },
            OpCode::Exit => 0
        };

        return (self.operation == OpCode::Exit, position);
    }

    fn add(&self, codes: &mut Vec<i32>) -> usize {
        let result = self.parameters[0].content(codes) + self.parameters[1].content(codes);
        codes[self.parameters[2].value as usize] = result as i32;

        return self.address + self.parameters.len() + 1;
    }

    fn multiply(&self, codes: &mut Vec<i32>) -> usize {
        let result = self.parameters[0].content(codes) * self.parameters[1].content(codes);
        codes[self.parameters[2].value as usize] = result as i32;

        return self.address + self.parameters.len() + 1;
    }

    fn input(&self, codes: &mut Vec<i32>) -> usize {
        let stdin = io::stdin();

        println!("Input a number: ");
        let input = stdin.lock().lines().next().unwrap().unwrap();
        let value = input.parse::<i32>().unwrap();

        codes[self.parameters[0].value as usize] = value;

        return self.address + self.parameters.len() + 1;
    }

    fn jump(&self, codes: &mut Vec<i32>, is_zero: &bool) -> usize {
        if (self.parameters[0].content(codes) == 0) != *is_zero {
            return self.parameters[1].content(codes) as usize;
        }

        return self.address + self.parameters.len() + 1;
    }

    fn compare(&self, codes: &mut Vec<i32>, condition: &Comparison) -> usize {
        let address = self.parameters[2].value as usize;
        let left = self.parameters[0].content(codes);
        let right = self.parameters[1].content(codes);

        match condition {
            Comparison::Equality => codes[address] = (left == right) as i32,
            Comparison::LesserThan => codes[address] = (left < right) as i32
        }

        return self.address + self.parameters.len() + 1;
    }

    fn output(&self, codes: &[i32]) -> usize {
        println!("{:?}", self.parameters[0].content(codes));

        return self.address + self.parameters.len() + 1;
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Mode {
    Position,
    Immediate,
    Invalid
}

#[derive(Debug)]
struct Parameter {
    value: isize,
    mode: Mode
}

impl Parameter {
    fn content(&self, codes: &[i32]) -> (isize) {
        match self.mode {
            Mode::Immediate => self.value,
            Mode::Position => codes[self.value as usize] as isize,
            Mode::Invalid => {
                println!("Couldn't handle the invalid mode");
                -1
            }
        }
    }
}

struct Processor {
    done: bool,
}

impl Processor {
    fn default () -> Processor {
        Processor{
            done: false,
        }
    }

    fn compute(&mut self, codes: &mut Vec<i32>) -> usize {
        let mut address = 0;

        while !self.done && address < codes.len() {
            let instruction = Instruction::construct(&codes, address);
            let (should_exit, new_address) = instruction.execute(codes);

            address = new_address;
            self.done = should_exit;
        }

        return address;
    }
}

fn parse_file(contents: &str) -> Vec<i32> {
    contents.trim().split(",").map(|x| x.parse::<i32>().unwrap()).collect()
}

fn main() {
    println!("Hello! This is day2 of advent of code 2019.");
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut codes = parse_file(&text);
    let mut processor = Processor::default();

    processor.compute(&mut codes);
}
