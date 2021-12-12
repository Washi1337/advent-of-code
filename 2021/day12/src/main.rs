use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    str::FromStr,
    time::Instant,
};

/// Represents a node in a graph.
pub struct Node {
    /// The unique ID for the node.
    pub id: usize,

    /// A value indicating whether the node represents a large cave or a small cave.
    pub is_large: bool,

    /// A collection of neighbours adjacent to this node.
    pub neighbours: Vec<usize>,
}

/// The special ID for the start node.
pub const NODE_ID_START: usize = 0;

/// The special ID for the end node.
pub const NODE_ID_END: usize = 1;

/// An undirected graph.
pub struct Graph {
    /// The nodes in the graph.
    pub nodes: Vec<Node>,
}

impl Node {
    /// Creates a new cave node.
    pub fn new(id: usize, is_large: bool) -> Self {
        Self {
            id,
            is_large,
            neighbours: Vec::new(),
        }
    }

    /// Gets a value indicating whether the node is the node to start searching from.
    pub fn is_start(&self) -> bool {
        self.id == NODE_ID_START
    }

    /// Gets a value indicating whether the node is the node to search for.
    pub fn is_end(&self) -> bool {
        self.id == NODE_ID_END
    }
}

impl Graph {
    /// Creates a new empty undirected graph.
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }

    /// Adds a node to the graph, and returns the ID of the newly generated node.
    pub fn add_node(&mut self, is_large: bool) -> usize {
        self.nodes.push(Node::new(self.nodes.len(), is_large));
        self.nodes.len() - 1
    }

    /// Connects two nodes together based on their IDs.
    pub fn connect(&mut self, origin_id: usize, target_id: usize) {
        self.nodes[origin_id].neighbours.push(target_id);
        self.nodes[target_id].neighbours.push(origin_id);
    }
}

/// The puzzle input.
pub struct Input {
    /// The graph that was stored in the input file.
    graph: Graph,
}

pub fn parse_input(file: &str) -> std::io::Result<Input> {
    let mut graph = Graph::new();

    // We map names to IDs, this allows for faster lookup later during exploration.
    let mut node_ids = HashMap::new();
    node_ids.insert(String::from_str("start").unwrap(), graph.add_node(false));
    node_ids.insert(String::from_str("end").unwrap(), graph.add_node(false));

    // Parse all lines in the file.
    let file = File::open(file)?;
    BufReader::new(file).lines().for_each(|line| {
        // Split the line into two parts.
        let line = line.expect("Expected a line");
        let mut split = line.split('-');

        // Get the individiual names of the nodes.
        let origin_name = String::from_str(split.next().expect("Expected origin node.")).unwrap();
        let target_name = String::from_str(split.next().expect("Expected target node.")).unwrap();

        // Convert them to IDs, and add them if they weren't added yet.
        let origin_id = get_or_add_node(&mut graph, &mut node_ids, origin_name);
        let target_id = get_or_add_node(&mut graph, &mut node_ids, target_name);

        // Connect the two nodes.
        graph.connect(origin_id, target_id);
    });

    /// Gets the (new) ID of the node with the provided name.
    /// This function will allocate a new node in the graph if the name was not known yet.
    fn get_or_add_node(
        graph: &mut Graph,
        node_ids: &mut HashMap<String, usize>,
        name: String,
    ) -> usize {
        if let Some(&node_id) = node_ids.get(&name) {
            return node_id;
        }

        let is_large = name.chars().nth(0).unwrap().is_uppercase();
        let node_id = graph.add_node(is_large);
        node_ids.insert(name, node_id);
        node_id
    }

    Ok(Input { graph })
}

/// Represents a tree structure that stores all explored paths in a [`Graph`].
struct PathTree {
    /// The path nodes making up the tree.
    nodes: Vec<PathNode>,
}

/// The special ID for the root node within a [`PathTree`].
pub const ROOT_PATH_ID: usize = 0;

