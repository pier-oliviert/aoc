use std::io::{self, BufRead};
use std::env;
use std::fs;

#[derive(Debug, PartialEq)]
enum OpCode {
    Addition,
    Multiplication,
    Input,
    Output,
    Exit,
    Invalid,
}

#[derive(Debug)]
struct Instruction {
    operation: OpCode,
    parameters: Vec<Parameter>
}

impl Instruction {
    fn construct(codes: &[i32], position: usize) -> Instruction {
        let (op_code, size, modes) = Instruction::modes(codes[position]);
        let mut parameters = Vec::new();

        for x in 1..=size {
            parameters.push(Parameter {
                value: codes[position + x] as isize,
                mode: modes[x - 1]
            })
        }

        Instruction {
            operation: op_code,
            parameters: parameters
        }
    }

    fn modes(number: i32) -> (OpCode, usize, [Mode; 3]) {
        let mut modes = [Mode::Position; 3];
        for x in 1..=3 {
            let mode = number % 10_i32.pow(6_u32 - x as u32) / 10_i32.pow(5_u32 - x as u32);
            modes[x - 1] = match mode {
                0 => Mode::Position,
                1 => Mode::Immediate,
                _ => Mode::Invalid
            };
        }

        modes.reverse();

        let op_code = match number % 100 {
            1 => OpCode::Addition,
            2 => OpCode::Multiplication,
            3 => OpCode::Input,
            4 => OpCode::Output,
            99 => OpCode::Exit,
            _ => OpCode::Invalid
        };

        let size = match op_code {
            OpCode::Addition | OpCode::Multiplication => 3,
            OpCode::Input | OpCode::Output => 1,
            OpCode::Exit | OpCode::Invalid => 0
        };

        return (op_code, size, modes);
    }


    fn execute(&self, codes: &mut Vec<i32>) -> (bool, usize) {
        match self.operation {
            OpCode::Addition => self.add(codes),
            OpCode::Multiplication => self.multiply(codes),
            OpCode::Input => self.input(codes),
            OpCode::Output => self.output(codes),
            OpCode::Exit | OpCode::Invalid => {}
        };

        return (self.operation == OpCode::Exit, self.parameters.len() + 1);
    }

    fn add(&self, codes: &mut Vec<i32>) -> () {
        let result = self.parameters[0].content(codes) + self.parameters[1].content(codes);
        codes[self.parameters[2].value as usize] = result as i32;
    }

    fn multiply(&self, codes: &mut Vec<i32>) -> () {
        let result = self.parameters[0].content(codes) * self.parameters[1].content(codes);
        codes[self.parameters[2].value as usize] = result as i32;
    }

    fn input(&self, codes: &mut Vec<i32>) -> () {
        let stdin = io::stdin();

        println!("Input a number: ");
        let input = stdin.lock().lines().next().unwrap().unwrap();
        let value = input.parse::<i32>().unwrap();

        codes[self.parameters[0].value as usize] = value;
    }

    fn output(&self, codes: &[i32]) -> () {
        println!("{:?}", self.parameters[0].content(codes));
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
                &println!("Couldn't handle the invalid mode");
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
        let mut position = 0;

        while !self.done && position < codes.len() {
            let instruction = Instruction::construct(&codes, position);
            let (should_exit, size) = instruction.execute(codes);
            position += size;

            self.done = should_exit
        }

        return position;
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
