use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, stdin};
use std::ops::Range;

type CoordType = i32;
type ItemType = u32;

#[derive(Clone, Debug)]
struct Grid {
    width: CoordType,
    height: CoordType,
    items: Vec<ItemType>,
    horizontal: Range<CoordType>,
    vertical: Range<CoordType>,
}

trait Day11 {
    fn parse_input(lines: &Vec<String>) -> Self;
    fn step(&mut self, steps: usize) -> usize;
}

impl Grid {
    fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    fn at(&self, x: CoordType, y: CoordType) -> Option<&ItemType> {
        self.items.get((y * self.width + x) as usize)
    }

    fn set(&mut self, x: CoordType, y: CoordType, value: ItemType) {
        if self.is_inside(x, y) {
            self.items[(y * self.width + x) as usize] = value;
        }
    }

    fn is_inside(&self, x: CoordType, y: CoordType) -> bool {
        self.horizontal.contains(&x) && self.vertical.contains(&y)
    }

    fn draw(&self) {
        for y in self.vertical.clone() {
            for x in self.horizontal.clone() {
                print!("{0}", self.at(x, y).expect(format!("Could not read item at ({0:?}, {1:?})", x, y).as_str()));
            }
            println!();
        }
    }

    fn neighbor_coords(&self, x: CoordType, y: CoordType) -> [(CoordType, CoordType); 8] {
        [
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1)
        ]
    }

    fn all(&self, predicate: impl Fn(&ItemType, CoordType, CoordType) -> bool) -> bool {
        for y in self.vertical.clone() {
            for x in self.horizontal.clone() {
                let item = self.at(x, y).expect(format!("Could not read item at ({0:?}, {1:?})", x, y).as_str());
                if !predicate(item, x, y) {
                    return false;
                }
            }
        }
        true
    }
}

impl Day11 for Grid {
    fn parse_input(lines: &Vec<String>) -> Self {
        let width = lines.iter().next()
            .expect("Could not read first line")
            .len() as CoordType;

        let height = lines.len() as CoordType;

        let mut energy_level = Vec::new();

        for line in lines {
            for token in line.chars() {
                energy_level.push(token.to_digit(10).expect("Could not read energy level"));
            }
        }

        Grid {
            width,
            height,
            items: energy_level,
            horizontal: 0..width,
            vertical: 0..height,
        }
    }

    fn step(&mut self, steps: usize) -> usize {
        let mut flashes = 0;

        for step in 0..steps {
            let mut seed_points = VecDeque::new();
            seed_points.reserve_exact(self.len());

            for y in self.vertical.clone() {
                for x in self.horizontal.clone() {
                    seed_points.push_back((x, y));
                }
            }

            let mut flashed: HashSet<(CoordType, CoordType)> = HashSet::new();
            flashed.reserve(self.len());

            while !seed_points.is_empty() {
                let (x, y) = seed_points.pop_front()
                    .expect("Could not pop front of seed_points");

                if !flashed.contains(&(x, y)) {
                    let mut value = self.at(x, y).expect(format!("Could not read item at ({0:?}, {1:?})", x, y).as_str()).to_owned();
                    value += 1;

                    if value > 9 {
                        flashed.insert((x, y));

                        let unvisited: Vec<(CoordType, CoordType)> = self.neighbor_coords(x, y)
                            .into_iter()
                            .filter(|&(x, y)| self.is_inside(x, y))
                            .filter(|&(x, y)| !flashed.contains(&(x, y)))
                            .collect();

                        seed_points.extend(unvisited);

                        value = 0;
                        flashes += 1;
                    }

                    self.set(x, y, value);
                }
            }
        }

        flashes
    }
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let mut grid: Grid = Day11::parse_input(&lines);

    // grid.draw();
    // println!("grid={0:?}", grid);

    let flashes = grid.clone().step(100);
    // grid.draw();
    println!("part1: flashes={0:?}", flashes);

    let mut first_synchronize = 0;
    while !grid.all(|&value, x, y| value == 0) {
        first_synchronize += 1;
        grid.step(1);
    }
    // grid.draw();
    println!("part2: first_synchronize={0:?}", first_synchronize);
}
