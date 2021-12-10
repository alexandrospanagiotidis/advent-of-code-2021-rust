use std::collections::{HashMap, VecDeque};
use std::io::{BufRead, stdin};

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let mut valid_pairs = HashMap::new();

    valid_pairs.insert('(', ')');
    valid_pairs.insert('[', ']');
    valid_pairs.insert('{', '}');
    valid_pairs.insert('<', '>');

    let mut illegal_char_count = HashMap::new();

    for line in lines.iter() {
        println!("scanning line={0:?}", line);

        let mut this_line = VecDeque::new();
        this_line.reserve_exact(line.len());

        for token in line.chars() {
            // Opening characters go on the stack
            if valid_pairs.contains_key(&token) {
                println!("found opening {0:?}", token);
                this_line.push_front(token);
            } else {
                // Otherwise we assume its the closing character, so check if it matches the top of stack
                let last_opening = this_line.pop_front()
                    .expect("Could not determine last opening char");

                let expected_closing = valid_pairs.get(&last_opening)
                    .expect(format!("Could not determine expected closing char for last_opening?{0:?}", last_opening).as_str());

                // Legal pairing
                if &token == expected_closing {
                    println!("found closing {0:?} (last_opening={1:?})", token, last_opening);
                    continue;
                }

                println!("{0:?} - Expected {1:?}, but found  {2:?} instead.", line, expected_closing, token);

                *illegal_char_count.entry(token).or_insert(0) += 1;
                break;
            }
        }
    }

    // println!("illegal_char_count={0:?}", illegal_char_count);

    let total_syntax_error_score: i32 = illegal_char_count.iter()
        .map(|(illegal_char, count)| {
            match illegal_char {
                ')' => count * 3,
                ']' => count * 57,
                '}' => count * 1197,
                '>' => count * 25137,
                _ => 0,
            }
        })
        .sum();
    println!("part1: total_syntax_error_score={0:?}", total_syntax_error_score);
}
