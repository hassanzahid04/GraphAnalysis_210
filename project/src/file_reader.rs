use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use crate::graph::Graph;

pub fn read_graph_from_file(file_path: &str) -> io::Result<Graph> {
    let file = File::open(file_path)?;
    let reader = io::BufReader::new(file);

    let mut graph = Graph::new();

    for line in reader.lines() {
        let line = line?;
        let users: Vec<&str> = line.split_whitespace().collect();

        if users.len() == 2 {
            graph.add_edge(users[0], users[1]);
        }
    }

    Ok(graph)
}