use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, stdin};
use std::ops::RangeInclusive;

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

    fn is_horizontal(&self) -> bool {
        self.x1 == self.x2
    }

    fn is_vertical(&self) -> bool {
        self.y1 == self.y2
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

    fn width(&self) -> RangeInclusive<usize> {
        RangeInclusive::new(0, (self.right - self.left) as usize)
    }

    fn height(&self) -> RangeInclusive<usize> {
        RangeInclusive::new(0, (self.bottom - self.top) as usize)
    }

    fn grow_to_contain(&mut self, x: i32, y: i32) {
        self.left = self.left.min(x);
        self.top = self.top.min(y);
        self.right = self.right.max(x);
        self.bottom = self.bottom.max(y);
    }
}

type GridKey = (i32, i32);

#[derive(Debug)]
struct Grid {
    crossings: HashMap<GridKey, i32>,
}

impl Grid {
    fn new() -> Self {
        Self {
            crossings: HashMap::new(),
        }
    }

    fn count_of_at_least_crossings(&self, min_crossings: i32) -> usize {
        self.crossings.iter()
            .filter(|(_, &count)| count >= min_crossings)
            .count()
    }

    fn increment_at(&mut self, x: i32, y: i32) {
        // println!("incrementing at ({0:?}, {1:?})", x, y);
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

    fn scan_horizontal(&mut self, y: i32, x1: i32, x2: i32) {
        let (left, right) = if x1 < x2 { (x1, x2) } else { (x2, x1) };
        // println!("horizontal: y={0:?} left={1:?} right={2:?}", y, left, right);

        for column in left..=right {
            self.increment_at(column, y);
        }
    }

    fn scan_vertical(&mut self, x: i32, y1: i32, y2: i32) {
        let (top, bottom) = if y1 < y2 { (y1, y2) } else { (y2, y1) };
        // println!("vertical: x={0:?} top={1:?} bottom={2:?}", x, top, bottom);

        for row in top..=bottom {
            self.increment_at(x, row);
        }
    }

    fn draw(&self) {
        let mut bounds = Bounds::new();

        for coordinate in self.crossings.keys() {
            bounds.grow_to_contain(coordinate.0, coordinate.1);
        }

        // println!("grid={0:?}", self);
        for row in bounds.width() {
            for column in bounds.height() {
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

    {
        let mut grid = Grid::new();

        for line in &lines {
            if line.is_horizontal() {
                grid.scan_vertical(line.x1, line.y1, line.y2);
            } else if line.is_vertical() {
                grid.scan_horizontal(line.y1, line.x1, line.x2);
            }
        }

        // grid.draw();

        let at_least_two_crossings = grid.count_of_at_least_crossings(2);

        println!("part1: result={0:?}", at_least_two_crossings);
        assert_eq!(at_least_two_crossings, 8111);
    }
    {
        let mut grid = Grid::new();

        for line in &lines {
            grid.bresenham(line);
        }

        // grid.draw();

        let at_least_two_crossings = grid.count_of_at_least_crossings(2);

        println!("part2: result={0:?}", at_least_two_crossings);
        assert_eq!(at_least_two_crossings, 22088);
    }
}