/// Represents a single node in a [`PathTree`].
/// To get the full path, treat this node as the head of a linked list.
struct PathNode {
    /// The ID of the node that was explored in the original [`Graph`] instance.
    node_id: usize,

    /// The ID of the path node that this path originated from.
    previous_path_id: usize,
}

impl PathTree {
    /// Creates a new path tree with one root node.
    pub fn new() -> Self {
        Self {
            nodes: vec![PathNode {
                node_id: ROOT_PATH_ID,
                previous_path_id: ROOT_PATH_ID,
            }],
        }
    }

    /// Creates a new path tree with one root node. The path tree will be able to
    /// contain `capacity` elements without reallocating.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut nodes = Vec::with_capacity(capacity);
        nodes.push(PathNode {
            node_id: ROOT_PATH_ID,
            previous_path_id: ROOT_PATH_ID,
        });
        Self { nodes }
    }

    /// Registers a new path in the path tree, originating from the provided path.
    pub fn add_path(&mut self, node_id: usize, previous_path_id: usize) -> usize {
        self.nodes.push(PathNode {
            node_id,
            previous_path_id,
        });
        self.nodes.len() - 1
    }

    /// Determines whether the provided node ID was traversed within the provided explored path.
    pub fn path_contains_node(&self, path_node_id: usize, node_id: usize) -> bool {
        let mut current_id = path_node_id;

        while current_id != ROOT_PATH_ID {
            let current_node = &self.nodes[current_id];

            if current_node.node_id == node_id {
                return true;
            }

            current_id = current_node.previous_path_id;
        }

        false
    }
}

fn find_distinct_paths(graph: &Graph, allow_small_twice: bool) -> usize {
    // Paths counter.
    let mut count = 0;

    // Exploration tree.
    let mut path_tree = PathTree::with_capacity(graph.nodes.len());

    // Allocate agenda and schedule starting node to be processed first..
    let mut agenda = Vec::with_capacity(graph.nodes.len());
    agenda.push((
        NODE_ID_START,
        false,
        path_tree.add_path(NODE_ID_START, ROOT_PATH_ID),
    ));

    while !agenda.is_empty() {
        let (node_id, twice, path_id) = agenda.pop().unwrap();

        // If we found the end, register it and don't explore this path any further.
        if node_id == NODE_ID_END {
            count += 1;
            continue;
        }

        // Explore this new path.
        let new_path_id = path_tree.add_path(node_id, path_id);

        // Look for neighbours.
        for &neighbour_id in graph.nodes[node_id].neighbours.iter() {
            let neighbour_node = &graph.nodes[neighbour_id];

            // Did we traverse this cave already? If we did, we can only do that if the cave is large.
            if !neighbour_node.is_large && path_tree.path_contains_node(new_path_id, neighbour_id) {
                // Part 2: We are actually allowed to traverse a small cave once, but only once!
                if allow_small_twice
                    && !neighbour_node.is_start()
                    && !neighbour_node.is_end()
                    && !twice
                {
                    agenda.push((neighbour_id, true, new_path_id));
                }
            } else {
                agenda.push((neighbour_id, twice, new_path_id));
            }
        }
    }

    count
}

pub fn part1(input: &Input) -> usize {
    find_distinct_paths(&input.graph, false)
}

pub fn part2(input: &Input) -> usize {
    find_distinct_paths(&input.graph, true)
}

fn main() -> std::io::Result<()> {
    let now = Instant::now();
    let input = parse_input("input.txt")?;
    let time_parse = now.elapsed();
    println!("Parse: (time: {}us)", time_parse.as_micros());

    let now = Instant::now();
    let result1 = part1(&input);
    let time1 = now.elapsed();
    println!("Solution 1: {} (time: {}us)", result1, time1.as_micros());

    let now = Instant::now();
    let result2 = part2(&input);
    let time2 = now.elapsed();
    println!("Solution 2: {} (time: {}us)", result2, time2.as_micros());

    Ok(())
}

// Parse: (time: 149us)
// Solution 1: 3576 (time: 1286us)
// Solution 2: 84271 (time: 21737us)
