use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Default)]
struct Point {
    x: i32,
    y: i32
}

#[derive(Debug, Default)]
struct Path {
    points: Vec<Point>
}

impl Path {
    fn run(&mut self, commands: &[String]) -> () {
        self.points.push(Point{x: 0, y: 0});
        let mut iteration = commands.iter();

        loop {
            match iteration.next() {
                None => break,
                Some(command) => {
                    let mut chars = command.chars();
                    let direction = chars.next().unwrap();
                    println!("{:?}", direction);
                    let amount: i32 = FromStr::from_str(&chars.as_str()[1..]).unwrap();

                    match direction {
                        'R' => self.right(&amount),
                        'L' => self.left(&amount),
                        'U' => self.up(&amount),
                        'D' => self.down(&amount),
                        _ => println!("Error, value outside of scope")
                    }
                }
            }
        }
    }

    fn right(&mut self, amount: &i32) -> () {
    }

    fn left(&mut self, amount: &i32) -> () {
    }

    fn up(&mut self, amount: &i32) -> () {
    }

    fn down(&mut self, amount: &i32) -> () {
    }
}

fn parse(filename: &str) -> Vec<Vec<String>> {
    let text = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    text.trim().split("\n").map(|path| {
        path.split(",").map(String::from).collect()
    }).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let raw_paths = parse(&args[1]);

    let mut main: Path = Path::default();

    main.run(&raw_paths[0]);
    println!("{:?}", main);
}
