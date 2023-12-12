mod graph;
mod bfs;
mod file_reader;

use std::env;
use std::io;

fn main() {
    // Read start and end users from the user at runtime
    let mut start_user = String::new();
    let mut end_user = String::new();

    println!("Enter start user:");
    io::stdin().read_line(&mut start_user).expect("Failed to read input");
    let start_user = start_user.trim();

    println!("Enter end user:");
    io::stdin().read_line(&mut end_user).expect("Failed to read input");
    let end_user = end_user.trim();

    // Read data from text file
    let graph = file_reader::read_graph_from_file("facebook_combined.txt").expect("Failed to read the file");

    // Find minimum distance between two users using BFS
    let distance = bfs::find_shortest_distance(&graph, start_user, end_user);
    match distance {
        Some(d) => println!("Minimum distance between user {} and {} is: {}", start_user, end_user, d),
        None => println!("Users {} and {} are not connected in the social network.", start_user, end_user),
    }
}