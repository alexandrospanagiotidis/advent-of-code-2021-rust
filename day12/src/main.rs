use std::collections::{HashMap, HashSet};
use std::io::{BufRead, stdin};

#[derive(Debug)]
struct Graph {
    connections: HashMap<String, HashSet<String>>,
}

fn parse_input(lines: Vec<String>) -> Graph {
    let mut connections = HashMap::new();

    for line in lines {
        let mut tokens = line.split('-');

        let source = tokens.next()
            .expect(format!("Could not read source from: {0:?}", line).as_str());

        let target = tokens.next()
            .expect(format!("Could not read target from: {0:?}", line).as_str());

        connections.entry(source.to_owned())
            .or_insert_with(HashSet::new)
            .insert(target.to_owned());

        connections.entry(target.to_owned())
            .or_insert_with(HashSet::new)
            .insert(source.to_owned());
    }

    Graph {
        connections
    }
}

fn is_small(cave: &str) -> bool {
    cave.to_ascii_lowercase() == *cave
}

fn simple_path_count(graph: &Graph, target: &str, current_node: &str, visits: &mut HashSet<String>) -> usize {
    if current_node == target {
        return 1;
    }

    let mut count = 0;

    if let Some(neighbors) = graph.connections.get(current_node) {
        for neighbor in neighbors {
            if is_small(neighbor) {
                if visits.contains(neighbor) {
                    continue;
                } else {
                    visits.insert(neighbor.to_owned());
                }
            }

            count += simple_path_count(graph, target, neighbor, visits);

            visits.remove(neighbor);
        }
    }

    count
}

fn part1(graph: &Graph) -> usize {
    let mut visits = HashSet::new();
    visits.insert("start".to_owned());

    simple_path_count(graph, "end", "start", &mut visits)
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let graph: Graph = parse_input(lines);
    println!("graph={0:?}", graph);

    let result = part1(&graph);
    println!("result={0:?}", result);
}
