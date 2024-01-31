#![allow(dead_code)]
use anyhow::Result;
use rand::{seq::SliceRandom, thread_rng, Rng};
use std::{
    collections::{hash_set, HashMap, HashSet},
    fmt::Debug,
};

fn edges_from_str(input: &str) -> (&str, Vec<&str>) {
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

fn is_connected(
    edges: &HashMap<usize, Vec<usize>>,
    forbidden_edges: &HashSet<(usize, usize)>,
) -> usize {
    let num_nodes = edges.len();
    let first_node = edges.keys().next().unwrap();
    let mut visited_nodes = HashSet::<&usize>::new();
    let mut queue = vec![first_node];
    while let Some(node) = queue.pop() {
        if visited_nodes.contains(node) {
            continue;
        }
        visited_nodes.insert(node);
        for target in edges.get(node).unwrap() {
            if forbidden_edges.contains(&order_edge(*node, *target)) {
                continue;
            }
            queue.push(target);
        }
    }
    let l = visited_nodes.len();
    l * (num_nodes - l)
}

fn dual_graph(
    graph: &HashMap<usize, Vec<usize>>,
    edges: &Vec<(usize, usize)>,
) -> HashMap<usize, Vec<usize>> {
    let edge_map = edges
        .iter()
        .enumerate()
        .map(|(i, edge)| (*edge, i))
        .collect::<HashMap<_, _>>();

    let mut new_graph = HashMap::<usize, Vec<usize>>::new();
    let mut inserted_edges = HashSet::<(usize, usize)>::new();

    // for edge in edges {
    //     let (node1, node2) = edge;
    //     let edge_id = edge_map.get(&edge).unwrap();
    for ((node1, node2), edge_id) in edge_map.iter() {
        for target in graph.get(&node1).unwrap() {
            let edge_id_other = edge_map.get(&order_edge(*node1, *target)).unwrap();
            if !inserted_edges.insert(order_edge(*edge_id, *edge_id_other)) {
                continue;
            }
            new_graph.entry(*edge_id).or_default().push(*edge_id_other);
            new_graph.entry(*edge_id_other).or_default().push(*edge_id);
        }
        for target in graph.get(&node2).unwrap() {
            let edge_id_other = edge_map.get(&order_edge(*node2, *target)).unwrap();
            if !inserted_edges.insert(order_edge(*edge_id, *edge_id_other)) {
                continue;
            }
            new_graph.entry(*edge_id).or_default().push(*edge_id_other);
            new_graph.entry(*edge_id_other).or_default().push(*edge_id);
        }
    }

    new_graph
}

fn page_rank(graph: &HashMap<usize, Vec<usize>>, damping: f64, n_iter: usize) -> Vec<f64> {
    let num_nodes = graph.len();
    let mut ranks = vec![1.0 / num_nodes as f64; num_nodes];
    let mut new_ranks = vec![0.0; num_nodes];

    for _ in 0..n_iter {
        for (node, targets) in graph {
            let num_targets = targets.len();
            for target in targets {
                new_ranks[*target] += ranks[*node] / num_targets as f64;
            }
        }
        for i in 0..num_nodes {
            new_ranks[i] = (1.0 - damping) / num_nodes as f64 + damping * new_ranks[i];
        }
        std::mem::swap(&mut ranks, &mut new_ranks);
        new_ranks.iter_mut().for_each(|v| *v = 0.0);
    }

    ranks
}

fn choose_random_edge(
    edges: &Vec<(usize, usize)>,
    forbidden_edges: &HashSet<(usize, usize)>,
) -> (usize, usize) {
    let mut rng = thread_rng();
    let mut edge = edges[rng.gen_range(0..edges.len())];
    while forbidden_edges.contains(&edge) {
        edge = edges[rng.gen_range(0..edges.len())];
    }
    edge
}

fn cut_edges_until_disconnected(
    forbidden_edges: HashSet<(usize, usize)>,
    edges: &Vec<(usize, usize)>,
    graph: &HashMap<usize, Vec<usize>>,
) -> HashSet<(usize, usize)> {
    let mut forbidden_edges = forbidden_edges;
    while is_connected(graph, &forbidden_edges) == 0 {
        let edge = choose_random_edge(edges, &forbidden_edges);
        forbidden_edges.insert(edge);
    }
    forbidden_edges
}

/// Remove a random edge from forbidden_edges while keeping the set disconencted
/// If no such edge can be found, return None
fn try_remove_random_edge(
    forbidden_edges: HashSet<(usize, usize)>,
    graph: &HashMap<usize, Vec<usize>>,
) -> Option<HashSet<(usize, usize)>> {
    let mut forbidden_edges = forbidden_edges;
    let mut edges = forbidden_edges.iter().cloned().collect::<Vec<_>>();
    let mut rng = thread_rng();
    edges.shuffle(&mut rng);

    for edge in edges {
        forbidden_edges.remove(&edge);
        if is_connected(graph, &forbidden_edges.clone()) > 0 {
            return Some(forbidden_edges);
        }
        forbidden_edges.insert(edge);
    }
    None
}

/// Remove a random edge
fn force_remove_random_edge(forbidden_edges: &mut HashSet<(usize, usize)>) {
    let mut rng = thread_rng();
    let edges = forbidden_edges.iter().cloned().collect::<Vec<_>>();

    let edge = edges[rng.gen_range(0..edges.len())];
    forbidden_edges.remove(&edge);
}

fn compute_distances(graph: &HashMap<usize, Vec<usize>>, node: usize) -> Vec<usize> {
    let mut distances = vec![usize::MAX; graph.len()];
    let mut queue = vec![(node, 0)];
    while let Some((node, dist)) = queue.pop() {
        if dist >= distances[node] {
            continue;
        }
        distances[node] = dist;
        for target in graph.get(&node).unwrap() {
            queue.push((*target, dist + 1));
        }
    }
    distances
}
fn floyd_warshall(graph: &HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
    let n = graph.len();
    let mut distances = vec![vec![usize::MAX; n]; n];

    for i in 0..n {
        distances[i][i] = 0;
    }

    for (&node, targets) in graph {
        for &target in targets {
            distances[node][target] = 1;
        }
    }

    for k in 0..n {
        println!("Progress: {}/{}", k, n);
        for i in 0..n {
            for j in 0..n {
                if distances[i][k] != usize::MAX && distances[k][j] != usize::MAX {
                    distances[i][j] = distances[i][j].min(distances[i][k] + distances[k][j]);
                }
            }
        }
    }

    distances
}

fn compute_distance_matrix(graph: &HashMap<usize, Vec<usize>>) -> Vec<Vec<usize>> {
    let mut distances = vec![vec![usize::MAX; graph.len()]; graph.len()];
    for (i, node) in graph.keys().enumerate() {
        let node_distances = compute_distances(graph, *node);
        println!("Progress: {}/{}", i, graph.len());
        for (target, dist) in node_distances.iter().enumerate() {
            distances[*node][target] = *dist;
        }
    }
    distances
}

fn main() -> Result<()> {
    let input = std::fs::read_to_string("day25/src/input.txt")?;

    let mut graph_with_str = HashMap::<&str, Vec<&str>>::new();
    for line in input.lines() {
        let (node, targets) = edges_from_str(line);
        for target in targets {
            graph_with_str.entry(node).or_default().push(target);
            graph_with_str.entry(target).or_default().push(node);
        }
    }
    let nodes = graph_with_str.keys().map(|k| *k).collect::<Vec<&str>>();

    let node_names: HashMap<&str, usize> = nodes
        .iter()
        .enumerate()
        .map(|(i, node)| (*node, i))
        .collect();

    let graph: HashMap<usize, Vec<usize>> = graph_with_str
        .iter()
        .map(|(node, targets)| {
            (
                *node_names.get(node).unwrap(),
                targets
                    .iter()
                    .map(|t| *node_names.get(t).unwrap())
                    .collect(),
            )
        })
        .collect();

    let forbidden_edges: HashSet<(usize, usize)> = HashSet::new();
    println!(
        "full graph is connected? {}",
        is_connected(&graph, &forbidden_edges)
    );

    let edges_set: HashSet<(usize, usize)> = graph
        .iter()
        .flat_map(|(node, targets)| {
            targets
                .iter()
                .map(|target| order_edge(*node, *target))
                .collect::<Vec<_>>()
        })
        .collect();
    let edges = edges_set.into_iter().collect::<Vec<_>>();
    println!("edges: {}", edges.len());

    // Print the bins of degrees
    let mut degrees: HashMap<usize, usize> = HashMap::new();
    for (node, targets) in graph.iter() {
        degrees
            .entry(targets.len())
            .and_modify(|v| *v += 1)
            .or_insert(1);
    }
    println!("degrees: {:?}", degrees);

    Ok(())
}
// So we betweennes centraility would be an excellent thing to use, but it's too expensive. 
// We can approximate it by just sampling some random points, and computing shortest
// distances to all other nodes, and just noting down the edges that we see that way.
// Do that for 100 vertices and it might be enough of a centrality measure. 
