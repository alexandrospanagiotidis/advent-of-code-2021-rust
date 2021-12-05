use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, stdin};

#[derive(Debug)]
struct Line {
    x1: u32,
    y1: u32,
    x2: u32,
    y2: u32,
}

impl Line {
    fn from_string(input: String) -> Self {
        let tokens: Vec<&str> = input.split(" -> ").collect();
        let mut tokens = tokens.iter();

        let pair = tokens.next()
            .expect(format!("Could not read x1,y1 from {0:?}", input).as_str());

        let mut pair: VecDeque<&str> = pair.split(',').collect();
        let x1 = Self::pop_as_u32(&mut pair);
        let y1 = Self::pop_as_u32(&mut pair);

        let pair = tokens.next()
            .expect(format!("Could not read x2,y2 from {0:?}", input).as_str());

        let mut pair: VecDeque<&str> = pair.split(',').collect();
        let x2 = Self::pop_as_u32(&mut pair);
        let y2 = Self::pop_as_u32(&mut pair);

        Self {
            x1,
            y1,
            x2,
            y2,
        }
    }

    fn pop_as_u32(left: &mut VecDeque<&str>) -> u32 {
        left.pop_front()
            .expect("Could not parse x1")
            .parse::<u32>()
            .unwrap()
    }

    fn left(&self) -> u32 {
        self.x1.min(self.x2)
    }

    fn right(&self) -> u32 {
        self.x1.max(self.x2)
    }

    fn top(&self) -> u32 {
        self.y1.min(self.y2)
    }

    fn bottom(&self) -> u32 {
        self.y1.max(self.y2)
    }
}

#[derive(Debug)]
struct Bounds {
    left: u32,
    top: u32,
    right: u32,
    bottom: u32,
}

impl Bounds {
    fn new() -> Self {
        Self {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        }
    }

    fn grow_to_contain(&mut self, line: &Line) {
        self.left = self.left.min(line.left());
        self.top = self.top.min(line.y1.min(line.y2));
        self.right = self.right.max(line.right());
        self.bottom = self.bottom.max(line.y1.max(line.y2));
    }
}

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    crossings: HashMap<usize, u32>,
}

impl Grid {
    fn new(bounds: &Bounds) -> Self {
        Self {
            width: (bounds.right - bounds.left) as usize,
            height: (bounds.bottom - bounds.top) as usize,
            crossings: HashMap::new(),
        }
    }

    fn scan_horizontal(&mut self, y: u32, left: u32, right: u32) {
        for column in left..right + 1 {
            let index = self.get_index(column, y);
            *self.crossings.entry(index).or_insert(0) += 1;
        }
    }

    fn scan_vertical(&mut self, x: u32, top: u32, bottom: u32) {
        for row in top..bottom + 1 {
            let index = self.get_index(x, row);
            *self.crossings.entry(index).or_insert(0) += 1;
        }
    }

    fn get_index(&self, x: u32, y: u32) -> usize {
        self.width * y as usize + x as usize
    }
}

fn main() {
    let lines: Vec<Line> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .map(|line| Line::from_string(line))
        .collect();

    // println!("lines={0:?}", lines);

    let mut bounds = Bounds::new();

    for line in &lines {
        bounds.grow_to_contain(line);
    }

    // println!("bounds={0:?}", bounds);

    let mut grid = Grid::new(&bounds);

    for line in &lines {
        if line.left() == line.right() {
            grid.scan_vertical(line.left(), line.top(), line.bottom());
        }
        if line.top() == line.bottom() {
            grid.scan_horizontal(line.top(), line.left(), line.right());
        }
    }

    // println!("grid={0:?}", grid);

    let at_least_crossings_count = grid.crossings.iter()
        .filter(|(_, &count)| count >= 2)
        .count();

    println!("part1: result={0:?}", at_least_crossings_count);
}
