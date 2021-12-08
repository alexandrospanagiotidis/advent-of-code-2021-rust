use std::collections::{HashMap};
use std::io::{BufRead, stdin};

use itertools::Itertools;

#[derive(Debug)]
struct SevenSegmentDisplay {
    digits: HashMap<i32, String>,
    output: Vec<i32>,
}

impl SevenSegmentDisplay {
    fn from_line(line: &String) -> Self {
        let mut input_output = line.split('|');

        let display_digits = input_output.next().expect("Could not determine display digits");
        let mut digits = HashMap::new();

        for display_digit in display_digits.split(' ') {
            // println!("display_digit={0:?}", display_digit);
            match display_digit.len() {
                2 => {
                    digits.insert(1, Self::sort_string(display_digit));
                }
                4 => {
                    digits.insert(4, Self::sort_string(display_digit));
                }
                3 => {
                    digits.insert(7, Self::sort_string(display_digit));
                }
                7 => {
                    digits.insert(8, Self::sort_string(display_digit));
                }
                _ => (),
            }
        }

        let output_digits = input_output.next().expect("Could not determine output digits");
        let mut output = Vec::new();

        for output_digit in output_digits.split(' ') {
            let output_digit = Self::sort_string(output_digit);
            // println!("output_digit={0:?}", output_digit);

            let output_digit = digits.iter()
                .filter(|&(_digit, segments)| *segments == output_digit)
                .nth(0);

            match output_digit {
                Some(v) => output.push(*v.0),
                None => (),
            }
        }

        Self {
            digits,
            output,
        }
    }

    #[inline]
    fn sort_string(input: &str) -> String {
        input.chars().sorted().collect::<String>()
    }
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let mut parsed_lines = Vec::new();
    parsed_lines.reserve_exact(lines.len());

    for line in lines {
        let display = SevenSegmentDisplay::from_line(&line);
        // println!("display={0:?}", display);

        parsed_lines.push(display);
    }

    let unique_numbers = parsed_lines.iter()
        .flat_map(|parsed_line| parsed_line.output.iter())
        .filter(|&output_digit| vec![1, 4, 7, 8].contains(output_digit))
        .count();
    println!("part1: unique_numbers={0:?}", unique_numbers);
    assert_eq!(unique_numbers, 392);
}
