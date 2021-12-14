use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, stdin};

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let mut lines = lines.iter();

    let polymer_template = lines.next()
        .expect("Could not read polymer template");

    println!("polymer_template={0:?}", polymer_template);

    // Skip empty line
    lines.next();

    let mut rules = HashMap::new();

    for line in lines {
        let mut tokens = line.split(" -> ");

        let input = tokens.next()
            .expect(format!("Could not determine input of rule: {0:?}", line).as_str());
        let output = tokens.next()
            .expect(format!("Could not determine output of rule: {0:?}", line).as_str());

        rules.insert(input, output);
    }

    println!("rules={0:?}", rules);

    let result = process_polymer(polymer_template, &mut rules, 10);
    println!("part1: result={0:?}", result);
}

fn process_polymer(polymer_template: &String, rules: &mut HashMap<&str, &str>, steps: i32) -> usize {
    let mut polymer = polymer_template.clone();

    for _i in 0..steps {
        polymer = step(&polymer, &rules);
    }
    // println!("polymer={0:?}", polymer);

    let element_count = count_elements(&polymer);
    // println!("element_count={0:?}", element_count);

    let least_occurring_element = element_count.iter().min_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
        .expect("Could not determine least_occurring_element");
    // println!("least_occurring_element={0:?}", least_occurring_element);

    let most_occurring_element = element_count.iter().max_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
        .expect("Could not determine most_occurring_element");
    // println!("most_occurring_element={0:?}", most_occurring_element);

    most_occurring_element.1 - least_occurring_element.1
}

fn step(polymer: &str, rules: &HashMap<&str, &str>) -> String {
    let mut output = VecDeque::new();

    for i in 0..=polymer.len() - 2 {
        let input = &polymer[i..i + 2];

        if let Some(reaction) = rules.get(input) {
            output.pop_back();
            output.push_back(input[0..1].to_owned());
            output.push_back(reaction.to_string());
            output.push_back(input[1..2].to_owned());
        } else {
            output.push_back(input.to_owned());
        }
    }

    output.into_iter().collect::<String>()
}

fn count_elements(polymer: &str) -> HashMap<char, usize> {
    polymer.chars()
        .fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        })
}
