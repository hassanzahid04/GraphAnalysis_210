use crate::graph::Graph;
use crate::bfs::find_shortest_distance;
use crate::file_reader::read_graph_from_file;

#[test]
fn test_bfs() {
    let mut graph = Graph::new();
    graph.add_edge("A", "B");
    graph.add_edge("B", "C");
    graph.add_edge("B", "D");
    graph.add_edge("D", "E");
    

    assert_eq!(find_shortest_distance(&graph, "A", "C"), Some(2));
    assert_eq!(find_shortest_distance(&graph, "A", "E"), Some(3));
    assert_eq!(find_shortest_distance(&graph, "A", "F"), None);
}