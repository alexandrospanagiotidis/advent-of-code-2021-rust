use std::collections::{HashMap};
use std::io::{BufRead, stdin};

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let mut lines = lines.iter();

    let polymer_template = lines.next()
        .expect("Could not read polymer template");
    // println!("polymer_template={0:?}", polymer_template);

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
    // println!("rules={0:?}", rules);

    let result = process_polymer(polymer_template, &mut rules, 10);
    println!("part1: result={0:?}", result);
    assert_eq!(result, 2068);

    let result = process_polymer(polymer_template, &mut rules, 40);
    println!("part2: result={0:?}", result);
    assert_eq!(result, 2158894777814);
}

fn process_polymer(polymer_template: &String, rules: &mut HashMap<&str, &str>, steps: i32) -> usize {
    let mut pair_count: HashMap<String, usize> = HashMap::new();
    let mut element_count: HashMap<String, usize> = HashMap::new();

    // Seed initial production
    for i in 0..=polymer_template.len() - 2 {
        let pair = &polymer_template[i..i + 2];

        *element_count.entry(pair[0..1].to_string()).or_insert(0) += 1;
        *element_count.entry(pair[1..2].to_string()).or_insert(0) += 1;

        let token = pair.to_string();
        *pair_count.entry(token).or_insert(0) += 1;
    }

    for _i in 0..steps {
        // println!("iteration={0:?}", i);
        // println!("pair_count={0:?}", pair_count);
        // println!("element_count={0:?}", element_count);

        let mut this_iteration: HashMap<String, usize> = HashMap::new();

        for (pair, count) in pair_count.iter() {
            let rule_output = rules.get(pair.as_str())
                .expect(format!("Could not find rule: {0:?}", pair).as_str());

            let rule_output = rule_output.to_string();

            let left = [&pair[0..1], &rule_output].join("");
            *this_iteration.entry(left).or_insert(0) += count;

            let right = [&rule_output, &pair[1..2]].join("");
            *this_iteration.entry(right).or_insert(0) += count;

            *element_count.entry(rule_output).or_insert(0) += count;
        }

        pair_count = this_iteration;
    }

    let least_occurring_element = element_count.iter().min_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
        .expect("Could not determine least_occurring_element");
    // println!("least_occurring_element={0:?}", least_occurring_element);

    let most_occurring_element = element_count.iter().max_by(|lhs, rhs| lhs.1.cmp(&rhs.1))
        .expect("Could not determine most_occurring_element");
    // println!("most_occurring_element={0:?}", most_occurring_element);

    most_occurring_element.1 - least_occurring_element.1
}
