use std::env;
use std::fs;
use std::str::FromStr;
use std::hash::{Hash, Hasher};
use std::collections::HashSet;
use std::time::Instant;

#[derive(Debug, Default, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    steps: i32
}

impl Hash for Point {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug, Default)]
struct Path {
    points: HashSet<Point>
}

impl Path {
    fn run(&mut self, commands: &[String]) -> () {
        let mut last_point = Point{x: 0, y: 0, steps: 0};
        let mut iteration = commands.iter();

        loop {
            match iteration.next() {
                None => break,
                Some(command) => {
                    let mut chars = command.chars();
                    let direction = chars.next().unwrap();
                    let amount: i32 = match FromStr::from_str(&chars.as_str()) {
                        Err(e) => break,
                        Ok(a) => a
                    };

                    let points = match direction {
                        'R' => self.right(&last_point, amount),
                        'L' => self.left(&last_point, amount),
                        'U' => self.up(&last_point, amount),
                        'D' => self.down(&last_point, amount),
                        _ => self.noop()
                    };

                    last_point = points.last().unwrap().clone();
                    for point in &points {
                        if !self.include(point) {
                            self.points.insert(*point);
                        }
                    }
                }
            }
        }
    }

    fn include(&self, point: &Point) -> bool {
        self.points.contains(point)
    }

    fn find(&self, point: &Point) -> &Point {
        self.points.get(point).unwrap()
    }

    fn noop(&self) -> Vec<Point> {
        Vec::new()
    }

    fn right(&self, point: &Point, amount: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for x in 1..=amount {
            points.push(Point{x: point.x + x, y: point.y, steps: point.steps + x});
        }

        return points;
    }

    fn left(&self, point: &Point, amount: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for x in 1..=amount {
            points.push(Point{x: point.x - x, y: point.y, steps: point.steps + x});
        }

        return points;
    }

    fn up(&self, point: &Point, amount: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for y in 1..=amount {
            points.push(Point{x: point.x, y: point.y + y, steps: point.steps + y});
        }

        return points;
    }

    fn down(&self, point: &Point, amount: i32) -> Vec<Point> {
        let mut points = Vec::new();

        for y in 1..=amount {
            points.push(Point{x: point.x, y: point.y - y, steps: point.steps + y});
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
    let now = Instant::now();
    let args: Vec<String> = env::args().collect();
    let raw_paths = parse(&args[1]);

    let mut main: Path = Path::default();

    main.run(&raw_paths[0]);

    let mut secondary: Path = Path::default();
    secondary.run(&raw_paths[1]);

    let mut intersections = Vec::new();

    for point in &secondary.points {
        if main.include(point) {
            let main_point = main.find(point);
            let composed_point = Point{
                x: point.x,
                y: point.y,
                steps: main_point.steps + point.steps
            };

            intersections.push(composed_point)
        }
    }

    let winner = intersections.iter().min_by_key(|point| point.steps).unwrap();
    println!("Winner: {:?}", winner);
    println!("{}ms", now.elapsed().as_millis());
}
