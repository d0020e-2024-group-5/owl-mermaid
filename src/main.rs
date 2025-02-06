// use petgraph::{self, prelude::UnGraphMap};

struct Node {
    name: String,
    label: String,
    node_type: String,
    file: String,
    // (edgename, nodename)
    edges: Vec<(String, String)>,
}

impl Node {
    fn new_from_str(data: &str, file: &str) -> Self {
        let mut lines = data.lines();
        // get the name and type of node
        let (name, node_type) = lines.next().unwrap().split_once(" a ").unwrap();
        let node_type = node_type.trim_end_matches([';', ' ']);
        // get the name, remove the prefix
        let label = name.split_once(':').unwrap().1;

        let mut edges: Vec<(String, String)> = Vec::new();

        // add pointer to node_type
        // edges.push(("subClassOf".to_string(), node_type.to_string()));

        // add the edges
        for line in lines {
            let trimmed = line.trim_start();
            if let Some(edge) = trimmed.strip_prefix("nodeOntology:pointsToServer") {
                let target_node =
                    format!("{}_{}", file, edge.trim_matches([' ', ',', ';', '.', '\n']));
                edges.push((
                    "inter_server".to_string(),
                    format!(
                        "{}_{}",
                        edge.trim_matches([' ', ',', ';', '.', '\n'])
                            .strip_prefix("minecraft:")
                            .unwrap(),
                        name
                    ),
                ));
                edges.push(("pointsToServer".to_string(), target_node));
            }
            if let Some(edge) = trimmed.strip_prefix("minecraft:") {
                let split = edge.split_once(' ').unwrap();
                let edge_name = split.0;
                for target_node in split.1.trim_matches([' ', ',', ';', '.', '\n']).split(", ") {
                    edges.push((edge_name.to_string(), format!("{}_{}", file, target_node)));
                }
            }
        }

        Node {
            name: format!("{}_{}", file, name),
            label: label.to_string(),
            node_type: node_type.to_string(),
            file: file.to_string(),
            edges: edges,
        }
    }

    fn define_node(&self) -> String {
        if self
            .edges
            .iter()
            .find(|(e, _)| e == "pointsToServer")
            .is_some()
        {
            format!("subgraph {}\n{}[{}]\nend", self.file, self.name, self.label)
        } else if self.node_type == "nodeOntology:Server" {
            format!(
                "subgraph {}\n{}[({})]\nend",
                self.file, self.name, self.label
            )
        } else {
            format!(
                "subgraph {}\n{}([{}])\nend",
                self.file, self.name, self.label
            )
        }
    }

    fn list_edges(&self) -> String {
        let mut s = String::new();
        for edges in &self.edges {
            s.push_str(&self.name);
            s.push_str("-->|");
            s.push_str(&edges.0);
            s.push_str("|");
            s.push_str(&edges.1);
            s.push_str("\n");
        }
        s
    }
}

fn clean_file(in_data: &str) -> String {
    let mut s = String::with_capacity(in_data.len());
    for line in in_data.lines(){
        if let Some(first) = line.chars().next() {
            if matches!(first, '#','"')
        } else {
            s.push_str("\n\n");
        }
    }
}
fn main() {
    // let mut g = UnGraphMap::<&str, &str>::new();

    let data_a = include_str!("../data/server_a.txt");
    let data_b = include_str!("../data/server_b.txt");
    let data_c = include_str!("../data/server_c.txt");

    let mut nodes: Vec<Node> = Vec::new();
    for node_data in data_a.split("\n\n") {
        nodes.push(Node::new_from_str(node_data, "Server_a"));
    }
    for node_data in data_b.split("\n\n") {
        nodes.push(Node::new_from_str(node_data, "Server_b"));
    }
    for node_data in data_c.split("\n\n") {
        nodes.push(Node::new_from_str(node_data, "Server_c"));
    }

    for n in nodes {
        println!("{}", &n.define_node());
        println!("{}", &n.list_edges());
    }
}
