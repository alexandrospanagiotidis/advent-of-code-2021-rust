// Very heavily inspired by
// https://github.com/ropewalker/advent_of_code_2021/blob/master/src/day12.rs
// https://github.com/BigPeet/coding_challenges/blob/master/advent_of_code/2021/day12/src/lib.rs

use std::borrow::Borrow;
use std::collections::{HashMap, HashSet, VecDeque};
use std::io::{BufRead, stdin};
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node {
    name: String,
    small: bool,
}

impl Node {
    fn new(s: &str) -> Node {
        Node {
            name: String::from(s),
            small: s.to_lowercase() == s,
        }
    }
}

#[derive(Debug)]
struct Graph {
    connections: HashMap<Rc<Node>, HashSet<Rc<Node>>>,
}

impl Graph {
    fn new() -> Self {
        Graph {
            connections: HashMap::new(),
        }
    }

    fn node(&self, name: &str) -> Option<&Node> {
        self.connections.keys()
            .find(|node| node.name == name)
            .map(|node| node.borrow())
    }

    fn add_edge(&mut self, a: Node, b: Node) {
        let a = Rc::new(a);
        let b = Rc::new(b);

        self.connections.entry(Rc::clone(&a))
            .or_insert_with(HashSet::new)
            .insert(Rc::clone(&b));

        self.connections.entry(Rc::clone(&b))
            .or_insert_with(HashSet::new)
            .insert(Rc::clone(&a));
    }

    fn find_all_paths<'a>(&'a self, source: &'a Node, target: &'a Node, predicate: &impl Fn(&Node, &Path) -> bool) -> Vec<Path> {
        let mut paths = Vec::new();
        let mut boundary = VecDeque::new();

        boundary.push_front((source, Path::new()));

        while let Some((current_node, mut path)) = boundary.pop_front() {
            path.add_node(current_node);

            if current_node == target {
                paths.push(path);
            } else if let Some(neighbors) = self.connections.get(current_node) {
                for valid_neighbor in neighbors.iter().filter(|neighbor| predicate(neighbor, &path)) {
                    boundary.push_back((valid_neighbor, path.clone()));
                }
            }
        }

        paths
    }
}

trait Day12 {
    fn parse_input(lines: Vec<String>) -> Self;
}

impl Day12 for Graph {
    fn parse_input(lines: Vec<String>) -> Self {
        let mut graph = Graph::new();

        for line in lines {
            let mut tokens = line.split('-');

            let source = tokens.next()
                .expect(format!("Could not read source from: {0:?}", line).as_str());

            let target = tokens.next()
                .expect(format!("Could not read target from: {0:?}", line).as_str());

            graph.add_edge(Node::new(source), Node::new(target));
        }

        graph
    }
}

#[derive(Clone, Debug)]
struct Path<'a> {
    path: Vec<&'a Node>,
    visited_twice: Option<&'a Node>,
}

impl<'a> Path<'a> {
    fn new() -> Self {
        Self {
            path: Vec::new(),
            visited_twice: None,
        }
    }

    fn contains(&self, node: &Node) -> bool {
        self.path.contains(&node)
    }

    fn count(&self, node: &Node) -> usize {
        self.path.iter().filter(|&path_node| path_node == &node).count()
    }

    fn add_node(&mut self, node: &'a Node) {
        if self.visited_twice.is_none() && node.small && self.contains(node) {
            self.visited_twice = Some(node);
        }

        self.path.push(node);
    }
}

fn main() {
    let lines: Vec<String> = stdin().lock().lines()
        .map(|line| line.expect("Cannot read line"))
        .collect();

    let graph: Graph = Day12::parse_input(lines);
    // println!("graph={0:?}", graph);

    let start = graph.node("start")
        .expect("Could not find starting node");

    let end = graph.node("end")
        .expect("Could not find starting node");

    let paths = graph.find_all_paths(start, end, &|node: &Node, path: &Path| {
        !(node.small && path.contains(node))
    });
    println!("part1={0:?}", paths.len());
    assert_eq!(paths.len(), 4691);

    let paths = graph.find_all_paths(start, end, &|node: &Node, path: &Path| {
        node != start && !(node.small && path.contains(node) && path.visited_twice.is_some())
    });
    println!("part2={0:?}", paths.len());
    assert_eq!(paths.len(), 140718);
}
