use std::io::{BufRead, stdin};
use std::ops::Add;

const WINDOW_SIZE: usize = 3;

fn main() {
    let mut numbers: Vec<i32> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .map(|line| line.parse::<i32>().expect(String::from("Could not convert to number: ").add(&line).as_str()))
        .collect();

    let mut left: usize = 0;
    let mut increased = 0;

    while left + WINDOW_SIZE < numbers.len() {
        let previous_window = &numbers[left..left + WINDOW_SIZE];
        let mut previous_sum: i32 = previous_window.iter().sum();

        left += 1;

        let current_window = &numbers[left..left + WINDOW_SIZE];
        let current_sum: i32 = current_window.iter().sum();

        if current_sum > previous_sum {
            increased += 1;
        }
    }

    println!("increased={0:#?}", increased);
}
