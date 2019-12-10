use crate::graph::Graph;

pub fn part_one(input: &Vec<(String, String)>) -> i64 {
    let graph = load_graph(input, false);
    graph.checksum()
}

pub fn part_two(input: &Vec<(String, String)>) -> i64 {
    let graph = load_graph(input, true);
    let distance = get_distance("YOU".to_string(), "SAN".to_string(), &graph);
    let distance_minimal_orbits = distance - 2;
    distance_minimal_orbits
}

fn load_graph(input: &Vec<(String, String)>, bi_dir: bool) -> Graph<String> {
    let mut graph = Graph::<String>::new();

    for (a, b) in input {
        let n1 = graph.insert_node(a.clone());
        let n2 = graph.insert_node(b.clone());

        if a == "COM" {
            graph.set_root(&n1);
        }
        graph.insert_edge(n1, n2, bi_dir);
    }

    graph
}

fn get_distance(val1: String, val2: String, graph: &Graph<String>) -> i64 {
    let n1 = graph.get_node(&val1);
    let n2 = graph.get_node(&val2);

    if let Some(root) = n1 {
        if let Some(node) = n2 {
            let dist = graph.get_distance_between(root.get_key(), node.get_key());
            return dist;
        }
    }
    return -1;
}
