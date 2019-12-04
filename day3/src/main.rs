use std::env;
use std::fs;
use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Eq)]
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
                    let point = self.points.last().unwrap().clone();
                    let mut chars = command.chars();
                    let direction = chars.next().unwrap();
                    let amount: i32 = match FromStr::from_str(&chars.as_str()) {
                        Err(e) => break,
                        Ok(a) => a
                    };

                    let mut points = match direction {
                        'R' => self.right(point, amount),
                        'L' => self.left(point, amount),
                        'U' => self.up(point, amount),
                        'D' => self.down(point, amount),
                        _ => self.noop()
                    };

                    self.points.append(&mut points);
                }
            }
        }
    }

    fn include(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    fn noop(&self) -> Vec<Point> {
        Vec::new()
    }

    fn right(&self, point: &Point, amount: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for x in 1..amount {
            points.push(Point{x: point.x + x, y: point.y});
        }

        return points;
    }

    fn left(&self, point: &Point, amount: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for x in 1..amount {
            points.push(Point{x: point.x - x, y: point.y});
        }

        return points;
    }

    fn up(&self, point: &Point, amount: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for y in 1..amount {
            points.push(Point{x: point.x, y: point.y + y});
        }

        return points;
    }

    fn down(&self, point: &Point, amount: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for y in 1..amount {
            points.push(Point{x: point.x, y: point.y - y});
        }

        return points;
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

    let mut secondary: Path = Path::default();
    secondary.run(&raw_paths[1]);

    let mut intersections = Vec::new();

    for point in &secondary.points {
        if main.include(point) {
            println!("{:?}", point);
            intersections.push(point.clone())
        }
    }

    println!("{:?}", intersections);
}
