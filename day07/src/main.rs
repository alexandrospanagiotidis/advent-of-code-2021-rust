use std::collections::{BTreeMap};
use std::io::{BufRead, stdin};

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let line = lines.iter().next()
        .expect("Could not read input line");

    let positions: Vec<i32> = line.split(',')
        .map(|token| token.parse::<i32>().unwrap())
        .collect();

    // println!("positions:{0:?}", positions);

    let same_heights = positions.iter()
        .fold(BTreeMap::new(), |mut acc, height| {
            *acc.entry(height).or_insert(0) += 1;
            acc
        });

    // println!("same_heights:{0:?}", same_heights);

    let mut target_height = -1;
    let mut global_difference = i32::MAX;

    for &current_height in same_heights.keys().rev() {
        let current_difference = positions.iter()
            .map(|position| (position - current_height).abs())
            .sum();

        if current_difference < global_difference {
            global_difference = current_difference;
            target_height = current_height.to_owned();
        }
    }

    println!("part1: target_height={0:?} result={1:?}", target_height, global_difference);
}
