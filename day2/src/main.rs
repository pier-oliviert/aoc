use std::env;
use std::fs;
use std::fmt;

struct IntCode {
    done: bool,
    noun: i32,
    verb: i32,
    output: i32,
    position: usize,
    codes: Vec<i32>
}

impl IntCode {
    fn default () -> IntCode {
        IntCode{
            done: false,
            noun: 0,
            verb: 0,
            output: 0,
            position: 0,
            codes: vec![]
        }
    }

    fn compute(&mut self) -> () {
        while !self.done && self.position < self.codes.len() {
            let start = self.position;
            let end = self.position + 4;
            let slice: Vec<i32> = self.codes[start..end].iter().cloned().collect();

            self.process(&slice);
            self.position += 4
        }
    }

    fn process(&mut self, row: &[i32]) -> () {
        let op_code = *row.get(0).unwrap_or(&99);
        let a = *self.codes.get(row[1] as usize).unwrap_or(&0);
        let b = *self.codes.get(row[2] as usize).unwrap_or(&0);
        let position = *row.get(3).unwrap_or(&0) as usize;

        match op_code {
            99 => self.finish(),
            1 => self.store(self.add(a, b), position),
            2 => self.store(self.multiply(a, b), position),
            _ => println!("Error, value outside of scope")
        }
    }

    fn finish(&mut self) -> () {
        self.done = true;
        self.output = *self.codes.get(0).unwrap();
        self.noun = *self.codes.get(1).unwrap();
        self.verb = *self.codes.get(2).unwrap();
    }

    fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }

    fn multiply(&self, a: i32, b: i32) -> i32 {
        a * b
    }

    fn store(&mut self, value: i32, position: usize) -> () {
        self.codes[position] = value
    }
}

impl fmt::Debug for IntCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "IntCode {{ codes: {:?} }}", self.codes)
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

    let memory = parse_file(&text);
    let output = 19690720;
    let mut seed = 0;

    loop {
        let mut copy: Vec<i32> = memory.iter().cloned().collect();
        copy[1] = seed / 100;
        copy[2] = seed % 100;

        let mut int_code: IntCode = IntCode { codes: copy, .. IntCode::default() };
        int_code.compute();

        if int_code.output == output {
            println!("{:?}", [int_code.noun, int_code.verb]);
            println!("{:?}", int_code.codes);
            break;
        }

        seed += 1;
    }
}
