use std::io::{BufRead, stdin};
use std::ops::Add;

type ValueType = i32;

#[derive(Debug)]
struct Position {
    horizontal: ValueType,
    depth: ValueType,
}

#[derive(Debug)]
struct PositionAim {
    horizontal: ValueType,
    depth: ValueType,
    aim: ValueType,
}

impl Position {
    fn new() -> Self {
        Position {
            horizontal: 0,
            depth: 0,
        }
    }

    fn forward(&mut self, delta: ValueType) {
        self.horizontal += delta;
    }

    fn down(&mut self, delta: ValueType) {
        self.depth += delta;
    }

    fn up(&mut self, delta: ValueType) {
        self.depth -= delta;
    }
}

impl PositionAim {
    fn new() -> Self {
        PositionAim {
            horizontal: 0,
            depth: 0,
            aim: 0,
        }
    }

    fn forward(&mut self, delta: ValueType) {
        self.horizontal += delta;
        self.depth += self.aim * delta;
    }

    fn down(&mut self, delta: ValueType) {
        self.aim += delta;
    }

    fn up(&mut self, delta: ValueType) {
        self.aim -= delta;
    }
}

fn part1(lines: Vec<String>) -> ValueType {
    let mut position = Position::new();

    for line in lines {
        let mut tokens = line.split(" ");

        let command = tokens.next().unwrap();
        let delta: ValueType = tokens.next().map(|token| token.parse::<ValueType>()).unwrap().unwrap();

        match command {
            "forward" => position.forward(delta),
            "up" => position.up(delta),
            "down" => position.down(delta),
            _ => panic!("Invalid command: {0:?}", command),
        };
    }

    println!("position={0:?}", position);

    position.horizontal * position.depth
}

fn part2(lines: Vec<String>) -> ValueType {
    let mut position = PositionAim::new();

    for line in lines {
        let mut tokens = line.split(" ");

        let command = tokens.next().unwrap();
        let delta: ValueType = tokens.next().map(|token| token.parse::<ValueType>()).unwrap().unwrap();

        match command {
            "forward" => position.forward(delta),
            "up" => position.up(delta),
            "down" => position.down(delta),
            _ => panic!("Invalid command: {0:?}", command),
        };
    }

    println!("position={0:?}", position);

    position.horizontal * position.depth
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    {
        let result = part1(lines.clone());
        println!("part1: result={0:?}", result);
    }

    {
        let result = part2(lines.clone());
        println!("part2: result={0:?}", result);
    }
}
