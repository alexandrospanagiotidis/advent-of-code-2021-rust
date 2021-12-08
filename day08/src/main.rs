use std::collections::{BTreeMap, BTreeSet};
use std::io::{BufRead, stdin};
use std::str::FromStr;

#[derive(Debug)]
struct SevenSegmentDisplay {
    digits: BTreeMap<i32, BTreeSet<char>>,
    output: Vec<i32>,
}

impl SevenSegmentDisplay {
    fn from_line(line: &String) -> Self {
        let mut input_output = line.split('|');

        let display_digits = input_output.next().expect("Could not determine display digits");
        let mut digits = BTreeMap::new();

        let mut len_5: Vec<BTreeSet<char>> = Vec::new();
        let mut len_6: Vec<BTreeSet<char>> = Vec::new();

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
                    len_5.push(BTreeSet::from_iter(display_digit.chars()));
                }
                6 => {
                    len_6.push(BTreeSet::from_iter(display_digit.chars()));
                }
                7 => {
                    digits.insert(8, BTreeSet::from_iter(display_digit.chars()));
                }
                _ => (),
            }
        }

        // Known: 1, 4, 7, 8
        {
            // 1 is completely contained in 3 but not in 2 or 5
            let one = digits.get(&1).unwrap();
            let three = len_5.iter()
                .enumerate()
                .filter(|&(_index, digit)| Self::contains_all(digit, one))
                .nth(0)
                .expect("Could not determine 3");

            digits.insert(3, len_5.remove(three.0));
        }
        // Known: 1, 3, 4, 7, 8
        {
            // 3 is completely contained in 9 but not in 0 or 6
            let three = digits.get(&3).unwrap();
            let nine = len_6.iter()
                .enumerate()
                .filter(|&(_index, digit)| Self::contains_all(digit, three))
                .nth(0)
                .expect("Could not determine 9");

            digits.insert(9, len_6.remove(nine.0));
        }
        // Known: 1, 3, 4, 7, 8, 9
        {
            // 1 is completely contained in 0 but not in 6
            let one = digits.get(&1).unwrap();
            let zero = len_6.iter()
                .enumerate()
                .filter(|&(_index, digit)| Self::contains_all(digit, one))
                .nth(0)
                .expect("Could not determine 0");

            digits.insert(0, len_6.remove(zero.0));
            digits.insert(6, len_6.pop().expect("Could not determine 6"));
        }
        // Known: 0, 1, 3, 4, 6, 7, 8, 9
        {
            // 5 is completely contained in 6 but not in 2
            let six = digits.get(&6).unwrap();
            let five = len_5.iter()
                .enumerate()
                .filter(|&(_index, digit)| Self::contains_all(six, digit))
                .nth(0)
                .expect("Could not determine 5");

            digits.insert(5, len_5.remove(five.0));
            digits.insert(2, len_5.pop().expect("Could not determine 2"));
        }

        let output_digits = input_output.next().expect("Could not determine output digits");
        let mut output = Vec::new();

        for output_digit in output_digits.split(' ') {
            // println!("output_digit={0:?}", output_digit);

            let output_digit = digits.iter()
                .filter(|&(_digit, segments)| *segments == BTreeSet::from_iter(output_digit.chars()))
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
    fn contains_all(superset: &BTreeSet<char>, subset: &BTreeSet<char>) -> bool {
        subset.iter().all(|element| superset.contains(element))
    }

    fn output_value(&self) -> i32 {
        self.output.iter()
            .map(|&d| std::char::from_digit(d as u32, 10).unwrap())
            .collect::<String>()
            .parse::<i32>()
            .expect("Could not parse output value")
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

    let unique_numbers = vec![1, 4, 7, 8];
    let unique_numbers_count = parsed_lines.iter()
        .flat_map(|parsed_line| parsed_line.output.iter())
        .filter(|&output_digit| unique_numbers.contains(output_digit))
        .count();
    println!("part1: unique_numbers={0:?}", unique_numbers_count);
    assert_eq!(unique_numbers, 392);

    let output_sum: i32 = parsed_lines.iter()
        .map(|parsed_line| parsed_line.output_value())
        .sum();
    println!("part2: output_sum={0:?}", output_sum);
    assert_eq!(output_sum, 1004688);
}
