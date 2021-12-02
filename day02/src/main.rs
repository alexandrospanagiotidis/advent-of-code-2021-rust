use std::io::{BufRead, stdin};
use std::ops::Add;

type ValueType = i32;

#[derive(Debug)]
struct Position {
    horizontal: ValueType,
    depth: ValueType,
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

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

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

    let result = position.horizontal * position.depth;

    println!("position={0:?} result={1:?}", position, result);
}
