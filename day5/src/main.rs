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
    parameters: Parameters
}

impl Instruction {
    fn construct(codes: &[i32], position: usize) -> Instruction {
        let (op_code, size, modes) = Instruction::modes(codes[position]);
        let mut parameters = Parameters::default();

        for x in 0..size {
            parameters.add(codes[position + x as usize], modes[x as usize], codes);
        }

        Instruction {
            operation: op_code,
            parameters: parameters
        }
    }

    fn modes(number: i32) -> (OpCode, u8, [Mode; 3]) {
        let mut modes = [Mode::Position; 3];
        for x in 1..=3 {
            modes[x - 1] = match number % 10_i32.pow(6_u32 - x as u32) / 10_i32.pow(7_u32 - x as u32) {
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
            OpCode::Addition | OpCode::Multiplication => 4,
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
            OpCode::Output => self.output(),
            OpCode::Exit | OpCode::Invalid => {}
        };

        return (self.operation == OpCode::Exit, self.parameters.len());
    }

    fn size (&self) -> usize {
        self.parameters.len() + 1
    }

    fn add(&self, codes: &mut Vec<i32>) -> () {
        let result = self.parameters.values[0] + self.parameters.values[1];
        codes[self.parameters.values[3] as usize] = result as i32;
    }

    fn multiply(&self, mut codes: &mut Vec<i32>) -> () {
        let result = self.parameters.values[0] * self.parameters.values[1];
        codes[self.parameters.values[3] as usize] = result as i32;
    }

    fn input(&self, mut codes: &mut Vec<i32>) -> () {
        let stdin = io::stdin();

        let input = stdin.lock().lines().next().unwrap().unwrap();
        let value = input.parse::<i32>().unwrap();

        codes[self.parameters.values[0] as usize] = value;
    }

    fn output(&self) -> () {
        println!("{:?}", self.parameters.values[0]);
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum Mode {
    Position,
    Immediate,
    Invalid
}

#[derive(Debug, Default)]
struct Parameters {
    count: u8,
    values: Vec<isize>
}

impl Parameters {
    fn add(&mut self, value: i32, mode: Mode, codes: &[i32]) -> () {
        match mode {
            Mode::Immediate => &self.values.push(value as isize),
            Mode::Position => &self.values.push(codes[value as usize] as isize),
            Mode::Invalid => &println!("Couldn't handle the invalid mode")
        };
    }

    fn len(&self) -> usize {
        self.values.len()
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
            println!("{:?}", instruction);
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
