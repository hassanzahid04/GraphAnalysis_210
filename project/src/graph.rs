use std::collections::{HashMap, HashSet};

pub struct Graph {
    vertices: HashMap<String, HashSet<String>>,
}

impl Graph {
    pub fn new() -> Self {
        Self {
            vertices: HashMap::new(),
        }
    }

    pub fn add_edge(&mut self, user1: &str, user2: &str) {
        self.vertices
            .entry(user1.to_string())
            .or_insert_with(HashSet::new)
            .insert(user2.to_string());

        self.vertices
            .entry(user2.to_string())
            .or_insert_with(HashSet::new)
            .insert(user1.to_string());
    }

    pub fn get_neighbors(&self, user: &str) -> Option<&HashSet<String>> {
        self.vertices.get(user)
    }
}