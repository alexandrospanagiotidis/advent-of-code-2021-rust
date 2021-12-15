use std::collections::HashMap;
use std::io::{BufRead, stdin};

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let grid: Grid = Day15::parse_input(&lines);

    let source = NodeIndex(0, 0);
    let target = NodeIndex(grid.width(), grid.height());

    let path = pathfinding::dijkstra(
        &source,
        |node_index| grid.neighbor_weights(&node_index),
        |node| *node == target,
    ).expect(format!("Could not determine path from {0:?} to {1:?}", source, target).as_str());
    // println!("part1: path={0:?}", path);

    let lowest_total_risk = path.1;
    println!("part1: lowest_total_risk={0:?}", lowest_total_risk);
    assert_eq!(lowest_total_risk, 435);
}

trait Day15 {
    fn parse_input(lines: &Vec<String>) -> Self;
}

#[derive(Debug)]
struct NodeData {
    value: u32,
}

impl NodeData {
    fn from_char(value: char) -> Self {
        NodeData {
            value: value.to_digit(10)
                .expect(format!("Could not parse to digit: {0:?}", value).as_str()),
        }
    }
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct NodeIndex(i32, i32);

#[derive(Debug)]
struct Grid {
    elements: HashMap<NodeIndex, NodeData>,
}

impl Grid {
    fn element_at(&self, node_index: &NodeIndex) -> Option<&NodeData> {
        self.elements.get(&node_index)
    }

    fn width(&self) -> i32 {
        self.elements.keys()
            .max_by(|&lhs, &rhs| lhs.0.cmp(&rhs.0))
            .expect("Could not determine width")
            .0
    }

    fn height(&self) -> i32 {
        self.elements.keys()
            .max_by(|&lhs, &rhs| lhs.1.cmp(&rhs.1))
            .expect("Could not determine width")
            .1
    }

    fn neighbor_indices(&self, node_index: &NodeIndex) -> Vec<NodeIndex> {
        let mut neighbor_indices = Vec::new();
        for (dx, dy) in [(0, -1), (1, 0), (0, 1), (-1, 0)] {
            neighbor_indices.push(NodeIndex(node_index.0 + dx, node_index.1 + dy));
        }
        neighbor_indices
    }

    fn edge_weight(&self, _from: &NodeIndex, to: &NodeIndex) -> Option<u32> {
        self.element_at(to).map(|node| node.value)
    }

    fn neighbor_weights(&self, node_index: &NodeIndex) -> Vec<(NodeIndex, u32)> {
        self.neighbor_indices(node_index).into_iter()
            .flat_map(|neighbor_index|
                self.edge_weight(node_index, &neighbor_index).map(|weight| (neighbor_index, weight)))
            .collect()
    }
}

impl Day15 for Grid {
    fn parse_input(lines: &Vec<String>) -> Self {
        let mut nodes = HashMap::new();
        let mut y = 0;

        for line in lines {
            let values = line.chars();

            for (x, value) in values.enumerate() {
                nodes.insert(NodeIndex(x as i32, y), NodeData::from_char(value));
            }

            y += 1;
        }

        Grid {
            elements: nodes,
        }
    }
}
