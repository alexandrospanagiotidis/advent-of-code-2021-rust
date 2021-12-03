use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() {
    let binary_numbers: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    assert_ne!(binary_numbers.len(), 0);

    let width = binary_numbers[0].len();
    let mut column: usize = 0;

    let mut gamma_rate_bits: Vec<i32> = Vec::new();
    gamma_rate_bits.reserve_exact(width);

    let mut epsilon_rate_bits: Vec<i32> = Vec::new();
    epsilon_rate_bits.reserve_exact(width);

    while column < width {
        let mut counter = HashMap::new();

        for line in &binary_numbers {
            let digit = line.get(column..column + 1).unwrap();
            *counter.entry(digit).or_insert(0) += 1;
        }

        // println!("column={0:?} counter={1:?}", column, counter);

        let zeroes = counter.get("0").unwrap();
        let ones = counter.get("1").unwrap();

        if zeroes > ones {
            gamma_rate_bits.push(0);
            epsilon_rate_bits.push(1);
        } else {
            gamma_rate_bits.push(1);
            epsilon_rate_bits.push(0);
        }

        column += 1;
    }

    // println!("gamma_rate_bits={0:?} epsilon_rate_bits={1:?}", gamma_rate_bits, epsilon_rate_bits);

    let gamma_rate = to_decimal(&mut gamma_rate_bits);
    let epsilon_rate = to_decimal(&mut epsilon_rate_bits);

    println!("gamma_rate={0:?} epsilon_rate={1:?} -> {2:?}", gamma_rate, epsilon_rate, gamma_rate * epsilon_rate);
}

fn to_decimal(bits: &mut Vec<i32>) -> i32 {
    let mut result = 0;

    for (index, bit) in bits.iter().rev().enumerate() {
        let g = if index == 0 && *bit == 0 {
            0
        } else {
            i32::pow(*bit * 2, index as u32)
        };
        println!("index={0:?} bit={1:?} w={2:?}", index, bit, g);
        result += g;
    }

    result
}
