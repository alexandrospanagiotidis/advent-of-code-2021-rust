use std::io::{BufRead, stdin};

type ItemType = u32;

#[derive(Debug)]
struct Grid {
    width: usize,
    height: usize,
    heightmap: Vec<ItemType>,
}

impl Grid {
    fn from_lines(lines: &Vec<String>) -> Self {
        let width = lines.iter().next()
            .expect("Could not read first line")
            .len();

        let height = lines.len();

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
        }
    }

    fn at(&self, x: usize, y: usize) -> Option<&ItemType> {
        self.heightmap.get(y * self.width + x)
    }

    fn is_lowest(&self, x: usize, y: usize) -> bool {
        let mid = self.at(x, y).expect("Could not determine mid");

        if y > 0 {
            if let Some(above) = self.at(x, y - 1) {
                if above <= mid {
                    return false;
                }
            }
        }

        if x > 0 {
            if let Some(left) = self.at(x - 1, y) {
                if left <= mid {
                    return false;
                }
            }
        }

        if x < self.width - 1 {
            if let Some(right) = self.at(x + 1, y) {
                if right <= mid {
                    return false;
                }
            }
        }

        if y < self.height - 1 {
            if let Some(bottom) = self.at(x, y + 1) {
                if bottom <= mid {
                    return false;
                }
            }
        }

        true
    }

    fn all_lowest_points(&self) -> Vec<&ItemType> {
        let mut lowest_points = Vec::new();

        for row in 0..self.height {
            for column in 0..self.width {
                if self.is_lowest(column, row) {
                    lowest_points.push(self.at(column, row)
                        .expect("Could not get lowest point"));
                }
            }
        }

        lowest_points
    }
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let grid = Grid::from_lines(&lines);
    // println!("grid={0:?}", grid);

    let lowest_points = grid.all_lowest_points();
    let risk_level: ItemType = lowest_points.into_iter()
        .map(|&lowest_value| lowest_value + 1)
        .sum();
    println!("part1: risk_level={0:?}", risk_level);
    assert_eq!(risk_level, 500);
}
