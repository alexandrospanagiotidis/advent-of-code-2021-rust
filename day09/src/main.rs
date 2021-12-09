use std::collections::{HashSet, VecDeque};
use std::io::{BufRead, stdin};
use std::ops::Range;

type CoordType = i32;
type ItemType = u32;

#[derive(Debug)]
struct Grid {
    width: CoordType,
    height: CoordType,
    heightmap: Vec<ItemType>,
    horizontal: Range<CoordType>,
    vertical: Range<CoordType>,
}

impl Grid {
    fn from_lines(lines: &Vec<String>) -> Self {
        let width = lines.iter().next()
            .expect("Could not read first line")
            .len() as CoordType;

        let height = lines.len() as CoordType;

        let mut heightmap = Vec::new();

        for line in lines {
            for token in line.chars() {
                heightmap.push(token.to_digit(10).expect("Could not read item"));
            }
        }

        Self {
            width,
            height,
            heightmap,
            horizontal: 0..width,
            vertical: 0..height,
        }
    }

    fn at(&self, x: CoordType, y: CoordType) -> Option<&ItemType> {
        self.heightmap.get((y * self.width + x) as usize)
    }

    fn neighbors_match(&self, x: CoordType, y: CoordType, predicate: impl Fn(&ItemType, CoordType, CoordType) -> bool) -> bool {
        let mid = self.at(x, y).expect("Could not determine mid");

        for (x, y) in [(x, y - 1), (x - 1, y), (x + 1, y), (x, y + 1)] {
            if self.is_inside(x, y) && !predicate(mid, x, y) {
                return false;
            }
        }

        true
    }

    fn all_lowest_points(&self) -> Vec<(&ItemType, CoordType, CoordType)> {
        let mut lowest_points = Vec::new();

        for row in self.vertical.clone() {
            for column in self.horizontal.clone() {
                if self.neighbors_match(column, row, |mid, x, y| {
                    self.at(x, y).map_or(false, |value| mid < value)
                }) {
                    let value = self.at(column, row)
                        .expect("Could not get lowest point");
                    lowest_points.push((value, column, row));
                }
            }
        }

        lowest_points
    }

    fn is_inside(&self, x: CoordType, y: CoordType) -> bool {
        self.horizontal.contains(&x) && self.vertical.contains(&y)
    }

    fn find_regions(&self, x: CoordType, y: CoordType, predicate: impl Fn(i32, i32) -> bool) -> Vec<&ItemType> {
        let mut values = Vec::new();

        let mut visited: HashSet<(CoordType, CoordType)> = HashSet::new();
        visited.insert((x, y));

        let mut seed_points = VecDeque::new();
        seed_points.push_front((x, y));

        while !seed_points.is_empty() {
            let p = seed_points.pop_front()
                .expect("Could not get next seed point");

            let value = self.at(p.0, p.1)
                .expect("Could not get mid point");

            values.push(value);

            for (x, y) in [(p.0, p.1 - 1 as CoordType), (p.0 - 1, p.1), (p.0 + 1, p.1), (p.0, p.1 + 1)] {
                if !visited.contains(&(x, y)) && self.is_inside(x, y) && predicate(x, y) {
                    seed_points.push_back((x, y));
                    visited.insert((x, y));
                }
            }
        }

        values
    }
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let grid = Grid::from_lines(&lines);
    // println!("grid={0:?}", grid);

    let lowest_points = grid.all_lowest_points();
    let risk_level: ItemType = lowest_points.iter()
        .map(|low_point| low_point.0 + 1)
        .sum();
    println!("part1: risk_level={0:?}", risk_level);
    assert_eq!(risk_level, 500);

    let mut basins = Vec::new();

    for lowest_point in lowest_points.iter() {
        basins.push(grid.find_regions(lowest_point.1, lowest_point.2, |x, y| {
            grid.at(x, y).map_or(false, |value| value < &9)
        }));
    }

    basins.sort_by(|a, b| (b.len().partial_cmp(&a.len())).unwrap());
    // println!("basins={0:?}", basins);

    let product_three_largest_basins = basins.iter()
        .take(3)
        .map(|basin| basin.len())
        .map(|len| len)
        .fold(1, |acc, value| acc * value);
    println!("part2: product_three_largest_basins={0:?}", product_three_largest_basins);
    assert_eq!(product_three_largest_basins, 970200);
}
