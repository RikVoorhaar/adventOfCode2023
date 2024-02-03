#![allow(dead_code)]
use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{
    collections::{hash_set, HashMap, HashSet},
    fmt::Debug,
};

struct Graph {
    nodes: Vec<Vec<usize>>,
    edges: Vec<(usize, usize)>,
    edge_names: Vec<String>,
    edge_names_inv: HashMap<String, usize>,
}

impl Graph {
    fn _from_str_vecs(input: Vec<(&str, Vec<&str>)>) -> Self {
        let mut edge_names_inv = HashMap::<String, usize>::new();
        let mut edges = Vec::<(usize, usize)>::new();
        let mut edge_names = Vec::<String>::new();
        let mut counter = 0;
        let mut nodes = Vec::<Vec<usize>>::new();

        for (node_src, nodes_tgt) in input {
            for node in (vec![node_src])
                .into_iter()
                .chain(nodes_tgt.iter().cloned())
            {
                if !edge_names_inv.contains_key(node) {
                    edge_names_inv.insert(node.to_string(), counter);
                    edge_names.push(node.to_string());
                    nodes.push(Vec::new());
                    counter += 1;
                }
            }
            for node_tgt in nodes_tgt {
                let src = edge_names_inv.get(node_src).unwrap();
                let tgt = edge_names_inv.get(node_tgt).unwrap();
                nodes[*src].push(*tgt);
                nodes[*tgt].push(*src);
                edges.push(order_edge(*src, *tgt));
            }
        }

        Self {
            nodes,
            edges,
            edge_names,
            edge_names_inv,
        }
    }

    fn from_str(input: &str) -> Self {
        Self::_from_str_vecs(input.lines().map(_edges_from_str).collect::<Vec<_>>())
    }

    fn num_nodes(&self) -> usize {
        self.nodes.len()
    }

    fn num_edges(&self) -> usize {
        self.edges.len()
    }
    fn is_connected(&self, forbidden_edges: &HashSet<(usize, usize)>) -> usize {
        let first_node = 0usize;
        let mut visited_nodes = HashSet::<usize>::new();
        let mut queue = vec![first_node];
        while let Some(node) = queue.pop() {
            if visited_nodes.contains(&node) {
                continue;
            }
            visited_nodes.insert(node);
            for &target in self.nodes.get(node).unwrap() {
                if forbidden_edges.contains(&order_edge(node, target)) {
                    continue;
                }
                queue.push(target);
            }
        }
        let l = visited_nodes.len();
        l * (self.num_nodes() - l)
    }
    fn floyd_warshall(&self) -> Vec<Vec<usize>> {
        let n = self.num_nodes();
        let mut distances = vec![vec![usize::MAX; n]; n];

        for i in 0..n {
            distances[i][i] = 0;
        }

        for (node, targets) in self.nodes.iter().enumerate() {
            for &target in targets {
                distances[node][target] = 1;
            }
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if distances[i][k] != usize::MAX && distances[k][j] != usize::MAX {
                        distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
                    }
                }
            }
            println!("Progress: {}/{}", k + 1, n);
        }

        distances
    }

    /// Gives all the edges involved in shortests paths from the source node
    fn edges_in_shortest_distancs(&self, source: usize) -> Vec<(usize, usize)> {
        let mut visited_nodes = HashSet::<usize>::new();
        let mut shell = HashSet::<usize>::new();
        let mut edges = Vec::<(usize, usize)>::new();
        let mut queue = vec![source];

        while !queue.is_empty() {
            for node in queue {
                for &target in self.nodes.get(node).unwrap() {
                    if visited_nodes.contains(&target) {
                        continue;
                    }
                    shell.insert(target);
                    edges.push(order_edge(node, target));
                }
            }
            visited_nodes.extend(shell.iter());
            queue = shell.into_iter().collect();
            shell = HashSet::new();
        }

        edges
    }
}

fn _edges_from_str(input: &str) -> (&str, Vec<&str>) {
    let mut split = input.split(": ");

    let node = split.next().unwrap();
    let targets = split.next().unwrap().split(' ').collect::<Vec<_>>();
    (node, targets)
}

fn order_edge(a: usize, b: usize) -> (usize, usize) {
    if a < b {
        (a, b)
    } else {
        (b, a)
    }
}

// fn compute_distances(graph: &HashMap<usize, Vec<usize>>, node: usize) -> Vec<usize> {
//     let mut distances = vec![usize::MAX; graph.len()];
//     let mut queue = vec![(node, 0)];
//     while let Some((node, dist)) = queue.pop() {
//         if dist >= distances[node] {
//             continue;
//         }
//         distances[node] = dist;
//         for target in graph.get(&node).unwrap() {
//             queue.push((*target, dist + 1));
//         }
//     }
//     distances
// }

// fn compute_distance_matrix(graph: &HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
//     let mut distances = vec![vec![usize::MAX; graph.len()]; graph.len()];
//     for (i, node) in graph.keys().enumerate() {
//         let node_distances = compute_distances(graph, *node);
//         println!("Progress: {}/{}", i, graph.len());
//         for (target, dist) in node_distances.iter().enumerate() {
//             distances[*node][target] = *dist;
//         }
//     }
//     distances
// }

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day25/src/input.txt")?;
    let graph = Graph::from_str(&input);
    let mut edge_count: HashMap<(usize, usize), usize> = HashMap::new();

    for node in 0..graph.num_nodes() {
        let edges_from_node = graph.edges_in_shortest_distancs(node);
        for edge in edges_from_node {
            *edge_count.entry(edge).or_insert(0) += 1;
        }
        println!("Progress {} / {}", node + 1, graph.num_nodes());
    }
    let mut betweennes: Vec<(usize, usize)> = Vec::new();
    for (index, edge) in graph.edges.iter().enumerate() {
        betweennes.push((index, *edge_count.get(edge).unwrap()));
    }
    betweennes.sort_by(|(_, x), (_, y)| y.cmp(x));
    println!("{:?}", betweennes);
    let mut counter = 0;
    for (i, (edge1, _)) in betweennes.iter().enumerate() {
        for (j, (edge2, _)) in betweennes.iter().enumerate().skip(i) {
            for (edge3, _) in betweennes.iter().skip(j) {
                counter += 1;
                let mut forbidden_edges = HashSet::new();
                forbidden_edges.insert(graph.edges[*edge1]);
                forbidden_edges.insert(graph.edges[*edge2]);
                forbidden_edges.insert(graph.edges[*edge3]);
                let connected = graph.is_connected(&forbidden_edges);
                if connected > 0 {
                    println!("Connected: {} after {} its", connected, counter);
                    return Ok(());
                }
            }
        }
    }

    Ok(())
}
