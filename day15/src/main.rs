use std::collections::HashMap;
use std::io::{BufRead, stdin};
use std::ops::Range;

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let grid: Grid = Day15::parse_input(&lines);

    let source = NodeIndex(0, 0);
    let target = NodeIndex(grid.width - 1, grid.height - 1);
    // println!("part1: source={0:?} target={1:?}", source, target);

    let path = pathfinding::dijkstra(
        &source,
        |node_index| grid.neighbor_weights(&node_index),
        |node| *node == target,
    ).expect(format!("Could not determine path from {0:?} to {1:?}", source, target).as_str());
    // println!("part1: path={0:?}", path);

    let lowest_total_risk = path.1;
    println!("part1: lowest_total_risk={0:?}", lowest_total_risk);
    assert_eq!(lowest_total_risk, 435);

    let target = NodeIndex(5 * grid.width - 1, 5 * grid.height - 1);
    // println!("part2: source={0:?} target={1:?}", source, target);

    let path = pathfinding::dijkstra(
        &source,
        |node_index| grid.neighbor_weights_wrapped(&node_index, 0..5 * grid.width, 0..5 * grid.height),
        |node| *node == target,
    ).expect(format!("Could not determine path from {0:?} to {1:?}", source, target).as_str());
    // println!("part2: path={0:?}", path);

    let lowest_total_risk = path.1;
    println!("part2: lowest_total_risk={0:?}", lowest_total_risk);
    assert_eq!(lowest_total_risk, 2842);
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
    width: i32,
    height: i32,
}

impl Grid {
    fn new(elements: HashMap<NodeIndex, NodeData>) -> Self {
        let max_index = elements.keys()
            .max_by(|&lhs, &rhs| lhs.cmp(&rhs))
            .expect("Could not determine width")
            .clone();

        Self {
            elements,
            width: max_index.0 + 1,
            height: max_index.1 + 1,
        }
    }

    fn element_at(&self, node_index: &NodeIndex) -> Option<&NodeData> {
        self.elements.get(&node_index)
    }

    fn neighbor_indices(&self, node_index: &NodeIndex) -> Vec<NodeIndex> {
        [
            (0, -1),
            (1, 0),
            (0, 1),
            (-1, 0)
        ].into_iter()
            .map(|(dx, dy)| NodeIndex(node_index.0 + dx, node_index.1 + dy))
            .collect()
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

    fn neighbor_weights_wrapped(&self, node_index: &NodeIndex, horizontal: Range<i32>, vertical: Range<i32>) -> Vec<(NodeIndex, u32)> {
        self.neighbor_indices(node_index).into_iter()
            .filter(|neighbor_index|
                horizontal.contains(&neighbor_index.0)
                    && vertical.contains(&neighbor_index.1)
            )
            .map(|neighbor_index| {
                // print!("node_index={0:?} neighbor_index={1:?}", node_index, neighbor_index);

                let extra_cost_x = neighbor_index.0 / self.width;
                let extra_cost_y = neighbor_index.1 / self.height;

                let base_index = NodeIndex(neighbor_index.0 % self.width, neighbor_index.1 % self.height);
                let base_cost = self.edge_weight(node_index, &base_index)
                    .expect(format!("Could not determine weight for {0:?}", base_index).as_str());

                let cost = (base_cost + extra_cost_x as u32 + extra_cost_y as u32 - 1) % 9 + 1;

                let result = (neighbor_index, cost);

                // println!(" base_index={0:?} base_cost={1:?} result={2:?}", base_index, base_cost, result);

                result
            })
            .collect()
    }
}

impl Day15 for Grid {
    fn parse_input(lines: &Vec<String>) -> Self {
        let mut elements = HashMap::new();
        let mut y = 0;

        for line in lines {
            let values = line.chars();

            for (x, value) in values.enumerate() {
                elements.insert(NodeIndex(x as i32, y), NodeData::from_char(value));
            }

            y += 1;
        }

        Grid::new(elements)
    }
}
