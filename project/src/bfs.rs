use std::collections::{HashSet, VecDeque};
use crate::graph::Graph;

pub fn find_shortest_distance(graph: &Graph, start_user: &str, end_user: &str) -> Option<usize> {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back((start_user.to_string(), 0));

    while let Some((current_user, distance)) = queue.pop_front() {
        if current_user == end_user {
            return Some(distance);
        }

        if visited.contains(&current_user) {
            continue;
        }

        visited.insert(current_user.clone());

        if let Some(neighbors) = graph.get_neighbors(&current_user) {
            for neighbor in neighbors {
                queue.push_back((neighbor.clone(), distance + 1));
            }
        }
    }

    None
}