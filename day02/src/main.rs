use std::fmt::Debug;
use std::io::{BufRead, stdin};

type ValueType = i32;

#[derive(Clone, Debug)]
struct Command {
    command: String,
    value: ValueType,
}

impl Command {
    fn new(line: String) -> Self {
        let mut tokens = line.split(" ");

        let command = tokens.next().unwrap().to_owned();
        let value: ValueType = tokens.next()
            .map(|token| token.parse::<>().unwrap())
            .unwrap();

        Self {
            command,
            value,
        }
    }
}

trait Navigation {
    fn forward(&mut self, value: ValueType);

    fn up(&mut self, value: ValueType);

    fn down(&mut self, value: ValueType);

    fn execute(&mut self, command: Command) {
        match command.command.as_str() {
            "forward" => self.forward(command.value),
            "up" => self.up(command.value),
            "down" => self.down(command.value),
            _ => panic!("Invalid command: {0:?}", command),
        };
    }
}

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

type CommandList = Vec<Command>;

impl Position {
    fn new() -> Self {
        Position {
            horizontal: ValueType::default(),
            depth: ValueType::default(),
        }
    }
}

impl Navigation for Position {
    fn forward(&mut self, delta: ValueType) {
        self.horizontal += delta;
    }

    fn up(&mut self, delta: ValueType) {
        self.depth -= delta;
    }

    fn down(&mut self, delta: ValueType) {
        self.depth += delta;
    }
}

impl PositionAim {
    fn new() -> Self {
        PositionAim {
            horizontal: ValueType::default(),
            depth: ValueType::default(),
            aim: ValueType::default(),
        }
    }
}

impl Navigation for PositionAim {
    fn forward(&mut self, delta: ValueType) {
        self.horizontal += delta;
        self.depth += self.aim * delta;
    }

    fn up(&mut self, delta: ValueType) {
        self.aim -= delta;
    }

    fn down(&mut self, delta: ValueType) {
        self.aim += delta;
    }
}

fn part1(commands: CommandList) -> ValueType {
    let mut position = Position::new();

    for command in commands {
        position.execute(command);
    }

    println!("position={0:?}", position);

    position.horizontal * position.depth
}

fn part2(commands: CommandList) -> ValueType {
    let mut position = PositionAim::new();

    for command in commands {
        position.execute(command);
    }

    println!("position={0:?}", position);

    position.horizontal * position.depth
}

fn main() {
    let commands: CommandList = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .map(|line| Command::new(line))
        .collect();

    {
        let result = part1(commands.clone());
        println!("part1: result={0:?}", result);
        assert_eq!(result, 1660158);
    }

    {
        let result = part2(commands.clone());
        println!("part2: result={0:?}", result);
        assert_eq!(result, 1604592846);
    }
}
