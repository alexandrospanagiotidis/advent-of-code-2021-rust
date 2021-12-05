use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, stdin};

#[derive(Debug)]
struct Line {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Line {
    fn from_string(input: String) -> Self {
        let tokens: Vec<&str> = input.split(" -> ").collect();
        let mut tokens = tokens.iter();

        let pair = tokens.next()
            .expect(format!("Could not read x1,y1 from {0:?}", input).as_str());

        let mut pair: VecDeque<&str> = pair.split(',').collect();
        let x1 = Self::pop_as_i32(&mut pair);
        let y1 = Self::pop_as_i32(&mut pair);

        let pair = tokens.next()
            .expect(format!("Could not read x2,y2 from {0:?}", input).as_str());

        let mut pair: VecDeque<&str> = pair.split(',').collect();
        let x2 = Self::pop_as_i32(&mut pair);
        let y2 = Self::pop_as_i32(&mut pair);

        Self {
            x1,
            y1,
            x2,
            y2,
        }
    }

    fn pop_as_i32(left: &mut VecDeque<&str>) -> i32 {
        left.pop_front()
            .expect("Could not parse x1")
            .parse::<i32>()
            .unwrap()
    }

    fn left(&self) -> i32 {
        self.x1.min(self.x2)
    }

    fn right(&self) -> i32 {
        self.x1.max(self.x2)
    }

    fn top(&self) -> i32 {
        self.y1.min(self.y2)
    }

    fn bottom(&self) -> i32 {
        self.y1.max(self.y2)
    }
}

#[derive(Debug)]
struct Bounds {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
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

type GridKey = (i32, i32);

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    crossings: HashMap<GridKey, i32>,
}

impl Grid {
    fn new(bounds: &Bounds) -> Self {
        Self {
            width: (bounds.right - bounds.left + 1) as usize,
            height: (bounds.bottom - bounds.top + 1) as usize,
            crossings: HashMap::new(),
        }
    }

    fn increment_at(&mut self, x: i32, y: i32) {
        println!("incrementing at ({0:?}, {1:?})", x, y);
        *self.crossings.entry((x, y)).or_insert(0) += 1
    }

    // via https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
    fn bresenham(&mut self, line: &Line) {
        let (mut x1, mut y1, x2, y2) = (line.x1, line.y1, line.x2, line.y2);

        let dx = (x2 - x1).abs();
        let sx = if x1 < x2 { 1 } else { -1 };

        let dy = -(y2 - y1).abs();
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx + dy;

        // println!("scan: line={0:?} dx={1:?} dy={2:?}", line, dx, dy);

        loop {
            self.increment_at(x1, y1);

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = err + err;

            if e2 >= dy {
                err += dy;
                x1 += sx;
            }

            if e2 <= dx {
                err += dx;
                y1 += sy;
            }
        }
    }

    fn scan_horizontal(&mut self, y: i32, left: i32, right: i32) {
        // println!("horizontal: y={0:?} left={1:?} right={2:?}", y, left, right);
        for column in left..=right {
            self.increment_at(column, y);
        }
    }

    fn scan_vertical(&mut self, x: i32, top: i32, bottom: i32) {
        // println!("vertical: x={0:?} top={1:?} bottom={2:?}", x, top, bottom);
        for row in top..=bottom {
            self.increment_at(x, row);
        }
    }

    fn draw(&self) {
        // println!("grid={0:?}", self);
        for row in 0..self.width {
            for column in 0..self.height {
                let key: GridKey = (column as i32, row as i32);
                if let Some(count) = self.crossings.get(&key) {
                    print!("{0}", count);
                } else {
                    print!(".");
                }
            }
            println!("|");
        }
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

    {
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
    {
        let mut grid = Grid::new(&bounds);

        for line in &lines {
            println!("scanning line={0:?}", line);
            grid.bresenham(line);
        }

        let at_least_crossings_count = grid.crossings.iter()
            .filter(|(_, &count)| count >= 2)
            .count();

        grid.draw();
        println!("part2: result={0:?}", at_least_crossings_count);
    }
}
