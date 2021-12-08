use std::collections::{BTreeSet, HashMap};
use std::io::{BufRead, stdin};

#[derive(Debug)]
struct SevenSegmentDisplay {
    digits: HashMap<i32, BTreeSet<char>>,
    output: Vec<i32>,
}

impl SevenSegmentDisplay {
    fn from_line(line: &String) -> Self {
        let mut input_output = line.split('|');

        let display_digits = input_output.next().expect("Could not determine display digits");
        let mut digits = HashMap::new();

        let mut len_5 = BTreeSet::new();
        let mut len_6 = BTreeSet::new();

        for display_digit in display_digits.split(' ') {
            // println!("display_digit={0:?}", display_digit);
            match display_digit.len() {
                2 => {
                    digits.insert(1, BTreeSet::from_iter(display_digit.chars()));
                }
                3 => {
                    digits.insert(7, BTreeSet::from_iter(display_digit.chars()));
                }
                4 => {
                    digits.insert(4, BTreeSet::from_iter(display_digit.chars()));
                }
                5 => {
                    len_5.insert(display_digit);
                }
                6 => {
                    len_6.insert(display_digit);
                }
                7 => {
                    digits.insert(8, BTreeSet::from_iter(display_digit.chars()));
                }
                _ => (),
            }
        }

        let output_digits = input_output.next().expect("Could not determine output digits");
        let mut output = Vec::new();

        for output_digit in output_digits.split(' ') {
            // println!("output_digit={0:?}", output_digit);

            let output_digit = digits.iter()
                .filter(|&(_digit, segments)| *segments== BTreeSet::from_iter(output_digit.chars()))
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
