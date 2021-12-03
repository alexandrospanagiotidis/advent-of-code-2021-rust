use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() {
    let binary_numbers: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    assert_ne!(binary_numbers.len(), 0);

    let width = binary_numbers[0].len();

    let mut gamma_rate_bits: Vec<u32> = Vec::new();
    gamma_rate_bits.reserve_exact(width);

    let mut epsilon_rate_bits: Vec<u32> = Vec::new();
    epsilon_rate_bits.reserve_exact(width);

    let mut oxygen_generator_rating = binary_numbers.clone();
    let mut co2_scrubber_rating = binary_numbers.clone();

    let mut column: usize = 0;

    while column < width {
        let counter = character_count_by_column(&binary_numbers, column);

        // println!("column={0:?} counter={1:?}", column, counter);

        let zeroes = counter.get("0").unwrap();
        let ones = counter.get("1").unwrap();

        let (gamma_rate_bit, epsilon_rate_bit) = if zeroes > ones { (0, 1) } else { (1, 0) };

        gamma_rate_bits.push(gamma_rate_bit);
        epsilon_rate_bits.push(epsilon_rate_bit);

        if oxygen_generator_rating.len() > 1 {
            let counter = character_count_by_column(&oxygen_generator_rating, column);
            let zeroes = counter.get("0").unwrap_or(&0);
            let ones = counter.get("1").unwrap_or(&0);
            let higher_occurrence = if zeroes > ones { "0" } else { "1" };

            oxygen_generator_rating = filter_by_column(&oxygen_generator_rating, column, higher_occurrence);
        }

        if co2_scrubber_rating.len() > 1 {
            let counter = character_count_by_column(&co2_scrubber_rating, column);
            let zeroes = counter.get("0").unwrap_or(&0);
            let ones = counter.get("1").unwrap_or(&0);
            let lower_occurrence = if zeroes > ones { "1" } else { "0" };

            co2_scrubber_rating = filter_by_column(&co2_scrubber_rating, column, lower_occurrence);
        }

        // println!("oxygen_generator_rating={0:?}", oxygen_generator_rating);
        // println!("co2_scrubber_rating={0:?}", co2_scrubber_rating);

        column += 1;
    }

    // println!("gamma_rate_bits={0:?} epsilon_rate_bits={1:?}", gamma_rate_bits, epsilon_rate_bits);

    let gamma_rate = to_decimal(&gamma_rate_bits);
    println!("gamma_rate={0:?}", gamma_rate);

    let epsilon_rate = to_decimal(&epsilon_rate_bits);
    println!("epsilon_rate={0:?}", epsilon_rate);

    let result = gamma_rate * epsilon_rate;
    println!("part1: result={0:?}", result);
    assert_eq!(result, 2640986);

    let oxygen_generator_rating: Vec<u32> = oxygen_generator_rating.first().map(|s| string_to_digit_vector(&s)).unwrap();
    let oxygen_generator_rating = to_decimal(&oxygen_generator_rating);
    println!("oxygen_generator_rating={0:?}", oxygen_generator_rating);

    let co2_scrubber_rating: Vec<u32> = co2_scrubber_rating.first().map(|s| string_to_digit_vector(&s)).unwrap();
    let co2_scrubber_rating = to_decimal(&co2_scrubber_rating);
    println!("co2_scrubber_rating={0:?}", co2_scrubber_rating);

    let result = oxygen_generator_rating * co2_scrubber_rating;
    println!("part2: result={0:?}", result);
    assert_eq!(result, 6822109);
}

fn string_to_digit_vector(input: &String) -> Vec<u32> {
    input.chars().map(|c| c.to_digit(10).unwrap()).collect()
}

fn character_count_by_column(lines: &Vec<String>, column: usize) -> HashMap<&str, u32> {
    let mut counter = HashMap::new();

    for line in lines {
        let digit = line.get(column..column + 1).unwrap();
        *counter.entry(digit).or_insert(0) += 1;
    }

    counter
}

fn filter_by_column(lines: &Vec<String>, column: usize, filter_by: &str) -> Vec<String> {
    lines.iter()
        .filter(|line| line.get(column..column + 1).unwrap() == filter_by)
        .map(|line| line.to_owned())
        .collect()
}

fn to_decimal(bits: &Vec<u32>) -> u32 {
    let mut result = 0;

    for (index, bit) in bits.iter().rev().enumerate() {
        let value = if index == 0 && *bit == 0 {
            0
        } else {
            u32::pow(*bit * 2, index as u32)
        };
        // println!("index={0:?} bit={1:?} w={2:?}", index, bit, g);
        result += value;
    }

    result
}
