use std::io::{BufRead, stdin};
use std::ops::Add;

fn main() {
    let mut numbers: Vec<i32> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .map(|line| line.parse::<i32>().expect(String::from("Could not convert to number: ").add(&line).as_str()))
        .collect();

    let previous = numbers.drain(..1).collect::<Vec<i32>>();
    let mut previous = previous.first().unwrap().to_owned();
    let mut increased = 0;

    for number in numbers {
        let difference = number - previous;

        if difference > 0 {
            increased += 1;
        }

        previous = number.to_owned();
    }

    println!("increased={0:#?}", increased);
}
