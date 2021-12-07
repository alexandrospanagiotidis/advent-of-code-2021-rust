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

    let (target_height, global_difference) = determine_global_optimum(&positions, |lhs, rhs| (lhs - rhs).abs());
    println!("part1: target_height={0:?} result={1:?}", target_height, global_difference);
    assert_eq!(global_difference, 355764);

    let (target_height, global_difference) = determine_global_optimum(&positions, |lhs, rhs| {
        let upper_bound = (lhs - rhs).abs();
        (upper_bound * (upper_bound + 1)) / 2
    });
    println!("part2: target_height={0:?} result={1:?}", target_height, global_difference);
    assert_eq!(global_difference, 99634572);
}

fn determine_global_optimum(positions: &Vec<i32>, optim_fn: impl Fn(&i32, &i32) -> i32) -> (i32, i32) {
    let mut target_height = -1;
    let mut global_difference = i32::MAX;

    let lower_bound = positions.iter().min().unwrap().clone();
    let upper_bound = positions.iter().max().unwrap().clone();

    for current_height in lower_bound..=upper_bound {
        let current_difference = positions.iter()
            .map(|position| optim_fn(&position, &current_height))
            .sum();

        if current_difference < global_difference {
            global_difference = current_difference;
            target_height = current_height;
        }
    }

    (target_height, global_difference)
}
