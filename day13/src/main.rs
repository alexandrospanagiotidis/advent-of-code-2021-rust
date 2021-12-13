use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, stdin};

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let mut day13 = Day13::parse_input(lines);
    // println!("day13={0:?}", day13);

    let folds: VecDeque<Fold> = day13.folds.iter().map(|x| x.clone()).collect();

    folds.front()
        .map(|fold| day13.fold_once(fold));
    day13.grid.print();
    println!("part1: #dots={0:?}", day13.grid.dots.len());
}

#[derive(Debug)]
struct Day13 {
    grid: Grid,
    folds: VecDeque<Fold>,
}

impl Day13 {
    fn parse_input(lines: Vec<String>) -> Self {
        let mut grid = Grid::new();
        let mut folds = VecDeque::new();

        let mut lines = lines.iter();

        while let Some(line) = lines.next() {
            if line.is_empty() {
                break;
            }

            let mut tokens = line.split(',');

            let x = tokens.next()
                .expect(format!("Could not read x: {0:?}", line).as_str())
                .parse::<u32>()
                .expect(format!("Could not parse x: {0:?}", line).as_str());

            let y = tokens.next()
                .expect(format!("Could not read y: {0:?}", line).as_str())
                .parse::<u32>()
                .expect(format!("Could not parse y: {0:?}", line).as_str());

            grid.add(x, y);

            if grid.width < x as usize {
                grid.width = x as usize;
            }

            if grid.height < y as usize {
                grid.height = y as usize;
            }
        }

        grid.width += 1;
        grid.height += 1;

        while let Some(line) = lines.next() {
            let mut tokens = line.split_whitespace()
                .nth(2)
                .expect(format!("Could not determine folding: {0:?}", line).as_str())
                .split('=');

            let axis = tokens.next()
                .expect(format!("Could not determine fold direction: {0:?}", line).as_str());

            let value = tokens.next()
                .expect(format!("Could not read fold axis: {0:?}", line).as_str())
                .parse::<u32>()
                .expect(format!("Could not parse fold axis: {0:?}", line).as_str());

            let fold = match axis {
                "x" => Fold::LEFT(value),
                "y" => Fold::UP(value),
                _ => panic!("Unknown fold axis: {0:?}", axis),
            };

            folds.push_back(fold);
        }

        Day13 {
            grid,
            folds,
        }
    }

    fn fold_up(&mut self, value: u32) {
        let mut new_grid = Grid::new();

        for &(x, y) in self.grid.dots.iter().filter(|&(_x, y)| y < &value) {
            new_grid.add(x, y);
        }

        for &(x, y) in self.grid.dots.iter().filter(|&(_x, y)| y >= &value) {
            let new_y = self.grid.height as i32 - 1 - y as i32;
            println!("Folding {0:?} by dy={1:?} up to {2:?}", (x, y), value, new_y);
            if new_y >= 0 {
                new_grid.add(x, new_y as u32);
            } else {
                panic!("Folding up is out-of-bounds={0:?}", (new_y, value, self.grid.height))
            }
        }

        new_grid.width = self.grid.width;
        new_grid.height = value as usize;

        self.grid = new_grid;
    }

    fn fold_left(&mut self, value: u32) {
        let mut new_grid = Grid::new();

        for &(x, y) in self.grid.dots.iter().filter(|&(x, _y)| x < &value) {
            new_grid.add(x, y);
        }

        for &(x, y) in self.grid.dots.iter().filter(|&(x, _y)| x >= &value) {
            let new_x = self.grid.width as i32 - 1 - x as i32;
            println!("Folding {0:?} by dx={1:?} up to {2:?}", (x, y), value, new_x);
            if new_x >= 0 {
                new_grid.add(new_x as u32, y);
            } else {
                panic!("Folding left is out-of-bounds={0:?}", (new_x, value, self.grid.width))
            }
        }

        new_grid.width = value as usize;
        new_grid.height = self.grid.height;

        self.grid = new_grid;
    }

    fn fold_once(&mut self, fold: &Fold) {
        match fold {
            Fold::UP(value) => self.fold_up(*value),
            Fold::LEFT(value) => self.fold_left(*value),
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Fold {
    UP(u32),
    LEFT(u32),
}

type DotType = (u32, u32);

#[derive(Debug)]
struct Grid {
    dots: HashSet<DotType>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new() -> Self {
        Grid {
            dots: HashSet::new(),
            width: 0,
            height: 0,
        }
    }

    fn add(&mut self, x: u32, y: u32) {
        self.dots.insert((x, y));
    }

    fn print(&self) {
        for row in 0..self.height {
            for column in 0..self.width {
                let c = self.dots.get(&(column as u32, row as u32))
                    .map(|_| '#')
                    .unwrap_or('.');

                print!("{0}", c);
            }
            println!();
        }
    }
}
