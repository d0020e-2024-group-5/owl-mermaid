// use petgraph::{self, prelude::UnGraphMap};

fn main() {
    // let mut g = UnGraphMap::<&str, &str>::new();

    let data = include_str!("OWL.txt");

    for node_data in data.split("\n\n") {
        let mut lines = node_data.lines();
        // add the node to the graph
        let label = lines
            .next()
            .unwrap()
            .split_once(" ")
            .unwrap()
            .0
            .strip_prefix("minecraft:")
            .unwrap();
        // g.add_node(label);

        // add all the edges
        for attribute in lines {
            if let Some(edge) = attribute.strip_prefix("    minecraft:") {
                let (edge_label, nodes) = edge.split_once(" ").unwrap();

                for node in nodes.split(", ") {
                    let node_label = node
                        .strip_prefix("minecraft:")
                        .unwrap()
                        .trim_end_matches(&[' ', ';', '.']);

                    println!("{} -->|{}| {}", label, edge_label, node_label);
                    // g.add_edge(label, node.strip_prefix("minecraft:").unwrap(), edge_label);
                }
            }
        }
    }
}
