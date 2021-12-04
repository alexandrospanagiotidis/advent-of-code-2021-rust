use std::collections::{VecDeque};
use std::io::{BufRead, stdin};

#[derive(Debug)]
struct BingoBoardEntry {
    value: u32,
    marked: bool,
}

impl BingoBoardEntry {
    fn new(value: u32) -> Self {
        Self {
            value,
            marked: false,
        }
    }
}

#[derive(Debug)]
struct BingoBoard {
    width: usize,
    height: usize,
    entries: Vec<BingoBoardEntry>,
}

impl BingoBoard {
    fn from_lines(lines: &mut VecDeque<String>) -> Self {
        let width = 5;
        let height = 5;
        let mut entries = Vec::new();

        entries.reserve_exact(width * height);

        for row in 0..height {
            let current_line = lines.pop_front()
                .expect(format!("Could not read row {0} for bingo board", row).as_str());

            let mut current_line = current_line.split_whitespace().into_iter();

            for column in 0..width {
                let number = current_line.next()
                    .expect(format!("Could not read ({0}, {1})", row, column).as_str());

                let number = number.parse::<u32>()
                    .expect(format!("Could not parse '{0}' into number at ({1}, {2})", number, row, column).as_str());

                entries.push(BingoBoardEntry::new(number));
            }
        }

        Self {
            width,
            height,
            entries,
        }
    }

    fn mark(&mut self, number: u32) {
        for entry in &mut self.entries {
            if entry.value == number {
                entry.marked = true;
                break;
            }
        }
    }

    fn is_bingo(&self) -> bool {
        'next_column: for column in 0..self.width {
            for row in 0..self.height {
                let index = self.get_index(row, column);
                let entry = self.entries.get(index).unwrap();

                if entry.marked == false {
                    continue 'next_column;
                }
            }

            // All entries in this column have been marked
            return true;
        }

        'next_row: for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row, column);
                let entry = self.entries.get(index).unwrap();

                if entry.marked == false {
                    continue 'next_row;
                }
            }

            // All entries in this row have been marked
            return true;
        }

        false
    }

    fn get_index(&self, row: usize, column: usize) -> usize {
        row * self.width + column
    }

    // fn entries_with<P>(&self, predicate: P) -> Filter<std::slice::Iter<'_, BingoBoardEntry>, P>
    //     where P: FnMut(&BingoBoardEntry) -> bool {
    //     self.entries.iter().filter(predicate)
    // }
}

fn main() {
    let mut lines: VecDeque<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let drawn_numbers = lines.pop_front()
        .expect("Could not read drawn numbers");

    let mut drawn_numbers: VecDeque<u32> = drawn_numbers.split(",")
        .into_iter()
        .map(|token| token.parse::<u32>().unwrap())
        .collect();

    // println!("drawn_numbers={0:?}", drawn_numbers);

    let mut boards: Vec<BingoBoard> = Vec::new();

    while !lines.is_empty() {
        // Skip newline between entries
        lines.pop_front();

        let board = BingoBoard::from_lines(&mut lines);

        // println!("board={0:?}", board);
        boards.push(board);
    }

    // println!("#boards={0:?}", boards.len());

    while !drawn_numbers.is_empty() {
        let drawn_number = drawn_numbers.pop_front()
            .expect("Could not get next drawn number");

        // println!("drawn_number={0:?}", drawn_number);

        for board in &mut boards {
            board.mark(drawn_number);

            if board.is_bingo() {
                let unmarked: Vec<u32> = board.entries.iter()
                    .filter(|entry| entry.marked == false)
                    .map(|entry| entry.value)
                    .collect();

                let sum: u32 = unmarked.iter().sum();

                println!("unmarked={0:?} sum={1:?} * drawn_number={2:?} = {3:?}", unmarked, sum, drawn_number, sum * drawn_number);
                return;
            } else {
                // println!("board={0:?}", board);
            }
        }
    }
}
