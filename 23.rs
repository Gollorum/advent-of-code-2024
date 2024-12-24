extern crate core;

use std::collections::{HashMap, HashSet};
use itertools::Itertools;

const INPUT_ACTUAL: &str = include_str!("inputs/23.txt");
const INPUT_SAMPLE: &str = include_str!("inputs/23_sample.txt");
const INPUT: &str = INPUT_ACTUAL;

fn find_cliques(mut node_pool: HashSet<usize>, current_clique: HashSet<usize>, mut excluded: HashSet<usize>, edges: &Vec<HashSet<usize>>) -> Vec<HashSet<usize>> {
    if node_pool.is_empty() && excluded.is_empty() {
        return vec![current_clique];
    }
    let mut cliques = Vec::new();
    while let Some(&node) = node_pool.iter().next() {
        let new_pool = edges[node].intersection(&node_pool).copied().collect();
        let new_excluded = edges[node].intersection(&excluded).cloned().collect();
        let mut new_clique = current_clique.clone();
        new_clique.insert(node);
        cliques.append(&mut find_cliques(new_pool, new_clique, new_excluded, &edges));
        node_pool.remove(&node);
        excluded.insert(node);
    }
    cliques
}

fn main() {
    let connections = INPUT.lines().map(|line| line.split_once('-').unwrap()).map(|(a, b)| (a.to_string(), b.to_string())).collect::<Vec<(String, String)>>();
    let (mut edge_map_str, nodes) = connections.iter().fold((HashMap::new(), HashSet::new()), |(mut map, mut set), (a, b)| {
        map.entry(a).or_insert(vec![]).push(b);
        map.entry(b).or_insert(vec![]).push(a);
        set.insert(a);
        set.insert(b);
        (map, set)
    });
    let nodes: Vec<&String> = nodes.iter().map(|&n| n).collect();
    let edge_map: Vec<HashSet<usize>> = nodes.iter().map(|node| edge_map_str.get(node).unwrap().iter().map(|neighbor| nodes.iter().position(|n| n == neighbor).unwrap()).collect()).collect();
    let mut triples = 0;
    for node in 0..nodes.len() {
        for &neighbor in edge_map.get(node).unwrap() {
            if node == neighbor {
                continue;
            }
            for &neighbor2 in edge_map.get(neighbor).unwrap() {
                if neighbor2 == node || neighbor2 == neighbor || !(nodes[node].starts_with('t') || nodes[neighbor].starts_with('t') || nodes[neighbor2].starts_with('t')) {
                    continue;
                }
                if let Some(out_edges) = edge_map.get(neighbor2) {
                    if out_edges.contains(&node) {
                        triples += 1;
                    }
                }
            }
        }
    }
    println!("Part 1: {}", triples / 6);

    let node_pool = (0..nodes.len()).collect::<HashSet<usize>>();
    let cliques = find_cliques(node_pool, HashSet::new(), HashSet::new(), &edge_map);
    let max_clique = cliques.iter()
        .map(|c| c.iter().map(|&i| nodes[i]).sorted().collect::<Vec<&String>>())
        .max_by_key(|clique| clique.len()).unwrap();
    println!("Part 2: {:?}", max_clique.iter().join(","));
}